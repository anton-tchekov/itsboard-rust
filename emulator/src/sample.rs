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
