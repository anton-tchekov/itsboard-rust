pub type Sample = u8;

struct SampleBuffer {
	samples: &'static [Sample]
}

fn sample_blocking(buf: &SampleBuffer) {
}
