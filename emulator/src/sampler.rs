use crate::sample::*;
use crate::delay::delay_ms;
use crate::test_utils::load_sample_buffer;

pub fn sample_blocking(buf: &mut SampleBuffer) {
	buf.clear();
	/*buf.push(0, 0);
	buf.push(1, 30_000_000);
	buf.push(0, 60_000_000);
	buf.push(0, 90_000_000);*/
	let samplebuf = load_sample_buffer("UART/UART_8N1_300_Hallo.csv");
	*buf = samplebuf;

	delay_ms(3000);
}
