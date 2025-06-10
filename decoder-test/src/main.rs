mod decoder;
mod decoder_uart;
mod decoder_onewire;
mod sample;
mod test_utils;

use crate::sample::SampleBuffer;
use crate::decoder::{Decoder, Section, SectionBuffer};
use crate::decoder_uart::{Parity, DataBits, StopBits, DecoderUart};
use crate::test_utils::load_buf_from_csv;

fn main() {
	let n = 9375;

	let mut buf = SampleBuffer {
		samples: [0; sample::BUF_SIZE],
		timestamps: [0; sample::BUF_SIZE],
		len: 0
	};

	/* 
	Das Format ist PROTOCOL_CONFIG_BAUDRATE_DATA
	also sende ich hier über UART 8 Bits ohne Parity Bit und einem Stop Bit
	bei einer Baudrate von 300 den Character 'H'
	*/
	load_buf_from_csv("../sample_data/UART/UART_8N1_300_H.csv", &mut buf).unwrap();

	/* Printed die Daten die man aus der CSV gelesen hat zur Überprüfung, kann gerne weg */
	for i in 0..buf.len
	{
		println!("{},{}", buf.timestamps[i], buf.samples[i]);
	}

	let uart = DecoderUart {
		rx_pin: 1,
		tx_pin: 0,
		databits: DataBits::Eight,
		parity: Parity::None,
		stopbits: StopBits::One,
		baudrate: 9600
	};

	let mut out_sections = SectionBuffer {
		sections: [Section::default(); decoder::SECBUF_SIZE],
		len: 0
	};

	let count = uart.decode(&buf, &mut out_sections);
}