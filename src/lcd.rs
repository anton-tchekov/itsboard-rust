use crate::delay::*;
use crate::hw::*;
use stm32f4xx_hal::pac::SPI1;
use stm32f4xx_hal::spi::Spi;

pub const LCD_WIDTH: u32 = 480;
pub const LCD_HEIGHT: u32 = 320;

const LCD_INIT_CMDS: [u8; 103] =
[
	0xF9,
	2,
	0x00,
	0x08,
	0xC0,
	2,
	0x19,
	0x1a,
	0xC1,
	2,
	0x45,
	0x00,
	0xC2,
	1,
	0x33,
	0xC5,
	2,
	0x00,
	0x28,
	0xB1,
	2,
	0xA0,
	0x11,
	0xB4,
	1,
	0x02,
	0xB6,
	3,
	0x00,
	0x42,
	0x3B,
	0xB7,
	1,
	0x07,
	0xE0,
	15,
	0x1F,
	0x25,
	0x22,
	0x0B,
	0x06,
	0x0A,
	0x4E,
	0xC6,
	0x39,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xE1,
	15,
	0x1F,
	0x3F,
	0x3F,
	0x0F,
	0x1F,
	0x0F,
	0x46,
	0x49,
	0x31,
	0x05,
	0x09,
	0x03,
	0x1C,
	0x1A,
	0x00,
	0xF1,
	8,
	0x36,
	0x04,
	0x00,
	0x3C,
	0x0F,
	0x0F,
	0xA4,
	0x02,
	0xF2,
	9,
	0x18,
	0xA3,
	0x12,
	0x02,
	0x32,
	0x12,
	0xFF,
	0x32,
	0x00,
	0xF4,
	5,
	0x40,
	0x00,
	0x08,
	0x91,
	0x04,
	0xF8,
	2,
	0x21,
	0x04,
	0x3A,
	1,
	0x55
];

fn lcd_param0(spi: &mut Spi<SPI1>)
{
	lcd_dc_1();
	lcd_cs_0();
	spi_ll_xchg(spi, 0);
}

fn lcd_param1(spi: &mut Spi<SPI1>, param: u8)
{
	spi_ll_xchg(spi, param);
	lcd_cs_1();
}

fn lcd_param(spi: &mut Spi<SPI1>, param: u8)
{
	lcd_param0(spi);
	lcd_param1(spi, param);
}

fn lcd_cmd(spi: &mut Spi<SPI1>, cmd: u8)
{
	lcd_dc_0();
	lcd_cs_0();
	spi_ll_xchg(spi, cmd);
	lcd_cs_1();
}

pub fn lcd_emit(spi: &mut Spi<SPI1>, color: u16)
{
	spi_ll_xchg(spi, (color >> 8) as u8);
	spi_ll_xchg(spi, (color & 0xFF) as u8);
}

fn lcd_reset()
{
	lcd_rst_1();
	delay_ms(500);
	lcd_rst_0();
	delay_ms(500);
	lcd_rst_1();
	delay_ms(500);
}

pub fn lcd_window_start(spi: &mut Spi<SPI1>, x: u32, y: u32, w: u32, h: u32)
{
	let ex = x + w - 1;
	let ey = y + h - 1;

	lcd_cmd(spi, 0x2A);
	lcd_param(spi, (x >> 8) as u8);
	lcd_param(spi, (x & 0xFF) as u8);
	lcd_param(spi, (ex >> 8) as u8);
	lcd_param(spi, (ex & 0xFF) as u8);

	lcd_cmd(spi, 0x2B);
	lcd_param(spi, (y >> 8) as u8);
	lcd_param(spi, (y & 0xFF) as u8);
	lcd_param(spi, (ey >> 8) as u8);
	lcd_param(spi, (ey & 0xFF) as u8);

	lcd_cmd(spi, 0x2C);
	lcd_dc_1();
	lcd_cs_0();
}

pub fn lcd_window_end()
{
	lcd_cs_1();
}

pub fn lcd_rect(spi: &mut Spi<SPI1>, x: u32, y: u32, w: u32, h: u32, color: u16)
{
	let mut count = w * h;
	let color_hi = (color >> 8) as u8;
	let color_lo = (color & 0xFF) as u8;

	lcd_window_start(spi, x, y, w, h);
	while count > 0
	{
		count -= 1;
		spi_ll_xchg(spi, color_hi);
		spi_ll_xchg(spi, color_lo);
	}

	lcd_window_end();
}

pub fn lcd_callback(spi: &mut Spi<SPI1>, x: u32, y: u32, w: u32, h: u32,
	callback: &dyn Fn(u32, u32) -> u16) {
	lcd_window_start(spi, x, y, w, h);
	let mut y0 = 0;
	while y0 < h {
		let mut x0 = 0;
		while x0 < w {
			lcd_emit(spi, callback(x0, y0));
			x0 += 1;
		}

		y0 += 1;
	}

	lcd_window_end();
}

pub fn lcd_init(spi: &mut Spi<SPI1>, color: u16)
{
	lcd_reset();

	let mut i = 0;
	while i < LCD_INIT_CMDS.len() {
		lcd_cmd(spi, LCD_INIT_CMDS[i]);
		i += 1;
		let mut num = LCD_INIT_CMDS[i];
		i += 1;
		while num > 0
		{
			num -= 1;
			lcd_param(spi, LCD_INIT_CMDS[i]);
			i += 1;
		}
	}

	lcd_cmd(spi, 0xB6);
	lcd_param(spi, 0x00);
	lcd_param(spi, 0x62);
	lcd_cmd(spi, 0x36);
	lcd_param(spi, 0x28);

	delay_ms(200);
	lcd_cmd(spi, 0x11);
	delay_ms(120);
	lcd_cmd(spi, 0x29);
	lcd_rect(spi, 0, 0, LCD_WIDTH, LCD_HEIGHT, color);
}

pub const fn lcd_color(r: u8, g: u8, b: u8) -> u16
{
	((r as u16 & 0xF8) << 8) |
		((g as u16 & 0xFC) << 3) |
		(b as u16 >> 3)
}
