use crate::delay::*;
use crate::hw::*;

const CMD_GO_IDLE_STATE      : u8 = 0x00;
const CMD_SEND_OP_COND       : u8 = 0x01;
const CMD_SEND_IF_COND       : u8 = 0x08;
const CMD_SEND_CSD           : u8 = 0x09;
const CMD_SEND_CID           : u8 = 0x0A;
const CMD_SET_BLOCKLEN       : u8 = 0x10;
const CMD_READ_SINGLE_BLOCK  : u8 = 0x11;
const CMD_WRITE_SINGLE_BLOCK : u8 = 0x18;
const CMD_SD_SEND_OP_COND    : u8 = 0x29;
const CMD_APP                : u8 = 0x37;
const CMD_READ_OCR           : u8 = 0x3A;

const IDLE_STATE             : u8 = 1 << 0;
const ILLEGAL_CMD            : u8 = 1 << 2;

const SD_1                   : u8 = 1 << 0;
const SD_2                   : u8 = 1 << 1;
pub const SD_HC              : u8 = 1 << 2;

pub struct Sd {
	pub serial: u32,
	pub capacity: u32,
	pub oem: [u8; 2],
	pub product_name: [u8; 5],
	pub manufacturer: u8,
	pub revision: u8,
	pub manufacturing_year: u8,
	pub manufacturing_month: u8,
	pub card_type: u8
}

impl Sd {
	pub fn init() -> Result<Sd, ()> {
		let mut sd = Sd {
			serial: 0,
			capacity: 0,
			oem: [0; 2],
			product_name: [0; 5],
			manufacturer: 0,
			revision: 0,
			manufacturing_year: 0,
			manufacturing_month: 0,
			card_type: 0
		};

		sd_cs_1();
		spi_slow();
		sd.card_type = 0;

		{
			let mut i = 0;
			while i < 10 {
				spi_xchg(0xFF);
				i += 1;
			}
		}

		sd_cs_0();
		{
			let mut i = 0;
			loop {
				if Self::command(CMD_GO_IDLE_STATE, 0) == IDLE_STATE {
					break;
				}

				if i == 0x1FF {
					sd_cs_1();
					return Err(());
				}

				i += 1;
			}
		}

		if (Self::command(CMD_SEND_IF_COND, 0x1AA) & ILLEGAL_CMD) == 0 {
			spi_xchg(0xFF);
			spi_xchg(0xFF);
			if ((spi_xchg(0xFF) & 0x01) == 0) || (spi_xchg(0xFF) != 0xAA) {
				return Err(());
			}

			sd.card_type |= SD_2;
		}
		else {
			Self::command(CMD_APP, 0);
			if (Self::command(CMD_SD_SEND_OP_COND, 0) & ILLEGAL_CMD) == 0 {
				sd.card_type |= SD_1;
			}
		}

		{
			let mut i = 0;
			loop {
				let response =
					if sd.card_type & (SD_1 | SD_2) != 0 {
						Self::command(CMD_APP, 0);
						Self::command(CMD_SD_SEND_OP_COND,
							if sd.card_type & SD_2 != 0 { 0x40000000 } else { 0 })
					}
					else {
						Self::command(CMD_SEND_OP_COND, 0)
					};

				if (response & IDLE_STATE) == 0 {
					break;
				}

				if i == 0x7FFF {
					sd_cs_1();
					return Err(());
				}

				i += 1;
			}
		}

		if sd.card_type & SD_2 != 0 {
			if Self::command(CMD_READ_OCR, 0) != 0 {
				sd_cs_1();
				return Err(());
			}

			if spi_xchg(0xFF) & 0x40 != 0 {
				sd.card_type |= SD_HC;
			}

			spi_xchg(0xFF);
			spi_xchg(0xFF);
			spi_xchg(0xFF);
		}

		if Self::command(CMD_SET_BLOCKLEN, 512) != 0 {
			sd_cs_1();
			return Err(());
		}

		sd_cs_1();
		spi_fast();
		delay_ms(20);

		sd_cs_0();
		let mut buf: [u8; 18] = [0; 18];

		Self::read_info(CMD_SEND_CID, &mut buf)?;

		sd.manufacturer = buf[0];
		sd.oem[0] = buf[1];
		sd.oem[1] = buf[2];
		sd.product_name[0] = buf[3];
		sd.product_name[1] = buf[4];
		sd.product_name[2] = buf[5];
		sd.product_name[3] = buf[6];
		sd.product_name[4] = buf[7];
		sd.revision = buf[8];
		sd.serial = ((buf[9] as u32) << 24) |
			((buf[10] as u32) << 16) |
			((buf[11] as u32) << 8) |
			(buf[12] as u32);
		sd.manufacturing_year = (buf[13] << 4) | (buf[14] >> 4);
		sd.manufacturing_month = buf[14] & 0x0F;

		Self::read_info(CMD_SEND_CSD, &mut buf)?;
		let csd_structure = buf[0] >> 6;
		sd.capacity = if csd_structure == 0x01 {
			((((((((buf[7] as u32) & 0x3F) + 1) << 8 |
				(buf[8] as u32)) + 1) << 8) |
				(buf[9] as u32)) + 1) << 10
		}
		else if csd_structure == 0x00 {
			let csd_read_bl_len: u32 = (buf[5] as u32) & 0x0F;
			let csd_c_size = ((((((buf[6] as u32) & 0x03) << 8) |
				(buf[7] as u32)) << 2) | ((buf[8] as u32) >> 6)) + 1;
			let csd_c_size_mult = (((buf[9] as u32) & 0x03) << 1) | ((buf[10] as u32) >> 7);
			(csd_c_size << (csd_c_size_mult + csd_read_bl_len + 2)) >> 9
		} else {
			0
		};

		sd_cs_1();
		Ok(sd)
	}

