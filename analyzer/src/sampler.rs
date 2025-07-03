use crate::hw::{buttons_read, timer_set, timer_get, blueread};
use crate::sample::{Sample, SampleBuffer};
use crate::decoder::TIMER_CLOCK_RATE;

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

		if ts > 45 * TIMER_CLOCK_RATE
		{
			break;
		}

		if port != prev
		{
			prev = port;
			if buf.len >= buf.samples.len() - 1
			{
				break;
			}

			buf.push(port, ts);
		}
	}

	buf.push(sample(), timer_get());
}
