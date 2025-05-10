use crate::hw::*;
use crate::sample::*;

fn sample() -> Sample
{
	blueread() as Sample
}

pub fn sample_blocking(buf: &mut SampleBuffer)
{
	let mut prev = sample();
	buf.clear();
	buf.push(sample(), 0);
	timer_set(0);
	loop
	{
		let buttons = buttons_read() & 0x80;
		if buttons != 0x80
		{
			break;
		}

		let port = sample();
		let ts = timer_get();
		if port != prev
		{
			prev = port;
			if buf.len >= buf.samples.len()
			{
				break;
			}

			buf.push(port, ts);
		}
	}

	buf.push(sample(), timer_get());
}
