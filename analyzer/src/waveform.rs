use crate::lcd::{LCD_WIDTH};

pub const CHANNEL_LABEL_WIDTH: u32 = 25;
pub const WAVEFORM_W: u32 = LCD_WIDTH - 1 - CHANNEL_LABEL_WIDTH;
pub const WAVEFORM_H: u32 = 18;
pub const WAVEFORM_SPACING: u32 = 26;
pub const WAVEFORMS_Y: u32 = 81;
pub const WAVEFORM_PIN_Y: u32 = 15;

pub struct WaveformBuffer
{
	data: [u16; WAVEFORM_W as usize]
}

impl WaveformBuffer
{
	pub fn new() -> Self
	{
		WaveformBuffer { data: [0; WAVEFORM_W as usize] }
	}

	pub fn line(&mut self, ch: u32, x0: u32, x1: u32, level: bool)
	{
		let bit = ch + if level == false { 1 } else { 0 };
		let mask = 1 << bit;
		for x in x0..=x1
		{
			self.data[x as usize] |= mask;
		}
	}

	//pub fn blit(&self, ch: u32, )

	pub fn update(&self, prev: &WaveformBuffer)
	{
	}

	pub fn render(&self)
	{

	}
}
