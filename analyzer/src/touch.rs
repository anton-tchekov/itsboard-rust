use cortex_m::delay;
use stm32f4xx_hal::pac::can1::msr::INAK_R;
use stm32f4xx_hal::pac::can1::rx::rdtr;
use stm32f4xx_hal::pac::sdio::clkcr;

use crate::hw::{tp_cs_0, tp_cs_1, get_tp_irq, spi_xchg};
use crate::delay::{delay_ms, delay_us};
use crate::lcd::{lcd_clear, LCD_BLACK, LCD_HEIGHT, LCD_WHITE, LCD_WIDTH};

type Point = u16;

#[derive(Clone)]
pub struct Coordinate
{
	pub x: Point,
	pub y: Point,
}

impl Coordinate
{
	pub fn new(x: Point, y: Point) -> Self
	{
		Self
		{
			x,
			y,
		}
	}
}

pub struct LCDTouch
{
	x_point_raw: Point,
	y_point_raw: Point,
	calibrated_touch: Coordinate,
	x_off: i16,
	y_off: i16,
	fx_fac: f32,
	fy_fac: f32,
	ch_status: u8,
}

const DUMMY_BYTE: u8 = 0xFF;
const D2U_L2R: u8 = 6;
const CURRENT_SCAN_DIR: u8 = D2U_L2R;
const INVALID_POINT: Point = 0xFFFF;
const TP_PRESS_DOWN: u8 = 0x80;
const ERR_RANGE: u16 = 50;
const READ_TIMES: u16 = 5;
const LOST_NUM: u16 = 1;

const CROSS: [u8; 100] = 
[
	//                       Column
	//                     1          2          3
	//            01234567 89012345 67890123 45678901
	//ROW  0      00000000 00001000 00000000 00000000   // (0,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  1      00000000 00001000 00000000 00000000   // (1,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  2      00000000 00001000 00000000 00000000   // (2,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  3      00000000 00001000 00000000 00000000   // (3,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  4      00000000 00001000 00000000 00000000   // (4,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  5      00000000 00001000 00000000 00000000   // (5,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  6      00000000 00111110 00000000 00000000   // (6,10) (6,11) (6,12) (6,13) (6,14)
				  0x00,    0x3E,    0x00,    0x00,
	//ROW  7      00000000 01001001 00000000 00000000   // (7,9) (7,12) (7,15)
				  0x00,    0x49,    0x00,    0x00,
	//ROW  8      00000000 10001000 10000000 00000000   // (8,8) (8,12) (8,16)
				  0x00,    0x88,    0x80,    0x00,
	//ROW  9      00000001 00001000 01000000 00000000   // (9,7) (9,12) (9,17)
				  0x01,    0x08,    0x40,    0x00,
	//ROW  10     00000010 00001000 00100000 00000000   // (10,6) (10,12) (10,18)
				  0x02,    0x08,    0x20,    0x00,
	//ROW  11     00000010 0001 1100 00100000 00000000   // (11,6) (11,11) (11,12) (11,13) (11,18)
				  0x02,    0x1C,    0x20,    0x00,
	//ROW  12     11111111 11111111 11111111 10000000   // (12,0) to (12,24)
				  0xFF,    0x0FF,    0xFF,    0x80,
	//ROW  13     00000010 00011100 00100000 00000000   // (13,6) (13,11) (13,12) (13,13) (13,18)
				  0x02,    0x1C,    0x20,    0x00,
	//ROW  14     00000010 00001000 00100000 00000000   // (14,6) (14,12) (14,18)
				  0x02,    0x08,    0x20,    0x00,
	//ROW  15     00000001 00001000 01000000 00000000   // (15,7) (15,12) (15,17)
				  0x01,    0x08,    0x40,    0x00,
	//ROW  16     00000000 10001000 10000000 00000000   // (16,8) (16,12) (16,16)
				  0x00,    0x88,    0x80,    0x00,
	//ROW  17     00000000 01001001 00000000 00000000   // (17,9) (17,12) (17,15)
				  0x00,    0x49,    0x00,    0x00,
	//ROW  18     00000000 00111110 00000000 00000000   // (18,10) (18,11) (18,12) (18,13) (18,14)
				  0x00,    0x3E,    0x00,    0x00,
	//ROW  19     00000000 00001000 00000000 00000000   // (19,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  20     00000000 00001000 00000000 00000000   // (20,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  21     00000000 00001000 00000000 00000000   // (21,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  22     00000000 00001000 00000000 00000000   // (22,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  23     00000000 00001000 00000000 00000000   // (23,12)
				  0x00,    0x08,    0x00,    0x00,
	//ROW  24     00000000 00001000 00000000 00000000   // (24,12)
				  0x00,    0x08,    0x00,    0x00
];

