use crate::hw::*;

// Type used to store one sample containing multiple channels in
// Switch to u16 later for 16 channels
pub type Sample = u8;

// Buffer containing samples
pub struct SampleBuffer<'a> {
	pub sample_rate: u32,
	pub samples: &'a mut [Sample],
	pub len: usize
}

impl SampleBuffer<'_> {
	// To later make paging possible
	pub fn get_sample(&self, idx: usize) -> Sample {
		self.samples[idx]
	}
}

const INTERVAL: u32 = 90;

fn sample() -> u8 {
	buttons_read()
}

// 1. Iteration, just collect samples in a fixed loop
pub fn sample_blocking(buf: &mut SampleBuffer) {
	buf.sample_rate = 1_000_000;

	let first = sample();
	while sample() == first {}

	let mut start = timer_get();

	for i in 0..buf.samples.len() {
		buf.samples[i] = sample();
		while timer_get() - start < INTERVAL {}
		start += INTERVAL;
	}

	buf.len = buf.samples.len();
	sample_trim(buf);
}

// Remove samples at end if all the same to reduce sending over UART
fn sample_trim(buf: &mut SampleBuffer) {
	let mut i = buf.len - 1;
	let last = buf.samples[i];
	while i > 0 {
		i -= 1;
		if buf.samples[i] != last {
			break;
		}
	}

	buf.len = i;
}
