use std::iter::Peekable;

// Type used to store one sample containing multiple channels in
pub type Sample = u16;

pub const BUF_SIZE: usize = 10_000;

// Buffer containing samples
pub struct SampleBuffer {
	// SoA to avoid padding while maintaining speed
	pub samples: [Sample; BUF_SIZE],
	pub timestamps: [u32; BUF_SIZE],
	pub len: usize,
}

impl SampleBuffer {
	pub fn clear(&mut self) {
		self.len = 0;
	}

	pub fn push(&mut self, port: Sample, ts: u32) {
		self.samples[self.len] = port;
		self.timestamps[self.len] = ts;
		self.len += 1;
	}

	pub fn get(&self, idx: usize, ch: u32) -> Option<(bool, u32)> {
		if idx >= self.len {
			return None;
		}
		Some((self.samples[idx] & (1 << ch) != 0, self.timestamps[idx]))
	}

	// start: Timstamp of window start
	// Returns sample index
	pub fn find_start(&self, start: u32) -> usize {
		let mut left = 0;
		let mut right = self.len as isize - 1;
		let mut closest_index: usize = 0;
		while left <= right {
			let mid = (left + right) / 2;
			if self.timestamps[mid as usize] <= start {
				closest_index = mid as usize;
				left = mid + 1;
			} else {
				right = mid - 1;
			}
		}

		closest_index
	}

	// start: Timstamp of window end
	// Returns sample index
	pub fn find_end(&self, end: u32) -> usize {
		let mut left = 0;
		let mut right = self.len as isize - 1;
		let mut closest_index: usize = self.len - 1;
		while left <= right {
			let mid = (left + right) / 2;
			if self.timestamps[mid as usize] >= end {
				closest_index = mid as usize;
				right = mid - 1;
			} else {
				left = mid + 1;
			}
		}

		closest_index
	}
}

struct Edge {
	rising: bool,
	at: u32,
}

pub struct EdgeWiseIterator<'a> {
	buffer: &'a SampleBuffer,
	idx: usize,
	ch: u32,
}

impl<'a> EdgeWiseIterator<'a> {
	pub fn new(buffer: &SampleBuffer, ch: u32) -> EdgeWiseIterator<'_> {
		EdgeWiseIterator {
			buffer,
			ch,
			idx: 0,
		}
	}
}

impl<'a> Iterator for EdgeWiseIterator<'a> {
	type Item = Edge;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx >= self.buffer.len {
			return None;
		}

		let (value, timestamp) = self.buffer.get(self.idx, self.ch)?;
		self.idx += 1;

		// Skip all samples with the same value
		while self.idx < self.buffer.len && self.buffer.get(self.idx, self.ch)?.0 == value {
			self.idx += 1;
		}

		Some(Edge { rising: value, at: timestamp })
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

type Pulse = BitSignal;

pub struct PulsewiseIterator<'a> {
	buffer: EdgeWiseIterator<'a>,
	prev: Option<Edge>,
}

impl <'a>From<EdgeWiseIterator<'a>> for PulsewiseIterator<'a> {
	fn from(mut iter: EdgeWiseIterator<'a>) -> Self {
		PulsewiseIterator {
			prev: iter.next(),
			buffer: iter,
		}
	}
}

impl<'a> Iterator for PulsewiseIterator<'a> {
	type Item = Pulse;

	fn next(&mut self) -> Option<Self::Item> {
		let prev_edge = self.prev.take()?;
		let next_edge = self.buffer.next()?;

		let result = Pulse {
			high: prev_edge.rising,
			start: prev_edge.at,
			end: next_edge.at,
		};

		self.prev = Some(next_edge);
		Some(result)
	}
}

// TODO: will be replaced in the future (maybe, it's not that bad)
// will probably be changed and moved in the future
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
		// TODO: remove .round() call - i believe it's not available without the stdlib
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
