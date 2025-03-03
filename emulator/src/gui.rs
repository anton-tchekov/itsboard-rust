use crate::lcd::*;
use crate::font::*;
use crate::terminus16_bold::*;
use crate::terminus16::*;
use crate::sample::*;
use crate::decoder_uart::*;
use crate::decoder::*;

const BUTTON_COUNT: usize = 8;
const CHANNEL_COUNT: usize = 8;
const ICON_BOX: u32 = 30;

// >>> Main Page: View Waveforms
// Controls: Start capture
// Up down left right
// Zoom in, Zoom out
// Settings
// Screenshot

// >>> Settings Page:
// Controls:
// Enable/Disable Channels via checkboxes
// Add protocol decoder
// Save capture
// Load capture
//
// Actions:
// Up down left right
// Enter
// Exit

// >>> Add protocol decoder
// List of all Protocol decoders + Cancel

// Add concrete protocol decoder
// Select pins: dropdown
// select baudrate etc: dropdown
// cancel and add button

// Save capture
// on screen Keyboard
// textbox for filename
// save and cancel
// move cursor + backspace

// Load capture
// cancel on top
// List of files: enter to select

enum Window {
	Main,
	AddDecoder,
	LoadCapture,
	SaveCapture,
}

#[derive(Copy, Clone)]
enum KeyIcon {
	Up,
	Down,
	Left,
	Right,
	Enter,
	Exit,
	Settings,
	Disabled
}

const BUTTON_HEIGHT: u32 = 30;

pub struct Button {
	x: u32,
	y: u32,
	w: u32,
	text: &'static str
}

impl Button {
	fn button_render(&self, sel: bool) {
		let color: u16 = LCD_WHITE;
		if sel {

		}
	}
}

enum Align {
	Left,
	Center,
	Right
}

pub struct Select {
	align: Align,
	options: &'static [&'static str]
}

pub struct Keyboard {
	x: u32,
	y: u32
}

pub struct Input {
	select: &'static Select,
	label: &'static str
}

impl Input {
	fn render() {

	}
}

const SELECT_PIN_LIST: [&'static str; 9] = [
	"/", "0", "1", "2", "3", "4", "5", "6", "7"
];

const SELECT_PARITY_LIST: [&'static str; 3] = [
	"None", "Even", "Odd"
];

const SELECT_STOP_BITS_LIST: [&'static str; 3] = [
	"1", "1.5", "2"
];

const SELECT_DATA_BITS_LIST: [&'static str; 5] = [
	"5", "6", "7", "8", "9"
];

const SELECT_BAUDRATE_LIST: [&'static str; 11] = [
	"300",
	"600",
	"1200",
	"1800",
	"2400",
	"4800",
	"9600",
	"19200",
	"38400",
	"57600",
	"115200"
];

static BAUDRATES: [u32; 11] = [
	300,
	600,
	1200,
	1800,
	2400,
	4800,
	9600,
	19200,
	38400,
	57600,
	115200
];

const SELECT_PIN: Select = Select {
	align: Align::Right,
	options: &SELECT_PIN_LIST
};

const SELECT_BAUDRATE: Select = Select {
	align: Align::Right,
	options: &SELECT_BAUDRATE_LIST
};

const SELECT_DATA_BITS: Select = Select {
	align: Align::Right,
	options: &SELECT_DATA_BITS_LIST
};

const SELECT_PARITY: Select = Select {
	align: Align::Left,
	options: &SELECT_PARITY_LIST
};

const SELECT_STOP_BITS: Select = Select {
	align: Align::Right,
	options: &SELECT_STOP_BITS_LIST
};

/* UART */
const UART_RX: Input = Input {
	select: &SELECT_PIN,
	label: "RX Pin"
};

const UART_TX: Input = Input {
	select: &SELECT_PIN,
	label: "TX Pin"
};

const UART_BAUDRATE: Input = Input {
	select: &SELECT_BAUDRATE,
	label: "Baudrate"
};

const UART_DATABITS: Input = Input {
	select: &SELECT_DATA_BITS,
	label: "Data Bits"
};

const UART_PARITY: Input = Input {
	select: &SELECT_PARITY,
	label: "Parity"
};

const UART_STOPBITS: Input = Input {
	select: &SELECT_STOP_BITS,
	label: "Stop Bits"
};

const UART_INPUTS: [&Input; 6] = [
	&UART_RX,
	&UART_TX,
	&UART_BAUDRATE,
	&UART_DATABITS,
	&UART_PARITY,
	&UART_STOPBITS
];

/* SPI */
const SPI_MISO: Input = Input {
	select: &SELECT_PIN,
	label: "MISO Pin"
};

