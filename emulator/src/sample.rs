// Type used to store one sample containing multiple channels in
pub type Sample = u16;

pub const BUF_SIZE: usize = 10_000;

// Buffer containing samples
pub struct SampleBuffer {
	pub samples: [Sample; BUF_SIZE],
	pub timestamps: [u32; BUF_SIZE],
	pub len: usize
}
