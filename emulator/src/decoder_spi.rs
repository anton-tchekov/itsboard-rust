use crate::decoder::*;
use crate::sample::*;

#[derive(PartialEq)]
pub enum BitOrder
{
	MsbFirst,
	LsbFirst
}

pub struct DecoderSPI
{
	pub mosi_pin: DecoderPin,
	pub miso_pin: DecoderPin,
	pub sck_pin: DecoderPin,
	pub cs_pin: DecoderPin,
	pub mode: u8,
	pub bitorder: BitOrder
}

fn extract(pins: Sample, pin: i32) -> u8
{
	if pin > 0
	{
		(pins >> pin) & 1
	}
	else
	{
		0
	}
}

impl Decoder for DecoderSPI
{
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()>
	{
		let word_size = 8;
		let mut mosidata = 0;
		let mut misodata = 0;
		let mut oldpins = 0;
		let mut bitcount = 0;
		let mut start_sample = 0;
		let mut sck = 0;
		let mut miso = 0;
		let mut mosi = 0;
		let mut cs = 0;
		let mut oldsck = 0;
		let mut first = true;

		for (ts, pins) in samples
		{
			sck  =  extract(pins, self.sck_pin);
			mosi =  extract(pins, self.mosi_pin);
			miso =  extract(pins, self.miso_pin);
			cs   =  extract(pins, self.cs_pin);

			if first
			{
				oldsck = sck;
				first = false;
			}

			if oldpins == pins { continue; }

			if sck == oldsck { continue; }
			oldsck = sck;

			if self.mode == 0 && sck == 0 { continue; }
			else if self.mode == 1 && sck == 1 { continue; }
			else if self.mode == 2 && sck == 1 { continue; }
			else if self.mode == 3 && sck == 0 { continue; }

			if bitcount == 0
			{
				start_sample = ts;
			}

			if self.bitorder == BitOrder::MsbFirst
			{
				mosidata |= mosi << (word_size - 1 - bitcount);
				misodata |= miso << (word_size - 1 - bitcount);
			}
			else
			{
				mosidata |= mosi << bitcount;
				misodata |= miso << bitcount;
			}

			bitcount += 1;
			if bitcount != word_size { continue; }

			output.push(Section { start: start_sample, end: ts, content: SectionContent::RxByte(misodata) });
			output.push(Section { start: start_sample, end: ts, content: SectionContent::TxByte(mosidata) });

			mosidata = 0;
			misodata = 0;
			bitcount = 0;
		}

		println!("Decoder SPI done!");

		Ok(())
	}

	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>
	{
		match idx
		{
			0 => Some(("MOSI", self.mosi_pin)),
			1 => Some(("MISO", self.miso_pin)),
			2 => Some(("SCK", self.sck_pin)),
			3 => Some(("CS", self.cs_pin)),
			_ => None,
		}
	}
}
