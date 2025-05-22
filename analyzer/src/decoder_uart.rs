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
		let mut bits = samples.bitwise_iter(self.rx_pin as u32, bit_time);

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
	
	fn get_pin(&self, idx: usize) -> Option<(&'static str, DecoderPin)> {
		match idx
		{
			0 => Some(("RX" , self.rx_pin)),
			1 => Some(("TX", self.tx_pin)),
			_ => None,
		}
	}
}
