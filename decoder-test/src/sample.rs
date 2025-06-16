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

	pub fn edge_iter(&self, ch: u32) -> EdgeWiseIterator<'_> {
		EdgeWiseIterator::new(self, ch)
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

#[derive(PartialEq, Eq)]
pub enum Edge {
	Rising,
	Falling
}

impl From<bool> for Edge {
	// bool is the value of the previous pulse
	fn from(value: bool) -> Self {
		match value {
			true => Edge::Falling,
			false => Edge::Rising
		}
	}
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

	pub fn current_index(&self) -> usize {
		self.idx
	}

	pub fn set_index(&mut self, idx: usize) -> Result<(), ()> {
		if idx > self.buffer.len {
			return Err(())
		}
		self.idx = idx;
		Ok(())
	}

	pub fn current_time(&self) -> u32 {
		if self.buffer.len == 0 {
			return 0
		}
		self.buffer.timestamps[self.idx]
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

		Some(Edge::from(value))
	}
}