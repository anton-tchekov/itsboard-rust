// ==== WARNING : MOSTLY UNTESTED ====

const SAMPLERATE: i32 = 1_000_000;
const CHANNELS: i32 = 8;
const BUFFER_SIZE: usize = 100_000;
type Sample = u8;

enum Parity {
	None,
	Odd,
	Even
}

struct UartConfig {
	start: usize,
	channel: i32, // 0-7
	baudrate: i32,
	databits: i32, // 5-8
	parity: Parity,
	stopbits: i32 // 1 or 2
}

fn range_set(start: usize, len: usize, channel: i32, value: bool, output: &mut [Sample]) -> usize {
	eprint!("{}", if value { 1 } else { 0 });

	let end = start + len;
	if value {
		let mask = 1 << channel;
		for i in start..end {
			output[i] |= mask;
		}
	}
	else {
		let mask = !(1 << channel);
		for i in start..end {
			output[i] &= mask;
		}
	}

	end
}

fn uart_generate(config: UartConfig, bytes: &[u8], output: &mut [Sample]) {
	assert!(config.databits >= 5 && config.databits <= 8);
	assert!(config.stopbits >= 1 && config.stopbits <= 2);
	assert!(config.baudrate > 0);

	let bittime = (SAMPLERATE / config.baudrate) as usize;

	eprintln!("Debug: Bittime = {}", bittime);

	eprint!("BitString: ");

	let mut pos = config.start;
	for byte in bytes {
		// Start Bit
		pos = range_set(pos, bittime, config.channel, false, output);

		// Data Bits LSB first
		let mut ones = 0;
		for i in 0..config.databits {
			let is_one = byte & (1 << i) != 0;
			pos = range_set(pos, bittime, config.channel, is_one, output);
			if is_one {
				ones += 1;
			}
		}

		// Insert parity
		match config.parity {
			Parity::Even => {
				pos = range_set(pos, bittime, config.channel, ones & 1 != 0, output);
			}
			Parity::Odd => {
				pos = range_set(pos, bittime, config.channel, ones & 1 == 0, output);
			}
			Parity::None => {}
		}

		// Insert stop bit(s)
		for _i in 0..config.stopbits {
			pos = range_set(pos, bittime, config.channel, true, output);
		}

		eprint!(" ");
	}

	eprintln!();
}

fn dump(data: &[Sample]) {
	for sample in data {
		for i in (0..CHANNELS).rev() {
			print!("{}", if sample & (1 << i) != 0 { 1 } else {0});
		}

		println!();
	}
}

fn main() {
	let mut data: [Sample; BUFFER_SIZE] = [0; BUFFER_SIZE];

	let message = "Hello World!\n";

	uart_generate(UartConfig{
		start: 10000,
		channel: 0,
		baudrate: 9600,
		databits: 8,
		parity: Parity::None,
		stopbits: 1
	}, message.as_bytes(), &mut data);

	dump(&data);
}
