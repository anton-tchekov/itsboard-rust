// Type used to store one sample containing multiple channels in
pub type Sample = u16;

// Buffer containing samples
pub struct SampleBuffer<'a> {
	pub samples: &'a mut [Sample],
	pub timestamps: &'a mut [u32],
	pub len: usize
}
