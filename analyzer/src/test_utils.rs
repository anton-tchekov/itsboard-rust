use crate::bit_reader::{BitOrder, BitReader};
use crate::sample::{SampleBuffer, BUF_SIZE};
use crate::decoder::{Decoder, Section, SectionBuffer, SectionBufferIter, SectionContent, SECBUF_SIZE};

const BASE_PATH: &str = "../sample_data/";

pub fn load_buf_from_csv(filename: &str, buf: &mut SampleBuffer) -> Result<(), csv::Error>
{
	let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_path(filename)?;

	for result in rdr.records()
	{
		let record = result?;

		let timestamp = &record[0].parse::<u32>().unwrap();
		let data = &record[1].parse::<u16>().unwrap();

		buf.push(*data as u8, *timestamp);
	}

	Ok(())
}

pub fn assert_top_layer_eq(actual: &SectionBuffer, expected: &[SectionContent]) {
	assert_eq!(&top_layer(actual), &expected);
}

pub fn decode_sections(file: &str, decoder: impl Decoder) -> SectionBuffer {
	let buf = load_sample_buffer(file);

	let mut out_sections = SectionBuffer {
		sections: [Section::default(); SECBUF_SIZE],
		len: 0,
	};

	let result  = decoder.decode(&buf, &mut out_sections);
	assert!(result.is_ok());

	out_sections
}

pub fn assert_bits_eq(amount: u8, buf: &mut SectionBufferIter, order: BitOrder, expected: u64) {
	let mut reader = BitReader::new(amount, order);

	while !reader.is_finished() {
		match buf.next() {
			Some(s) => match s.content {
				SectionContent::Bit(b) => reader.read_bit(b),
				_ => continue,
			},
			None => panic!("Unexpected end of buffer while reading bits"),
		};
	}

	assert_eq!(reader.get_value(), Some(expected));
}

pub fn assert_bits_lsb_eq(amount: u8, buf: &mut SectionBufferIter, expected: u64) {
	assert_bits_eq(amount, buf, BitOrder::LSB, expected);
}

pub fn assert_bits_msb_eq(amount: u8, buf: &mut SectionBufferIter, expected: u64) {
	assert_bits_eq(amount, buf, BitOrder::MSB, expected);
}

fn assert_no_time_overlap(buf: &SectionBuffer, is_bit_layer: bool) {
	let buf: Vec<Section> = buf
		.iter()
		.filter(|s| match s.content {
			SectionContent::Bit(_) => is_bit_layer,
			_ => !is_bit_layer
		})
		.cloned()
		.collect();

	let mut iter = buf.iter();
	let mut prev = match iter.next() {
		Some(s) => s,
		None => return,
	};

	for section in iter {
		assert!(prev.end <= section.start);
		prev = section
	}
}

pub fn assert_bit_layer_no_time_overlap(buf: &SectionBuffer) {
	assert_no_time_overlap(buf, true);
}

pub fn assert_top_layer_no_time_overlap(buf: &SectionBuffer) {
	assert_no_time_overlap(buf, false);
}

fn top_layer(buf: &SectionBuffer) -> Vec<SectionContent> {
	buf
	.iter()
	.filter_map(|s| match s.content {
		SectionContent::Bit(_) => None,
		other => Some(other),
	})
	.collect()
}

fn bit_layer(buf: &SectionBuffer) -> Vec<bool> {
	buf
	.iter()
	.filter_map(|s| match s.content {
		SectionContent::Bit(b) => Some(b),
		_ => None,
	})
	.collect()
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