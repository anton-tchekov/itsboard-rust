// Type used to store one sample containing multiple channels in
pub type Sample = u16;

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
