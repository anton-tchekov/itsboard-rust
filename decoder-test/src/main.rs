mod decoder;
mod decoder_uart;
mod sample;

use std::fs::read_to_string;

use crate::sample::SampleBuffer;
use crate::decoder::{Decoder, Section, SectionBuffer};
use crate::decoder_uart::{Parity, DataBits, StopBits, DecoderUart};

fn main() {
	let n = 9375;

	let buf = SampleBuffer {
		samples: &[ 0, 1, 0, 1, 0, 1, 0, 1 ],
		time_stamps: &[ 0*n, 1*n, 2*n, 3*n, 4*n, 5*n, 6*n, 7*n ],
		len: 8,
	};

	let uart = DecoderUart {
		rx_pin: 1,
		tx_pin: 0,
		databits: DataBits::Eight,
		parity: Parity::None,
		stopbits: StopBits::One,
		baudrate: 9600
	};

	let mut out_sections = SectionBuffer {
		sections: &mut [Section::default(); 128],
		len: 0
	};

	let count = uart.decode(&buf, &mut out_sections);
}
