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
const SD_HC                  : u8 = 1 << 2;

struct Sd {
	card_type: u8
}

impl Sd {
	pub fn init() -> Result<Sd, ()> {
		let mut sd = Sd { card_type: 0 };

		sd_deselect();
		spi_slow();
		sd.card_type = 0;

		{
			let mut i = 0;
			while i < 10 {
				spi_xchg(0xFF);
				i += 1;
			}
		}

		sd_select();
		{
			let mut i = 0;
			loop {
				if Self::command(CMD_GO_IDLE_STATE, 0) == IDLE_STATE {
					break;
				}

				if i == 0x1FF {
					sd_deselect();
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
					sd_deselect();
					return Err(());
				}

				i += 1;
			}
		}

		if sd.card_type & SD_2 != 0 {
			if Self::command(CMD_READ_OCR, 0) != 0 {
				sd_deselect();
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
			sd_deselect();
			return Err(());
		}

		sd_deselect();
		spi_fast();
		delay_ms(20);
		return Ok(sd);
	}

	fn command(cmd: u8, arg: u32) -> u8 {
		spi_xchg(0xFF);
		spi_xchg(0x40 | cmd);
		spi_xchg(((arg >> 24) & 0xFF) as u8);
		spi_xchg(((arg >> 16) & 0xFF) as u8);
		spi_xchg(((arg >> 8) & 0xFF) as u8);
		spi_xchg(((arg >> 0) & 0xFF) as u8);

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

		return response;
	}

	fn block_addr(card_type: u8, block: u32) -> u32 {
		if card_type & SD_HC != 0 { block } else { block >> 9 }
	}

	pub fn read(&self, block: u32, buf: &mut [u8]) -> Result<(), ()> {
		sd_select();
		if Self::command(CMD_READ_SINGLE_BLOCK,
			Self::block_addr(self.card_type, block)) != 0 {
			sd_deselect();
			return Err(());
		}

		{
			let mut i = 0;
			loop {
				if spi_xchg(0xFF) == 0xFE {
					break;
				}

				if i == 0xFFFF {
					sd_deselect();
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
		sd_deselect();
		spi_xchg(0xFF);
		return Ok(());
	}

	pub fn write(&self, block: u32, buf: &[u8]) -> Result<(), ()> {
		sd_select();
		if Self::command(CMD_WRITE_SINGLE_BLOCK,
			Self::block_addr(self.card_type, block)) != 0 {
			sd_deselect();
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
					sd_deselect();
					return Err(());
				}

				i += 1;
			}
		}

		spi_xchg(0xFF);
		sd_deselect();
		return Ok(());
	}
}
