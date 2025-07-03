// Type used to store one sample containing multiple channels in
pub type Sample = u8;

pub const BUF_SIZE: usize = 1000;

// Buffer containing samples
pub struct SampleBuffer
{
	// SoA to avoid padding while maintaining speed
	pub samples: [Sample; BUF_SIZE],
	pub timestamps: [u32; BUF_SIZE],
	pub len: usize
}

impl Default for SampleBuffer
{
	fn default() -> Self
	{
		Self::new()
	}
}

impl SampleBuffer
{
	pub fn new() -> Self
	{
		SampleBuffer
		{
			samples: [0; BUF_SIZE],
			timestamps: [0; BUF_SIZE],
			len: 0
		}
	}

	pub fn clear(&mut self)
	{
		self.len = 0;
	}

	pub fn push(&mut self, port: Sample, ts: u32)
	{
		self.samples[self.len] = port;
		self.timestamps[self.len] = ts;
		self.len += 1;
	}

	pub fn get(&self, idx: usize, ch: u32) -> (bool, u32)
	{
		(self.samples[idx] & (1 << ch) != 0, self.timestamps[idx])
	}

	// TODO: maybe change name or merge it with get
	pub fn get_content(&self, idx: usize, ch: u32) -> Option<(bool, u32)>
	{
		if idx >= self.len {
			return None;
		}
		Some((self.samples[idx] & (1 << ch) != 0, self.timestamps[idx]))
	}

	pub fn edge_iter(&self, ch: u32) -> EdgeWiseIterator<'_>
	{
		EdgeWiseIterator::new(self, ch)
	}

	// start: Timstamp of window start
	// Returns sample index
	pub fn find_prev(&self, start: u32) -> usize
	{
		if self.len == 0
		{
			return 0;
		}

		let mut left = 0;
		let mut right = self.len as isize - 1;
		let mut closest_index: usize = 0;
		while left <= right
		{
			let mid = (left + right) / 2;
			if self.timestamps[mid as usize] < start
			{
				closest_index = mid as usize;
				left = mid + 1;
			}
			else
			{
				right = mid - 1;
			}
		}

		closest_index
	}

	// start: Timstamp of window start
	// Returns sample index
	pub fn find_start(&self, start: u32) -> usize
	{
		let mut left = 0;
		let mut right = self.len as isize - 1;
		let mut closest_index: usize = 0;
		while left <= right
		{
			let mid = (left + right) / 2;
			if self.timestamps[mid as usize] <= start
			{
				closest_index = mid as usize;
				left = mid + 1;
			}
			else
			{
				right = mid - 1;
			}
		}

		closest_index
	}

	// start: Timstamp of window end
	// Returns sample index
	pub fn find_end(&self, end: u32) -> usize
	{
		let mut left = 0;
		let mut right = self.len as isize - 1;
		let mut closest_index: usize = self.len - 1;
		while left <= right
		{
			let mid = (left + right) / 2;
			if self.timestamps[mid as usize] >= end
			{
				closest_index = mid as usize;
				right = mid - 1;
			}
			else
			{
				left = mid + 1;
			}
		}

		closest_index
	}

	// start: Timstamp of window end
	// Returns sample index
	pub fn find_next(&self, end: u32) -> usize
	{
		if self.len == 0
		{
			return 0;
		}

		let mut left = 0;
		let mut right = self.len as isize - 1;
		let mut closest_index: usize = self.len - 1;
		while left <= right
		{
			let mid = (left + right) / 2;
			if self.timestamps[mid as usize] > end
			{
				closest_index = mid as usize;
				right = mid - 1;
			}
			else
			{
				left = mid + 1;
			}
		}

		closest_index
	}
}

impl<'a> IntoIterator for &'a SampleBuffer
{
	type Item = (u32, Sample);
	type IntoIter = SampleBufferIterator<'a>;

	fn into_iter(self) -> Self::IntoIter
	{
		SampleBufferIterator
		{
			buf: self,
			idx: 0
		}
	}
}

pub struct SampleBufferIterator<'a>
{
	buf: &'a SampleBuffer,
	idx: usize
}

impl<'a> Iterator for SampleBufferIterator<'a>
{
	type Item = (u32, Sample);
	fn next(&mut self) -> Option<(u32, Sample)>
	{
		if self.idx >= self.buf.len
		{
			return None;
		}

		let result = (self.buf.timestamps[self.idx], self.buf.samples[self.idx]);
		self.idx += 1;
		Some(result)
	}
}

