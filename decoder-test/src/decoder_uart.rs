use crate::bit_reader::BitReader;
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
	Start(StartState),
	Data(DataState),
	Parity(ParityState),
	Stop(StopState)
}

impl DecoderUartState {
	fn process(&self, bits: &mut BitwiseIterator, output: &mut UartOutput, decoder: &DecoderUart) -> Option<DecoderUartState> {
		match &self {
			DecoderUartState::Start(state) => {state.process(bits, output)},
			DecoderUartState::Data(state) => {state.process(bits, output, decoder.databits)},
			DecoderUartState::Parity(state) => {state.process(bits, output, decoder.parity)},
			DecoderUartState::Stop(state) => {state.process(bits, output, decoder.stopbits)},
		}
	}
}

#[derive(Default, Clone, Copy)]
pub struct BitSignal {
	pub high: bool,
	pub end: u32,
	pub start: u32,
}

impl BitSignal {
	pub fn duration(&self) -> u32 {
		self.end - self.start
	}
}

// TODO: will be replaced in the future (maybe, it's not that bad)
// will probably be changed and moved in the future
pub struct BitwiseIterator<'a> {
	buffer: EdgeWiseIterator<'a>,
	expected_bit_time: f32,
	current_pulse: Pulse,
	bit_time: u32,
}

impl<'a> BitwiseIterator<'a> {
	pub fn from(buffer: PulsewiseIterator<'a>, expected_bit_time: f32) -> Self {
		BitwiseIterator {
			buffer,
			expected_bit_time,
			current_pulse: Pulse::default(),
			bit_time: 0,
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

	fn current_time(&self) -> u32 {
		self.current_pulse.start
	}

	// Forward the iterator to the next pulse
	// Returns the pulse as BitData
	pub fn next_pulse(&mut self) -> Option<BitSignal> {
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

	fn fetch_next_pulse(&mut self) -> Option<Pulse> {
		let next = self.buffer.next()?;
		Some(self.calculate_pulse(next))
	}

	fn calculate_pulse(&mut self, mut pulse: Pulse) -> Pulse {
		// Calc bit timings for the current pulse
		let duration = pulse.duration();
		let bit_count = (duration as f32 / self.expected_bit_time).round() as u32;
		// .max, as pulse must describe at least one bit
		let bit_time = duration / bit_count.max(1);

		let padding = duration % bit_time;
		let end_padding = padding / 2;
		let start_padding = padding - end_padding;

		pulse.start += start_padding;
		pulse.end -= end_padding;

		self.bit_time = bit_time;
		pulse
	}
}

impl<'a> Iterator for BitwiseIterator<'a> {
	type Item = BitSignal;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_bit()
	}
}

struct UartOutput<'a> {
	output: &'a mut SectionBuffer
}

impl <'a>UartOutput<'a> {
	fn push(&mut self, section: Section) -> Option<()> {
		self.output.push(section)
	}

	pub fn push_signal(&mut self, bit: BitSignal, content: SectionContent) -> Option<()> {
		self.push(Section { 
			start: bit.start, 
			end: bit.end,
			content
		})
	}
}

#[derive(Copy, Clone)]
struct StartState;

impl StartState {
	pub fn process(&self, bits: &mut BitwiseIterator, output: &mut UartOutput) -> Option<DecoderUartState> {
		let mut bit = bits.next()?;
		if bit.high {
			bits.next_pulse()?;
			bit = bits.next()?;
		}
		output.push_signal(bit, SectionContent::StartBit)?;
		Some(DecoderUartState::Data(DataState::default()))
	}
}

#[derive(Copy, Clone, Default)]
struct DataState;
impl DataState {
	pub fn process(mut self, bits: &mut BitwiseIterator, output: &mut UartOutput, databits: DataBits) -> Option<DecoderUartState> {
		let reader = BitReader::lsb(databits as u8);
		let start = bits.peek()?.start;

		while !reader.is_finished() {
			let bit = bits.next()?;
			reader.read_bit(bit.high);

			output.push_signal(bit, SectionContent::Bit(bit.high))?
		};
		let value = reader.get_value().unwrap();

		output.push(Section { 
			start,
			end: bits.current_time(),
			content: SectionContent::Data(value)
		})?;

		Some(DecoderUartState::Parity(value))
	}
}

#[derive(Copy, Clone)]
struct ParityState {
	word: u32,
}

impl ParityState {
	pub fn process(self, bits: &mut BitwiseIterator, output: &mut UartOutput, parity: Parity) -> Option<DecoderUartState> {
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

		output.push_signal(bit, content)?;
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

	pub fn process(&self, bits: &mut BitwiseIterator, output: &mut UartOutput, stopbits: StopBits) -> Option<DecoderUartState> {
		let start = bits.peek()?.start;
		let bit = bits.next()?;

		let next_bit = match stopbits {
			StopBits::OneAndHalf => { bits.next_halve_bit()?.high },
			StopBits::Two => { bits.next()?.high }
			_ => {true}
		};

		let content = self.get_content(!(next_bit && bit.high));
		let end = bits.current_time();

		output.push(Section { start, end, content})?;
		Some(DecoderUartState::Start(StartState))
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
		if self.baudrate > TIMER_CLOCK_RATE {return Err(())}
		let bit_time = TIMER_CLOCK_RATE as f32 / self.baudrate as f32;

		let mut bits = BitwiseIterator::from(samples, bit_time);
		let mut output = UartOutput {output};
		let mut state = DecoderUartState::Start(StartState);

		loop {
			state = match state.process(&mut bits, &mut output, &self) {
				Some(state) => state,
				None => break
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
			SectionContent::StartBit,
			SectionContent::Word('H' as u32),
			SectionContent::StopBit,
		];

		assert_section_sequence(&mut section_iter, &expected);
	}

	#[test]
	fn test_8n1_hallo() {
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_Hallo.csv", uart);
		let mut section_iter = sections.iter();

		let expected = [
			SectionContent::StartBit, SectionContent::Word('H' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('a' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('l' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('l' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('o' as u32), SectionContent::StopBit,
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
			SectionContent::StartBit, SectionContent::Word('1' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('2' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('3' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('4' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('5' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('6' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('7' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('8' as u32), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Word('9' as u32), SectionContent::StopBit,
		];

		assert_section_sequence(&mut section_iter, &expected);
	}
}
