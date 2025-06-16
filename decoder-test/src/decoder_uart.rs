use crate::decoder::*;
use crate::sample::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Parity {
	None,
	Even,
	Odd
}

#[derive(Clone, Copy)]
pub enum DataBits {
	Five = 5,
	Six = 6,
	Seven = 7,
	Eight = 8,
	Nine = 9
}

#[derive(Clone, Copy)]
pub enum StopBits {
	One,
	OneAndHalf,
	Two
}

#[derive(Copy, Clone)]
enum DecoderUartState {
	Idle(IdleState),
	Start(StartState),
	Data(DataState),
	Parity(ParityState),
	Stop(StopState)
}

// TODO: will be replaced in the future (maybe, it's not that bad)
// will probably be changed and moved in the future
pub struct BitwiseIterator<'a> {
	buffer: EdgeWiseIterator<'a>,
	idx: usize,
	start: u32,
	target_idx: usize,
	expected_bit_time: f32,
	bit_time: u32,
}

impl<'a> BitwiseIterator<'a> {
	pub fn from(buffer: EdgeWiseIterator<'a>, expected_bit_time: f32) -> Self {
		BitwiseIterator {
			start: buffer.current_time(),
			buffer,
			expected_bit_time,
			bit_time: 0,
			target_idx: 0,
			idx: 0
		}
	}

	pub fn peek(&mut self) -> Option<BitSignal> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		};

		Some(BitSignal {
			high: self.current_pulse.high,
			start: self.current_pulse.start,
			end: self.current_pulse.start + self.bit_time,
		})
	}

	// Forward the iterator to the next pulse
	// Returns the pulse as BitData
	pub fn next_edge(&mut self) -> Option<Edge> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		}

		let start = self.current_pulse.start;
		self.current_pulse.start = self.current_pulse.end;

		Some(BitSignal {
			high: self.current_pulse.high,
			start: start,
			end: self.current_pulse.end,
		})
	}

	// TODO: improve this. right now it can break the iterator if used wrong
	pub fn next_halve_bit(&mut self) -> Option<BitSignal> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		}

		let start = self.current_pulse.start;
		self.current_pulse.start += self.bit_time / 2;

		Some(BitSignal {
			high: self.current_pulse.high,
			start: start,
			end: self.current_pulse.start,
		})
	}

	pub fn next_bit(&mut self) -> Option<BitSignal> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		}

		let start = self.current_pulse.start;
		self.current_pulse.start += self.bit_time;

		Some(BitSignal {
			high: self.current_pulse.high,
			start,
			end: self.current_pulse.start,
		})
	}

	fn fetch_next_edge(&mut self) -> Option<Edge> {
		self.start = self.buffer.current_time();
		let next = self.buffer.next()?;
		let end = self.buffer.current_time();

		self.calc_next_target(end);
		Some(next)
	}

	fn current_time(&self) -> u32 {
		self.start + self.idx * self.bit_time
	}

	fn calc_next_target(&mut self, end: u32) {
		// Calc bit timings for the current pulse
		let duration = end - self.start;
		// TODO: remove .round() call - i believe it's not available without the stdlib
		// .max, as pulse must describe at least one bit
		let bit_count = (duration as f32 / self.expected_bit_time).round().max(1) as u32;
		let bit_time = duration / bit_count;

		let padding = duration % bit_time;
		let end_padding = padding / 2;
		let start_padding = padding - end_padding;

		self.start += start_padding;
		self.bit_time = bit_time;
		self.target_idx = bit_count;
		self.idx = 0;
	}
}

impl<'a> Iterator for BitwiseIterator<'a> {
	type Item = BitSignal;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_bit()
	}
}

#[derive(Copy, Clone)]
struct IdleState;

impl IdleState {
	pub fn process(&self, bits: &mut BitwiseIterator, data: &mut Option<Section>) -> Option<DecoderUartState> {
		if bits.peek()?.high {
			let bit = bits.next_pulse()?;
			*data = Some(Section::from_bit(&bit, SectionContent::Empty));
		}

		Some(DecoderUartState::Start(StartState))
	}
}

#[derive(Copy, Clone)]
struct StartState;

impl StartState {
	pub fn process(&self, bits: &mut BitwiseIterator, data: &mut Option<Section>) -> Option<DecoderUartState> {
		*data = Some(Section::from_bit(&bits.next()?, SectionContent::StartBit));

		Some(DecoderUartState::Data(DataState::default()))
	}
}

#[derive(Copy, Clone, Default)]
struct DataState;

impl DataState {

	fn iterate(&mut self, databits: DataBits, bits: &mut BitwiseIterator, section: &mut Section, word: &mut u32) -> Option<()> {
		for i in 0..databits as usize {
			let bit = bits.next()?;

			*word |= (bit.high as u32) << i;
			section.end = bit.end_time;
		}

		Some(())
	}

	pub fn process(mut self, bits: &mut BitwiseIterator, data: &mut Option<Section>, databits: DataBits) -> Option<DecoderUartState> {
		let mut word: u32 = 0;

		let mut section = Section::default();
		section.start = bits.peek()?.start_time;

		let completed = self.iterate(databits, bits, &mut section, &mut word);
		section.content = SectionContent::Word(word);

		*data = Some(section);

		let result = match completed {
			Some(()) => {Some(DecoderUartState::Parity(ParityState { word }))},
			None => {None}
		};

		result
	}
}

#[derive(Copy, Clone)]
struct ParityState {
	word: u32,
}

