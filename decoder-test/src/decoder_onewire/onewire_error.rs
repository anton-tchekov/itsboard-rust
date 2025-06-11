#[derive(Copy, Clone)]
pub enum OneWireError {
	ResponseTooShort,
	ResponseTooLong,
	ResetTooShort,
	ResetTooLong,
	BitInitTooShort,
	BitInitTooLong,
	BitSlotTooShort,
	BitSlotTooLong,
	LineRecoveryTooShort,
	UnexpectedReset,
}

impl OneWireError {
	pub fn to_string(&self) -> &'static str {
		match self {
			OneWireError::ResponseTooShort => "Response too short",
			OneWireError::ResponseTooLong => "Response too long",
			OneWireError::ResetTooShort => "Reset too short",
			OneWireError::ResetTooLong => "Reset too long",
			OneWireError::BitInitTooShort => "Bit initialization too short",
			OneWireError::BitInitTooLong => "Bit initialization too long",
			OneWireError::BitSlotTooShort => "Bit slot too short",
			OneWireError::BitSlotTooLong => "Bit slot too long",
			OneWireError::LineRecoveryTooShort => "Line recovery too short",
			OneWireError::UnexpectedReset => "Unexpected reset pulse",
		}
	}
}