use crate::bit_reader::BitReader;
use crate::decoder::{SectionBuffer, Section, SectionContent, Decoder, DecoderPin, TIMER_CLOCK_RATE};
use crate::sample::{SampleBuffer, BitSignal, Pulse, PulsewiseIterator};
use libm::roundf;

// TODO: use .peekable() in rust core
#[derive(Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Parity {
	None,
	Even,
	Odd
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum DataBits {
	Five = 5,
	Six = 6,
	Seven = 7,
	Eight = 8,
	Nine = 9
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum StopBits {
	One,
	OneAndHalf,
	Two
}

#[derive(Clone, Copy)]
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

pub struct BitwiseIterator<'a> {
	buffer: PulsewiseIterator<'a>,
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
		let bit_count = roundf(duration as f32 / self.expected_bit_time) as u32;
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
		if self.output.is_full() {return None};
		self.output.push(section);
		Some(())
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
	pub fn process(self, bits: &mut BitwiseIterator, output: &mut UartOutput, databits: DataBits) -> Option<DecoderUartState> {
		let mut reader = BitReader::lsb(databits as u8);
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

		Some(DecoderUartState::Parity(ParityState { word: value }))
	}
}

#[derive(Copy, Clone)]
struct ParityState {
	word: u64,
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

#[derive(serde::Serialize, serde::Deserialize)]
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

		let mut bits = BitwiseIterator::from(samples.edge_iter(self.rx_pin).into(), bit_time);
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

	fn is_valid(&self) -> bool
	{
		self.tx_pin != self.rx_pin
	}

	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)> {
		match idx
		{
			0 => Some(("RX" , self.rx_pin)),
			1 => Some(("TX", self.tx_pin)),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::decoder_uart::{StopBits, Parity, DataBits, DecoderUart, SectionContent};
	use crate::test_utils::{decode_sections, assert_bits_lsb_eq, assert_top_layer_eq,
		assert_bit_layer_no_time_overlap, assert_top_layer_no_time_overlap};

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

	#[test]
	fn test_8n1_h() {
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_H.csv", uart);
		let mut section_iter = sections.iter();

		assert_top_layer_eq(&sections, &[
			SectionContent::StartBit, SectionContent::Data('H' as u64), SectionContent::StopBit,
		]);

		// bit layer
		assert_bits_lsb_eq(8, &mut section_iter, 'H' as u64);
	}

	#[test]
	fn test_8n1_hallo() {
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_Hallo.csv", uart);
		let mut section_iter = sections.iter();

		assert_top_layer_eq(&sections, &[
			SectionContent::StartBit, SectionContent::Data('H' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('a' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('l' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('l' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('o' as u64), SectionContent::StopBit,
		]);

		// bit layer
		assert_bits_lsb_eq(8, &mut section_iter, 'H' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, 'a' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, 'l' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, 'l' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, 'o' as u64);
	}

	#[test]
	fn test_8n1_1234567() {
		// TODO: ask Haron if the test was taken correctly (it seems to be wrong), could still be useful
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_123456789.csv", uart);
		let mut section_iter = sections.iter();

		assert_top_layer_eq(&sections, &[
			SectionContent::StartBit, SectionContent::Data('1' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('2' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('3' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('4' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('5' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('6' as u64), SectionContent::StopBit,
			SectionContent::StartBit, SectionContent::Data('7' as u64), SectionContent::StopBit,
			SectionContent::StartBit, // sudden stop
		]);

		// bit layer
		assert_bits_lsb_eq(8, &mut section_iter, '1' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, '2' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, '3' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, '4' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, '5' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, '6' as u64);
		assert_bits_lsb_eq(8, &mut section_iter, '7' as u64);
	}

	#[test]
	fn test_time_overlap() {
		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_Hallo.csv", uart);
		assert_bit_layer_no_time_overlap(&sections);
		assert_top_layer_no_time_overlap(&sections);

		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_H.csv", uart);
		assert_bit_layer_no_time_overlap(&sections);
		assert_top_layer_no_time_overlap(&sections);

		let uart = decoder_8n1_300();
		let sections = decode_sections("UART/UART_8N1_300_123456789.csv", uart);
		assert_bit_layer_no_time_overlap(&sections);
		assert_top_layer_no_time_overlap(&sections);
	}
}