const SPI_MOSI: Input = Input {
	select: &SELECT_PIN,
	label: "MOSI Pin"
};

const SPI_SCK: Input = Input {
	select: &SELECT_PIN,
	label: "SCK Pin"
};

const SPI_CS: Input = Input {
	select: &SELECT_PIN,
	label: "CS Pin"
};

const SPI_INPUTS: [&Input; 4] = [
	&SPI_MOSI,
	&SPI_MISO,
	&SPI_SCK,
	&SPI_CS
];

/* I2C */
const I2C_SDA: Input = Input {
	select: &SELECT_PIN,
	label: "SDA Pin"
};

const I2C_SCL: Input = Input {
	select: &SELECT_PIN,
	label: "SCL Pin"
};

const I2C_INPUTS: [&Input; 2] = [
	&I2C_SDA,
	&I2C_SCL
];

/* OneWire */
const ONEWIRE_PIN: Input = Input {
	select: &SELECT_PIN,
	label: "OneWire Pin"
};

const ONEWIRE_INPUTS: [&Input; 1] = [
	&ONEWIRE_PIN
];

/* Get value */
fn item_to_baudrate(idx: usize) -> u32 {
	BAUDRATES[idx]
}

fn item_to_pin(idx: usize) -> DecoderPin {
	(idx as i32) - 1
}

fn item_to_databits(idx: usize) -> DataBits {
	match idx {
		0 => DataBits::Five,
		1 => DataBits::Six,
		2 => DataBits::Seven,
		4 => DataBits::Nine,
		_ => DataBits::Eight
	}
}

fn item_to_parity(idx: usize) -> ParitySetting {
	match idx {
		1 => ParitySetting::Even,
		2 => ParitySetting::Odd,
		_ => ParitySetting::None
	}
}

fn item_to_stopbits(idx: usize) -> StopBits {
	match idx {
		1 => StopBits::OneAndHalf,
		2 => StopBits::Two,
		_ => StopBits::One
	}
}

pub struct Gui {
	icons: [KeyIcon; BUTTON_COUNT],
	visible_channels: Sample,
	sample_offset: u32,
	pixels_per_sample: u32
}

impl Gui {
	pub fn init() -> Self {
		lcd_str(5, 5, "ITS-Board Logic Analyzer V0.1",
			LCD_WHITE, LCD_BLACK, &TERMINUS16_BOLD);



		lcd_str(5, 299, "Created by Joel Kypke, Haron Nazari, Anton Tchekov",
			LCD_WHITE, LCD_BLACK, &TERMINUS16);

		Gui {
			icons: [KeyIcon::Disabled; 8],
			visible_channels: 0xFF,
			sample_offset: 0,
			pixels_per_sample: 5
		}
	}

	fn title_bar(&self) {
		lcd_hline(0, ICON_BOX, LCD_WIDTH, LCD_WHITE);
		lcd_str(5, 5, "Add Protocol Decoder",
			LCD_WHITE, LCD_BLACK, &TERMINUS16);
	}

	fn icon_box(&self) {
		lcd_hline(0, LCD_HEIGHT - ICON_BOX - 1, LCD_WIDTH, LCD_WHITE);
		for i in 0..BUTTON_COUNT {
			lcd_vline(LCD_WIDTH - (i as u32 + 1) * (ICON_BOX + 1),
				LCD_HEIGHT - ICON_BOX, ICON_BOX, LCD_WHITE);
		}

		let mut x = LCD_WIDTH - 8 * (ICON_BOX + 1) + 7;
		let y = LCD_HEIGHT - ICON_BOX + 7;
		lcd_icon_bw(x, y, ICON_EXIT);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_LEFT);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_UP);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_ENTER);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_DOWN);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_RIGHT);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_SCREENSHOT);
		x += ICON_BOX + 1;
		lcd_icon_bw(x, y, ICON_FOLDER);
	}

	pub fn base(&self) {
		lcd_clear(LCD_BLACK);
		self.icon_box();
	}

	pub fn waveform(&self, data: &[Sample]) {

	}

	fn keyicon_render(icon: &KeyIcon) {

	}

	fn keybind_render() {

	}

	fn keybinds_set() {

	}

	fn screenshot() {

	}

	fn handle_key(key: i32) {
		// key to action

		// Perform action in current context
	}

	fn checkbox_render(checked: bool) {
		if checked {

		}
		else {

		}
	}
}

fn channel_str(channel: u32, out: &mut [u8]) {
	out[0] = b'D';
	out[1] = (channel / 10) as u8 + b'0';
	out[2] = (channel % 10) as u8 + b'0';
}

fn channel_render() {
}
