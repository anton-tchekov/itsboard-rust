use crate::sample::*;

pub fn sample_blocking(buf: &mut SampleBuffer) {
	buf.clear();
	buf.push(0, 0);
	buf.push(1, 30_000_000);
	buf.push(0, 60_000_000);
	buf.push(0, 100_000_000);
}
