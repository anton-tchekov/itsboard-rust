use crate::hw::*;
use crate::sample::*;

fn sample() -> Sample {
	(blueread() as Sample) | (yellowread() as Sample) << 8
}

pub fn sample_blocking(buf: &mut SampleBuffer) {
	let mut prev = sample();
	buf.len = 0;
	loop {
		let buttons = buttons_read() & 0x80;
		if buttons != 0x80
		{
			break;
		}

		let port = sample();
		let ts = timer_get();
		if port != prev {
			prev = port;
			if buf.len >= buf.samples.len() {
				break;
			}

			buf.samples[buf.len] = port;
			buf.timestamps[buf.len] = ts;
			buf.len += 1;
		}
	}
}