impl LCDTouch
{
	pub fn new() -> Self
	{
		let mut x = Self
		{
			x_point_raw: 0,
			y_point_raw: 0,
			calibrated_touch: Coordinate::new(INVALID_POINT, INVALID_POINT),
			x_off: 0,
			y_off: 0,
			fx_fac: 0.0,
			fy_fac: 0.0,
			ch_status: 0,
		};

		x.set_default_cal();

		return x;
	}

	pub fn is_pressed(&mut self) -> bool
	{
		self.scan()
	}

	pub fn get_touch_coords(&self) -> Coordinate
	{
		return self.calibrated_touch.clone();
	}

	fn read_adc(&self, cmd: u8) -> u16
	{
		let mut data: u16;

		// Not implemented
		// SPIsetTPSpeed();	
		tp_cs_0();
		spi_xchg(0);
		spi_xchg(cmd);
		delay_us(200);

		data = spi_xchg(DUMMY_BYTE) as u16;
		data <<= 8;
		data |= spi_xchg(DUMMY_BYTE) as u16;

		// Not implmeneted, its just for debug
		// UPDATE_TP_CMD(CMD, Data); 
		data >>= 3;
		tp_cs_1();

		// Not implemented
		// SPIsetLCDSpeed();

		return data;
	}

	fn read_adc_avg(&self, cmd: u8) -> u16
	{
		let mut read_buf: [u16; READ_TIMES as usize] = [0; READ_TIMES as usize];
		let mut read_sum: u16 = 0;
		let mut read_temp: u16 = 0;

		//Read and save multiple samples
		for i in 0..READ_TIMES as usize
		{
			read_buf[i] = self.read_adc(cmd);
			delay_us(200);
		}
		
		//Sort from small to large
		for i in 0..(READ_TIMES-1) as usize {
			for j in (i+1)..READ_TIMES as usize
			{
				if read_buf[i] > read_buf[j] 
				{
					read_temp = read_buf[i];
					read_buf[i] = read_buf[j];
					read_buf[j] = read_temp;
				}
			}
		}

		//Exclude the largest and the smallest
		for i in (LOST_NUM as usize)..(READ_TIMES-LOST_NUM) as usize
		{
			read_sum += read_buf[i];
		}

		//Averaging
		read_temp = read_sum / (READ_TIMES - 2 * LOST_NUM);

		return read_temp;
	}

	fn read_adc_xy(&self) -> (u16, u16)
	{
		let mut result: (u16, u16) = (0, 0);

		result.0 = self.read_adc_avg(0xD0);
		result.1 = self.read_adc_avg(0x90);

		return result;
	}

	fn read_twiceadc(&self) -> (bool, u16, u16)
	{
		let xch_adc1: u16;
		let ych_adc1: u16;
		let xch_adc2: u16;
		let ych_adc2: u16;

		let xch_adc: u16;
		let ych_adc: u16;

		// Not Implemented but might be neccessary
		// SPIsetTPSpeed();

		(xch_adc1, ych_adc1) = self.read_adc_xy();
		delay_us(10);
		(xch_adc2, ych_adc2) = self.read_adc_xy();

		// Not Implemented but might be neccessary if speed is changed before this
		// SPIsetLCDSpeed();

		xch_adc = (xch_adc1 + xch_adc2) / 2;
		ych_adc = (ych_adc1 + ych_adc2) / 2;

		let validity =  
		((xch_adc2 <= xch_adc1 && xch_adc1 < xch_adc2 + ERR_RANGE) ||
        (xch_adc1 <= xch_adc2 && xch_adc2 < xch_adc1 + ERR_RANGE)) &&
        ((ych_adc2 <= ych_adc1 && ych_adc1 < ych_adc2 + ERR_RANGE) ||
        (ych_adc1 <= ych_adc2 && ych_adc2 < ych_adc1 + ERR_RANGE));

		return (validity, xch_adc, ych_adc);
	}

	fn set_default_cal(&mut self)
	{
		self.fx_fac = -0.132443;
		self.fy_fac = 0.089997;
		self.x_off = 516;
		self.y_off = -22;
	}

	fn scan(&mut self) -> bool
	{
		const MAX_COL: u16 = LCD_WIDTH as u16 - 1;
		const MAX_ROW: u16 = LCD_HEIGHT as u16 - 1;

		if get_tp_irq()
		{
			// Touch not pressed
			self.ch_status &= !TP_PRESS_DOWN;
			return false;
		}
		// Touch pressed
		self.ch_status |= TP_PRESS_DOWN;

		let rd_twice = self.read_twiceadc();
		self.x_point_raw = rd_twice.1;
		self.y_point_raw = rd_twice.2;

		if rd_twice.0
		{
			// Convert Results to coordinates
			self.calibrated_touch.x = (MAX_COL as f32 - (self.fx_fac * self.y_point_raw as f32) - self.x_off as f32) as u16;
			self.calibrated_touch.y = (MAX_ROW as f32 - (self.fy_fac * self.x_point_raw as f32) - self.y_off as f32) as u16;
		}
		else
		{
			return false;
		}

		return true;
	}
}