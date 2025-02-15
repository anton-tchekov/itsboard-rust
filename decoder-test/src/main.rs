mod decoder;
mod decoder_i2c;
mod decoder_spi;
mod decoder_uart;
mod decoder_onewire;
mod sample;

use std::fs::read_to_string;

use crate::sample::SampleBuffer;
use crate::decoder::{Decoder, Range, Section};
use crate::decoder_uart::{Parity, DataBits, StopBits, DecoderUart};

// Die Datei wurde mit einer Sample Rate von 1.000.000 pro sek aufgenommen
// 8 Channels <=> Immer ein 1 Byte pro Sample
// Channel 0 -> Bit 0 etc.
// Die Dateien sind als .txt, damit man leichter manuell anpassen kann vlt.

// data/uart.txt
// Ist die Kommunikation über UART mit 9600 Baud, 8N1
// RX Pin: 1
// TX Pin: 0
// Auf TX wird gesendet: "Hello World!\n"

// Unten als Beispiel das Auslesen und Ausgeben der Channels
// ~ Anton

fn main() {
	let filename = "data/uart.txt";

	// --- Einlesen ---
	// Ineffizient aber egal
	let mut lines = Vec::new();
	for line in read_to_string(filename).unwrap().lines() {
		lines.push(line.to_string())
	}

	let mut data: Vec<u8> = Vec::new();
	for line in &lines {
		let mut byte = 0;
		let mut i = 0;
		for c in line.bytes() {
			if c == b'1' {
				byte |= 1 << i;
			}

			i += 1;
		}

		data.push(byte);
	}
	// ----------------

	let buf = SampleBuffer {
		sample_rate: 1_000_000,
		samples: &data[..],
		len: data.len()
	};

	let uart = DecoderUart {
		rx_pin: 1,
		tx_pin: 0,
		databits: DataBits::Eight,
		parity: Parity::None,
		stopbits: StopBits::One,
		baudrate: 9600
	};

	let mut out_sections: [Section; 128] = [Section::default(); 128];

	let count = uart.decode(&buf, Range { start: 0, len: buf.len, }, &mut out_sections);
}
