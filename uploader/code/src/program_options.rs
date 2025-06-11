#[derive(Debug, PartialEq)]
pub enum ProgramOptions
{
	None,
	LogicAnalyzer,
}

impl std::string::ToString for ProgramOptions
{
	fn to_string(&self) -> String 
	{
		match self
		{
			ProgramOptions::None => String::from(""),
			ProgramOptions::LogicAnalyzer => String::from("../programs/logic_analyzer.hex"),
		}
	}
}