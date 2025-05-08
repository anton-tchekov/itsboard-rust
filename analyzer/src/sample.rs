// Type used to store one sample containing multiple channels in
pub type Sample = u16;

// Buffer containing samples
pub struct SampleBuffer<'a> {
	pub sample_rate: u32,
	pub samples: &'a mut [Sample],
	pub len: usize
}
