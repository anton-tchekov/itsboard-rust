use crate::sample::SampleBuffer;
use crate::delay::delay_ms;
use crate::test_utils::load_sample_buffer;

pub fn sample_blocking(buf: &mut SampleBuffer)
{
	buf.clear();
	let samplebuf = load_sample_buffer("1Wire/OneWireSearchROM.csv");
	*buf = samplebuf;
	delay_ms(1500);
}
