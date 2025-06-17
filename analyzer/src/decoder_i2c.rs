use crate::decoder::*;
use crate::sample::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DecoderI2C
{
	pub sda_pin: DecoderPin,
	pub scl_pin: DecoderPin
}

#[derive(Debug, PartialEq)]
enum Edge
{
	Stable,
	Rising,
	Falling,
	Error,
}

impl Edge
{
	pub fn get_edge(last: bool, cur: bool) -> Edge
	{
		if last == cur
		{
			Edge::Stable
		}
		else if last && !cur
		{
			Edge::Falling
		}
		else if !last && cur
		{
			Edge::Rising
		}
		else
		{
			Edge::Error
		}
	}
}

use Edge::*;
impl Decoder for DecoderI2C
{
	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		let samples_len = samples.len;

		let mut in_transmission = false;
		let mut address_seen = false;
		let mut last_scl = true;
		let mut last_sda = true;
		let mut last_ts = 0;

		let mut start_found = false;

		let mut cur_byte: u8 = 0;
		let mut cur_byte_index: u8 = 0;
		let mut cur_byte_start = 0;

		for i in 0..samples_len
		{
			let scl: bool = samples.samples[i] & (1 << self.get_pin(1).unwrap().1 as u8) > 0;
			let sda: bool = samples.samples[i] & (1 << self.get_pin(0).unwrap().1 as u8) > 0;
			let ts = samples.timestamps[i];

			let sda_edge: Edge =  Edge::get_edge(last_sda, sda);
			let scl_edge: Edge =  Edge::get_edge(last_scl, scl);

			if !in_transmission
			{
				/* Start Condition */
				if sda_edge == Falling && scl
				{
					start_found = true;
				}
				else if scl_edge == Falling && start_found
				{
					start_found = false;
					in_transmission = true;
					output.push(Section{start: last_ts, end: ts, content: SectionContent::StartBit});
				}
			}
			else
			{
				/* Repeated Start */
				if sda_edge == Falling && scl
				{
					in_transmission = true;
					output.push(Section{start: last_ts, end: ts, content: SectionContent::RepeatedStart});
				}
				/* Stop Condition */
				else if sda_edge == Rising && scl
				{
					in_transmission = false;
					address_seen = false;
					output.push(Section{start: last_ts, end: ts, content: SectionContent::StopBit});
				}
				/* Receiving Bits */
				else if scl_edge == Rising
				{
					/* If we dont already have a Full Byte keep adding them up */
					if cur_byte_index < 8
					{
						if cur_byte_index == 0
						{
							cur_byte_start = ts;
						}
						cur_byte |= (sda as u8) << (7 - cur_byte_index);
						cur_byte_index += 1;
					}
					else
					{
						/* If we have already seen the Address we push it as normal byte, else its an Address with RW Bit */
						if address_seen
						{
							output.push(Section{start: cur_byte_start, end: ts, content: SectionContent::Byte(cur_byte)});
						}
						else
						{
							let addr = cur_byte >> 1;
							let rw = cur_byte & 1;
							output.push(Section{start: cur_byte_start, end: ts, content: SectionContent::I2cAddress(addr)});

							output.push(Section{start: last_ts, end: ts, content:
								if rw == 0 { SectionContent::I2cWrite } else { SectionContent::I2cRead }});

							address_seen = true;
						}

						/* Check for Acknowledge after the last Bit */
						let ack = !sda;
						output.push(Section{start: last_ts, end: ts, content:
							if ack { SectionContent::Ack } else { SectionContent::Nak }});

						cur_byte = 0;
						cur_byte_index = 0;
					}
				}
			}

			last_scl = scl;
			last_sda = sda;
			last_ts = ts;
		}

		Ok(())
	}

	fn is_valid(&self) -> bool
	{
		self.sda_pin != self.scl_pin
	}

	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)>
	{
		match idx
		{
			0 => Some(("SDA", self.sda_pin)),
			1 => Some(("SCL", self.scl_pin)),
			_ => None,
		}
	}
}