impl ParityState {
	pub fn process(self, bits: &mut BitwiseIterator, data: &mut Option<Section>, parity: Parity) -> Option<DecoderUartState> {
		if parity == Parity::None {
			return Some(DecoderUartState::Stop(StopState))
		}

		let bit = bits.next()?;

		let ones = self.word.count_ones() + bit.high as u32;
		let is_even = ones % 2 == 0;

		let content = match (parity, is_even) {
			(Parity::Even, false) => {SectionContent::Err("expected even parity, but was odd")},
			(Parity::Odd, true) => {SectionContent::Err("expected odd parity, but was even")},
			_ => SectionContent::ParityBit(bit.high)
		};

		*data = Some(Section::from_bit(&bit, content));
		Some(DecoderUartState::Stop(StopState))
	}
}

#[derive(Copy, Clone)]

struct StopState;

impl StopState {
	fn get_content(&self, has_error: bool) -> SectionContent {
		if has_error {
			return SectionContent::Err("expected high bit value, but was low")
		}

		SectionContent::StopBit
	}

	pub fn process(&self, bits: &mut BitwiseIterator, data: &mut Option<Section>, stopbits: StopBits) -> Option<DecoderUartState> {
		let bit = bits.next()?;
		let mut section = Section::from_bit(&bit, SectionContent::StopBit);

		let next_bit = match stopbits {
			StopBits::One => {
				*data = Some(section);
				return Some(DecoderUartState::Idle(IdleState))
			},
			StopBits::OneAndHalf => { bits.next_halve_bit()?},
			StopBits::Two => { bits.next()? }
		};

		section.content = self.get_content(!(next_bit.high && bit.high));
		section.end = next_bit.end_time;
		*data = Some(section);

		Some(DecoderUartState::Idle(IdleState))
	}
}

pub struct DecoderUart {
	pub rx_pin: DecoderPin,
	pub tx_pin: DecoderPin,
	pub databits: DataBits,
	pub parity: Parity,
	pub stopbits: StopBits,
	pub baudrate: u32,
}

impl Decoder for DecoderUart {

	fn decode(&self, samples: &SampleBuffer, output: &mut SectionBuffer) -> Result<(), ()> {
		let bit_time = TIMER_CLOCK_RATE as f32 / self.baudrate as f32;
		let mut bits = samples.bitwise_iter(self.rx_pin, bit_time);

		let mut section: Option<Section> = None;
		let mut state = DecoderUartState::Idle(IdleState);

		while bits.peek().is_some() {
			let result: Option<DecoderUartState> = match state {
				DecoderUartState::Idle(state) => {state.process(&mut bits, &mut section)},
				DecoderUartState::Start(state) => {state.process(&mut bits, &mut section)},
				DecoderUartState::Data(state) => {state.process(&mut bits, &mut section, self.databits)},
				DecoderUartState::Parity(state) => {state.process(&mut bits, &mut section, self.parity)},
				DecoderUartState::Stop(state) => {state.process(&mut bits, &mut section, self.stopbits)},
			};

			if let Some(result_section) = section {
				output.push(result_section)?;
				section = None;
			}

			match result {
				Some(new_state) => {state = new_state},
				None => {break}
			}
		}

		Ok(())
	}

	fn num_pins(&self) -> usize {
		2
	}

	fn get_pin(&self, idx: usize) -> Option<DecoderPin> {
		match idx
		{
			0 => Some(self.rx_pin),
			1 => Some(self.tx_pin),
			_ => None,
		}
	}

	fn get_pin_name(&self, idx: usize) -> Option<&'static str> {
		match idx
		{
			0 => Some("RX"),
			1 => Some("TX"),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::sample::*;
	use crate::decoder::*;
	use crate::decoder_uart::*;
	use crate::test_utils::*;

	fn decoder_8n1_300() -> DecoderUart {
		DecoderUart {
			rx_pin: 0,
			tx_pin: 1,
			databits: DataBits::Eight,
			parity: Parity::None,
			stopbits: StopBits::One,
			baudrate: 300,
		}
	}

	fn decode_sections(file: &str, uart: DecoderUart) -> SectionBuffer {
		let buf = load_sample_buffer(file);

		let mut out_sections = SectionBuffer {
			sections: [Section::default(); SECBUF_SIZE],
			len: 0,
		};

		let result  = uart.decode(&buf, &mut out_sections);
		assert!(result.is_ok());

		out_sections
	}

	fn assert_section_sequence(actual: &mut SectionBufferIter, expected: &[SectionContent]) {
		for expected_content in expected {
			expect_section(actual.next(), *expected_content);
		}
		assert!(actual.next().is_none());
	}

	#[test]
	fn test_8n1_h() {
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_H.csv", uart);
		let mut section_iter = sections.iter();

		let expected = [
			SectionContent::Empty,
			SectionContent::StartBit,
			SectionContent::Word('H' as u32),
			SectionContent::StopBit,
			SectionContent::Empty,
		];

		assert_section_sequence(&mut section_iter, &expected);
	}

	#[test]
	fn test_8n1_hallo() {
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_Hallo.csv", uart);
		let mut section_iter = sections.iter();

		let expected = [
			SectionContent::Empty,
			SectionContent::StartBit, SectionContent::Word('H' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('a' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('l' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('l' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('o' as u32), SectionContent::StopBit,
			SectionContent::Empty,
		];

		assert_section_sequence(&mut section_iter, &expected);
	}

	#[test]
	fn test_8n1_123456789() {
		// TODO: ask Haron if the test was taken correctly (it seems to be wrong), could still be useful
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_123456789.csv", uart);


		for section in sections.iter() {
			println!("{:?}", section);
		}

		let mut section_iter = sections.iter();

		let expected = [
			SectionContent::Empty,
			SectionContent::StartBit, SectionContent::Word('1' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('2' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('3' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('4' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('5' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('6' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('7' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('8' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('9' as u32), SectionContent::StopBit,
			SectionContent::Empty,
		];

		assert_section_sequence(&mut section_iter, &expected);
	}

}
