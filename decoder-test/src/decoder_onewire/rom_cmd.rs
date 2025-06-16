use crate::decoder_onewire::onewire_error::OneWireError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ROMCmd {
	ReadROM,
	SkipROM,
	MatchROM,
	SearchROM,
	OverdriveSkipROM,
	OverdriveMatchROM
}

impl TryFrom<u8> for ROMCmd {
	type Error = OneWireError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x33 => Ok(ROMCmd::ReadROM),
			0xCC => Ok(ROMCmd::SkipROM),
			0x55 => Ok(ROMCmd::MatchROM),
			0xF0 => Ok(ROMCmd::SearchROM),
			0x3C => Ok(ROMCmd::OverdriveSkipROM),
			0x69 => Ok(ROMCmd::OverdriveMatchROM),
			_ => Err("Invalid ROM command"),
		}
	}
}

impl ROMCmd {
	fn to_string(&self) -> OneWireError {
		match self {
			ROMCmd::ReadROM => "Read ROM",
			ROMCmd::SkipROM => "Skip ROM",
			ROMCmd::MatchROM => "Match ROM",
			ROMCmd::SearchROM => "Search ROM",
			ROMCmd::OverdriveSkipROM => "Overdrive Skip ROM",
			ROMCmd::OverdriveMatchROM => "Overdrive Match ROM",
		}
	}
}