	fn read_info(reg: u8, buf: &mut [u8]) -> Result<(), ()> {
		if Self::command(reg, 0) != 0 {
			sd_cs_1();
			return Err(());
		}

		let mut i = 0;
		while spi_xchg(0xFF) != 0xFE {
			i += 1;
			if i > 0x7FFF {
				sd_cs_1();
				return Err(());
			}
		}

		i = 0;
		while i < 18 {
			buf[i] = spi_xchg(0xFF);
			i += 1;
		}

		Ok(())
	}

	fn command(cmd: u8, arg: u32) -> u8 {
		spi_xchg(0xFF);
		spi_xchg(0x40 | cmd);
		spi_xchg(((arg >> 24) & 0xFF) as u8);
		spi_xchg(((arg >> 16) & 0xFF) as u8);
		spi_xchg(((arg >> 8) & 0xFF) as u8);
		spi_xchg((arg & 0xFF) as u8);

		if cmd == CMD_GO_IDLE_STATE {
			spi_xchg(0x95);
		}
		else if cmd == CMD_SEND_IF_COND {
			spi_xchg(0x87);
		}
		else {
			spi_xchg(0xFF);
		}

		let mut response = 0;
		let mut i = 0;
		while i < 10 {
			response = spi_xchg(0xFF);
			if response != 0xFF {
				break;
			}

			i += 1;
		}

		response
	}

	fn block_addr(card_type: u8, block: u32) -> u32 {
		if card_type & SD_HC != 0 { block } else { block >> 9 }
	}

	pub fn read(&self, block: u32, buf: &mut [u8]) -> Result<(), ()> {
		sd_cs_0();
		if Self::command(CMD_READ_SINGLE_BLOCK,
			Self::block_addr(self.card_type, block)) != 0 {
			sd_cs_1();
			return Err(());
		}

		{
			let mut i = 0;
			loop {
				if spi_xchg(0xFF) == 0xFE {
					break;
				}

				if i == 0xFFFF {
					sd_cs_1();
					return Err(());
				}

				i += 1;
			}
		}

		{
			let mut i = 0;
			while i < 512 {
				buf[i] = spi_xchg(0xFF);
				i += 1;
			}
		}

		spi_xchg(0xFF);
		spi_xchg(0xFF);
		sd_cs_1();
		spi_xchg(0xFF);
		Ok(())
	}

	pub fn write(&self, block: u32, buf: &[u8]) -> Result<(), ()> {
		sd_cs_0();
		if Self::command(CMD_WRITE_SINGLE_BLOCK,
			Self::block_addr(self.card_type, block)) != 0 {
			sd_cs_1();
			return Err(());
		}

		spi_xchg(0xFE);

		{
			let mut i = 0;
			while i < 512 {
				spi_xchg(buf[i]);
				i += 1;
			}
		}

		spi_xchg(0xFF);
		spi_xchg(0xFF);

		{
			let mut i = 0;
			loop {
				if spi_xchg(0xFF) == 0xFF {
					break;
				}

				if i == 0xFFFF {
					sd_cs_1();
					return Err(());
				}

				i += 1;
			}
		}

		spi_xchg(0xFF);
		sd_cs_1();
		Ok(())
	}
}