#[derive(PartialEq, Eq, Debug)]
pub enum Edge
{
	Rising,
	Falling
}

impl From<bool> for Edge
{
	// bool is the value of the previous pulse
	fn from(value: bool) -> Self
	{
		match value
		{
			true => Edge::Falling,
			false => Edge::Rising
		}
	}
}

impl Into<bool> for Edge
{
	fn into(self) -> bool
	{
		match self
		{
			Edge::Falling => true,
			Edge::Rising => false
		}
	}
}

pub struct EdgeWiseIterator<'a>
{
	buffer: &'a SampleBuffer,
	idx: usize,
	ch: u32,
}

impl<'a> EdgeWiseIterator<'a>
{
	pub fn new(buffer: &SampleBuffer, ch: u32) -> EdgeWiseIterator<'_>
	{
		EdgeWiseIterator
		{
			buffer,
			ch,
			idx: 0,
		}
	}

	pub fn current_index(&self) -> usize
	{
		self.idx
	}

	pub fn set_index(&mut self, idx: usize) -> Result<(), ()>
	{
		if idx >= self.buffer.len
		{
			return Err(())
		}
		self.idx = idx;
		Ok(())
	}

	pub fn current_time(&self) -> u32
	{
		if self.buffer.len == 0
		{
			return 0
		}
		self.buffer.timestamps[self.idx]
	}
}

impl<'a> Iterator for EdgeWiseIterator<'a>
{
	type Item = Edge;

	fn next(&mut self) -> Option<Self::Item>
	{
		self.set_index(self.idx + 1).ok()?;
		let (value, _) = self.buffer.get_content(self.idx - 1, self.ch)?;

		// Skip all samples with the same value
		while self.buffer.get_content(self.idx, self.ch)?.0 == value && self.set_index(self.idx + 1).is_ok() {}
		Some(Edge::from(value))
	}
}

#[derive(Default, Clone, Copy)]
pub struct BitSignal
{
	pub high: bool,
	pub end: u32,
	pub start: u32,
}

impl BitSignal
{
	pub fn duration(&self) -> u32
	{
		self.end - self.start
	}
}

pub type Pulse = BitSignal;

pub struct PulsewiseIterator<'a>
{
	buffer: EdgeWiseIterator<'a>,
}

impl <'a>From<EdgeWiseIterator<'a>> for PulsewiseIterator<'a>
{
	fn from(iter: EdgeWiseIterator<'a>) -> Self
	{
		PulsewiseIterator
		{
			buffer: iter,
		}
	}
}

impl<'a> Iterator for PulsewiseIterator<'a>
{
	type Item = Pulse;

	fn next(&mut self) -> Option<Self::Item>
	{
		let start = self.buffer.current_time();
		let next_edge = self.buffer.next()?;
		let end = self.buffer.current_time();

		Some(Pulse {
			high: next_edge.into(),
			start,
			end,
		})
	}
}

#[cfg(test)]
mod tests
{
	use crate::sample::{SampleBuffer, Edge};
	use crate::test_utils::load_sample_buffer;

	fn sample_buffer() -> SampleBuffer
	{
		load_sample_buffer("UART/UART_8N1_300_H.csv")
	}

	#[test]
	fn test_get()
	{
		let buf = sample_buffer();
		assert_eq!(buf.len, 8);

		assert_eq!(buf.get_content(0, 0), Some((true, 0)));
		assert_eq!(buf.get_content(1, 0), Some((false, 38173934)));
		assert_eq!(buf.get_content(2, 0), Some((true, 39372990)));
		assert_eq!(buf.get_content(3, 0), Some((false, 39672860)));
		assert_eq!(buf.get_content(4, 0), Some((true, 40272490)));
		assert_eq!(buf.get_content(5, 0), Some((false, 40572216)));
		assert_eq!(buf.get_content(6, 0), Some((true, 40872038)));
		assert_eq!(buf.get_content(7, 0), Some((true, 65141112)))
	}

	#[test]
	fn test_edge_iterator()
	{
		let buf = sample_buffer();
		let edge_iter = buf.edge_iter(0);

		let edges: Vec<Edge> = edge_iter.collect();

		assert_eq!(
			edges,
			vec![
				Edge::Falling,
				Edge::Rising,
				Edge::Falling,
				Edge::Rising,
				Edge::Falling,
				Edge::Rising,
				Edge::Falling,
			]
		);
	}
}
