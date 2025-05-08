use crate::hw::*;
use crate::sample::*;

const INTERVAL: u32 = 90;

fn sample() -> Sample {
	(blueread() as Sample) | (yellowread() as Sample) << 8
}

// 1. Iteration, just collect samples in a fixed loop
pub fn sample_blocking(buf: &mut SampleBuffer) {
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
