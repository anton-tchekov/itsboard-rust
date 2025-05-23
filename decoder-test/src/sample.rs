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

	pub fn bitwise_iter(&self, ch: u32, bit_time: f32) -> BitwiseIterator<'_> {
		BitwiseIterator::new(self, bit_time, ch)
	}

	pub fn pulse_end(&self, mut idx: usize, ch: u32) -> Option<usize> {
		let (initial_bit, _) = self.get(idx, ch)?;
		idx += 1;

		let (mut current_bit, _) = self.get(idx, ch)?;

		while current_bit == initial_bit {
			idx += 1;
			match self.get(idx, ch) {
				Some((next_bit, _)) => current_bit = next_bit,
				None => return Some(idx - 1),
			}
		}

		Some(idx)
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

#[derive(Default)]
pub struct BitData {
	pub high: bool,
	pub end_time: u32,
	pub start_time: u32,
}

#[derive(Default)]
struct Pulse {
	value: bool,
	start: u32,
	end: u32,
	bit_time: u32,
}

// TODO: could prove useful for other protocols, maybe make tests for it
// will probably be changed and moved in the future
pub struct BitwiseIterator<'a> {
	buffer: &'a SampleBuffer,
	idx: usize,
	// TODO: ask why channel is u32 and not u16
	ch: u32,
	expected_bit_time: f32,
	current_pulse: Pulse,
}

impl<'a> BitwiseIterator<'a> {
	pub fn new(buffer: &SampleBuffer, expected_bit_time: f32, ch: u32) -> BitwiseIterator<'_> {
		BitwiseIterator {
			expected_bit_time,
			buffer,
			ch,
			idx: 0,
			current_pulse: Pulse::default(),
		}
	}

	pub fn peek(&mut self) -> Option<BitData> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		};

		Some(BitData {
			high: self.current_pulse.value,
			start_time: self.current_pulse.start,
			end_time: self.current_pulse.start + self.current_pulse.bit_time,
		})
	}

	// Forward the iterator to the next pulse
	// Returns the pulse as BitData
	pub fn next_pulse(&mut self) -> Option<BitData> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		}

		let start = self.current_pulse.start;
		self.current_pulse.start = self.current_pulse.end;

		Some(BitData {
			high: self.current_pulse.value,
			start_time: start,
			end_time: self.current_pulse.end,
		})
	}

	// TODO: improve this. right now it can break the iterator if used wrong
	pub fn next_halve_bit(&mut self) -> Option<BitData> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		}

		let start = self.current_pulse.start;
		self.current_pulse.start += self.current_pulse.bit_time / 2;

		Some(BitData {
			high: self.current_pulse.value,
			start_time: start,
			end_time: self.current_pulse.start,
		})
	}

	pub fn next_bit(&mut self) -> Option<BitData> {
		if self.current_pulse.start == self.current_pulse.end {
			self.current_pulse = self.fetch_next_pulse()?;
		}

		let start = self.current_pulse.start;
		self.current_pulse.start += self.current_pulse.bit_time;

		Some(BitData {
			high: self.current_pulse.value,
			start_time: start,
			end_time: self.current_pulse.start,
		})
	}

	fn fetch_next_pulse(&mut self) -> Option<Pulse> {
		let (value, start_time) = self.buffer.get(self.idx, self.ch)?;
		let end_idx = self.buffer.pulse_end(self.idx, self.ch)?;
		let end_time = self.buffer.timestamps[end_idx];
		self.idx = end_idx;

		Some(self.calculate_pulse(value, start_time, end_time))
	}

	fn calculate_pulse(&self, value: bool, mut start: u32, mut end: u32) -> Pulse {
		// Calc bit timings for the current pulse
		let pulse_duration = end - start;
		// TODO: remove .round() call - i believe it's not available without the stdlib
		let bit_count = (pulse_duration as f32 / self.expected_bit_time).round() as u32;
		// .max, as pulse must describe at least one bit
		let bit_time = pulse_duration / bit_count.max(1);

		let padding = pulse_duration % bit_time;
		let end_padding = padding / 2;
		let start_padding = padding - end_padding;

		start += start_padding;
		end -= end_padding;

		Pulse {
			end,
			start,
			bit_time,
			value,
		}
	}
}

impl<'a> Iterator for BitwiseIterator<'a> {
	type Item = BitData;

	fn next(&mut self) -> Option<Self::Item> {
		self.next_bit()
	}
}
