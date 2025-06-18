use csv::*;

use crate::bit_reader::{BitOrder, BitReader};
use crate::sample::SampleBuffer;
use crate::sample::*;
use crate::decoder::{Section, SectionBufferIter, SectionContent};

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

pub fn expect_bits(amount: u8, mut buf: SectionBufferIter, order: BitOrder, expected: u64) {
    let mut reader = BitReader::new(amount, order);

    for _ in 0..amount {
        let bit = loop {
            match buf.next() {
                Some(s) => match s.content {
                    SectionContent::Bit(b) => break b,
                    _ => continue,
                },
                None => panic!("Unexpected end of buffer while reading bits"),
            }
        };
        reader.read_bit(bit);
    }

    assert_eq!(reader.get_value(), Some(expected));
}

pub fn expect_bits_lsb(amount: u8, buf: SectionBufferIter, expected: u64) {
	expect_bits(amount, buf, BitOrder::LSB, expected);
}

pub fn expect_bits_msb(amount: u8, buf: SectionBufferIter, expected: u64) {
	expect_bits(amount, buf, BitOrder::MSB, expected);
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