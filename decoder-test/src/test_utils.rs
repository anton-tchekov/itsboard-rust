use csv::*;

use crate::sample::SampleBuffer;
use crate::sample::*;
use crate::decoder::{Section, SectionContent};

const BASE_PATH: &str = "../sample_data/";

pub fn load_buf_from_csv(filename: &str, buf: &mut SampleBuffer) -> Result<()>
{
	let mut rdr = ReaderBuilder::new().has_headers(true).from_path(filename)?;

	for result in rdr.records()
	{
		let record = result?;

		let timestamp = &record[0].parse::<u32>().unwrap();
		let data = &record[1].parse::<u16>().unwrap();

		buf.push(*data, *timestamp);
	}

	Ok(())
}

pub fn load_sample_buffer(path: &str) -> SampleBuffer {
	let mut buf = SampleBuffer {
		samples: [0; BUF_SIZE],
		timestamps: [0; BUF_SIZE],
		len: 0,
	};

	load_buf_from_csv(&format!("{BASE_PATH}{path}"), &mut buf).expect("Failed to load buffer from CSV");

	buf
}

pub fn expect_section(section: Option<&Section>, content: SectionContent)
{
	assert!(section.is_some());

	let section = section.unwrap();
	assert_eq!(section.content, content);
}