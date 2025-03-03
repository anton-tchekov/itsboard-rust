use crate::lcd::*;
use crate::font::*;
use crate::terminus16_bold::*;
use crate::terminus16::*;
use crate::sample::*;
use crate::decoder_uart::*;
use crate::decoder_spi::*;
use crate::decoder_i2c::*;
use crate::decoder_onewire::*;
use crate::decoder::*;

const BUTTON_COUNT: usize = 8;
const CHANNEL_COUNT: usize = 8;
const ICON_BOX: u32 = 30;

const COLOR_SEL: u16 = lcd_color(0, 128, 255);
const BORDER_SEL: u32 = 2;
const BORDER_DEFAULT: u32 = 1;
const TITLE_FONT: &Font = &TERMINUS16_BOLD;
const BUTTON_HEIGHT: u32 = 26;
const BUTTON_FONT: &Font = &TERMINUS16_BOLD;
const DECODER_COUNT: u32 = 4;

const TITLE_Y: u32 = ICON_BOX / 2 - TITLE_FONT.width;
const TITLE_X: u32 = TITLE_Y;
const CH_Y_BEGIN: u32 = 48;

const DA_PADDING: u32 = 10;
const Y_BEGIN: u32 = ICON_BOX + 1;
const DA_BTN_WIDTH: u32 = 100;

enum Action {
	Up,
	Down,
	Left,
	Right,
	Enter,
	Escape
}

enum Mode {
	Init,
	Main,
	DecoderAdd,
	DecoderUart,
	DecoderSpi,
	DecoderOneWire,
	DecoderI2C,
	Channels,
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

pub struct Button {
	x: u32,
	y: u32,
	w: u32,
	text: &'static str
}

impl Button {
	fn render(&self, sel: bool) {
		let text_x = self.x + self.w / 2;
		let text_y = self.y + BUTTON_HEIGHT / 2;
		lcd_str_center(text_x, text_y, self.text,
				LCD_WHITE, LCD_BLACK, &BUTTON_FONT);

		if sel { self.select(); } else { self.deselect(); }
	}

	fn undraw(&self) {
		let text_w = BUTTON_FONT.width(self.text);
		let text_h = BUTTON_FONT.height;
		let text_x = self.x + self.w / 2 - text_w / 2;
		let text_y = self.y + BUTTON_HEIGHT / 2 - text_h / 2;
		lcd_rect(text_x, text_y, text_w, text_h, LCD_BLACK);

		lcd_rect_border(self.x, self.y, self.w, BUTTON_HEIGHT,
			BORDER_SEL, LCD_BLACK);
	}

	fn select(&self) {
		lcd_rect_border(self.x, self.y, self.w, BUTTON_HEIGHT,
			BORDER_SEL, COLOR_SEL);
	}

	fn deselect(&self) {
		lcd_rect_border(self.x, self.y, self.w, BUTTON_HEIGHT,
			BORDER_DEFAULT, LCD_BLACK);
		lcd_rect_border(self.x + 1, self.y + 1, self.w - 2, BUTTON_HEIGHT - 2,
			BORDER_DEFAULT, LCD_WHITE);
	}
}

pub struct Checkbox<'a> {
	x: u32,
	y: u32,
	label: &'a str
}

impl Checkbox<'_> {
	fn render(&self, sel: bool, checked: bool) {

	}

	fn undraw(&self) {

	}

	fn select(&self) {

	}

	fn deselect(&self) {

	}

	fn check(&self) {

	}

	fn uncheck(&self) {

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
	fn render(&self) {

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
	visible_channels: u32,
	sample_offset: u32,
	pixels_per_sample: u32,
	cur_title: &'static str,
	mode: Mode,
	ma_selected: u32,
	ch_selected: u32,
	da_selected: u32,
	cd_selected: u32,
	optcnt: u32
}

impl Gui {
	fn top_divider() {
		lcd_hline(0, ICON_BOX, LCD_WIDTH, LCD_WHITE);
	}

	fn bottom_divider() {
		lcd_hline(0, LCD_HEIGHT - ICON_BOX - 1, LCD_WIDTH, LCD_WHITE);
	}

	pub fn print_info() {

	}

	pub fn init() -> Self {
		Self::top_divider();
		Self::bottom_divider();

		lcd_str(TITLE_X, LCD_HEIGHT - TITLE_Y - TERMINUS16.height,
			"Created by Joel Kypke, Haron Nazari, Anton Tchekov",
			LCD_WHITE, LCD_BLACK, &TERMINUS16);

		Self::print_info();

		let mut gui = Gui {
			icons: [KeyIcon::Disabled; 8],
			visible_channels: 0xAA55,
			sample_offset: 0,
			pixels_per_sample: 5,
			cur_title: "",
			mode: Mode::Init,
			ma_selected: 0,
			da_selected: 0,
			ch_selected: 0,
			cd_selected: 0,
			optcnt: 0
		};

		gui.title_set("ITS-Board Logic Analyzer V0.1");
		gui
	}

	fn title_set(&mut self, new_title: &'static str) {
		lcd_str(TITLE_X, TITLE_Y, new_title,
			LCD_WHITE, LCD_BLACK, TITLE_FONT);

		let len_diff = self.cur_title.len() as i32 - new_title.len() as i32;
		if len_diff > 0 {
			let x = TITLE_FONT.width(new_title);
			let w = len_diff as u32 * TITLE_FONT.width;
			lcd_rect(TITLE_X + x, TITLE_Y, w, TITLE_FONT.height, LCD_BLACK);
		}

		self.cur_title = new_title;
	}

	fn icon_box(&self) {
		lcd_rect(0, LCD_HEIGHT - ICON_BOX, LCD_WIDTH, ICON_BOX, LCD_BLACK);

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

	pub fn waveform(&self, data: &[Sample]) {

	}

	fn button_to_action(key: i32) -> Option<Action> {
		match key {
			5 => Some(Action::Up),
			3 => Some(Action::Down),
			6 => Some(Action::Left),
			2 => Some(Action::Right),
			4 => Some(Action::Enter),
			7 => Some(Action::Escape),
			_ => None
		}
	}

	pub fn key(&mut self, key: i32) {
		if let Some(action) = Self::button_to_action(key) {
			match self.mode {
				Mode::Init => {
					self.icon_box();
					self.mode_switch(Mode::Main);
				}
				Mode::Main => { self.ma_action(action); }
				Mode::Channels => { self.ch_action(action); }
				Mode::DecoderAdd => { self.da_action(action); }
				Mode::DecoderUart => { self.u_action(action); }
				Mode::DecoderSpi => { self.s_action(action); }
				Mode::DecoderI2C => { self.i_action(action); }
				Mode::DecoderOneWire => { self.o_action(action); }
				_ => {}
			};
		}
	}

	fn mode_switch(&mut self, new_mode: Mode) {
		match self.mode {
			Mode::Main => self.ma_close(),
			Mode::Channels => self.ch_close(),
			Mode::DecoderAdd => self.da_close(),
			Mode::DecoderUart => self.u_close(),
			Mode::DecoderSpi => self.s_close(),
			Mode::DecoderI2C => self.i_close(),
			Mode::DecoderOneWire => self.o_close(),
			_ => {}
		};

		self.mode = new_mode;

		match self.mode {
			Mode::Main => self.ma_open(),
			Mode::Channels => self.ch_open(),
			Mode::DecoderAdd => self.da_open(),
			Mode::DecoderUart => self.u_open(),
			Mode::DecoderSpi => self.s_open(),
			Mode::DecoderI2C => self.i_open(),
			Mode::DecoderOneWire => self.o_open(),
			_ => {}
		};
	}

	/* === CD COMMON === */
	//fn option_render(&mut self,

	fn select_render(&mut self, select: &Select) {
		let new_cnt = select.options.len() as u32;

		if self.optcnt > new_cnt {
			// Erase
			// lcd_rect();
		}
		else if self.optcnt < new_cnt {
			// Add
		}

		let mut i = 0;
		for option in select.options {
			// option_render();
		}

		self.optcnt = new_cnt;
	}

	fn input_render(&mut self, input: &Input, y: u32, sel: bool) {
		lcd_str(DA_PADDING,
			Y_BEGIN + DA_PADDING + y * 40,
			input.label,
			LCD_WHITE, LCD_BLACK, &TERMINUS16);

		lcd_rect_border(DA_PADDING, Y_BEGIN + DA_PADDING + y * 40 + 16,
			100, 20, 1, LCD_WHITE);

		lcd_str(DA_PADDING + 2,
			Y_BEGIN + DA_PADDING + y * 40 + 18,
			"aaa.ggg.111",
			LCD_WHITE, LCD_BLACK, &TERMINUS16);

		if sel {
			self.select_render(input.select);
		}
	}

	fn cd_render(&mut self, inputs: &[&Input]) {
		self.cd_selected = 0;
		let mut y = 0;
		for input in inputs {
			self.input_render(input, y, y == self.cd_selected);
			y += 1;
		}
	}

	/* === U MODE === */
	fn u_open(&mut self) {
		self.title_set("UART Decoder");
		self.cd_render(&UART_INPUTS);
	}

	fn u_close(&mut self) {

	}

	fn u_update(&mut self) {

	}

	fn u_action(&mut self, action: Action) {
		match action {
			Action::Up => {
				if self.cd_selected > 0 {
					self.cd_selected -= 1;
				}
			}
			Action::Down => {

			}
			Action::Escape => { self.mode_switch(Mode::DecoderAdd); }
			_ => {}
		};
	}

	fn u_exit(&mut self) {
		let d = DecoderUart {
			rx_pin: item_to_pin(1),
			tx_pin: item_to_pin(1),
			databits: item_to_databits(1),
			parity: item_to_parity(1),
			stopbits: item_to_stopbits(1),
			baudrate: item_to_baudrate(1)
		};
	}

	/* === S MODE === */
	fn s_open(&mut self) {
		self.title_set("SPI Decoder");
		self.cd_render(&SPI_INPUTS);
	}

	fn s_close(&mut self) {

	}

	fn s_action(&mut self, action: Action) {
		match action {
			Action::Escape => { self.mode_switch(Mode::DecoderAdd); }
			_ => {}
		}
	}

	fn s_exit(&mut self) {
		let d = DecoderSPI {
			miso_pin: item_to_pin(1),
			mosi_pin: item_to_pin(1),
			sck_pin: item_to_pin(1),
			cs_pin: item_to_pin(1)
		};
	}

	/* === I MODE === */
	fn i_open(&mut self) {
		self.title_set("I2C Decoder");
		self.cd_render(&I2C_INPUTS);
	}

	fn i_close(&mut self) {

	}

	fn i_action(&mut self, action: Action) {
		match action {
			Action::Escape => { self.mode_switch(Mode::DecoderAdd); }
			_ => {}
		}
	}

	fn i_exit(&mut self) {
		let d = DecoderI2C {
			sda_pin: item_to_pin(1),
			scl_pin: item_to_pin(1)
		};
	}

	/* === O MODE === */
	fn o_open(&mut self) {
		self.title_set("OneWire Decoder");
		self.cd_render(&ONEWIRE_INPUTS);
	}

	fn o_close(&mut self) {

	}

	fn o_action(&mut self, action: Action) {
		match action {
			Action::Escape => { self.mode_switch(Mode::DecoderAdd); }
			_ => {}
		}
	}

	fn o_exit(&mut self) {
		let d = DecoderOneWire {
			onewire_pin: item_to_pin(1)
		};
	}

	/* === MA MODE === */
	fn ma_update(&mut self, i: u32, sel: bool) {
		const ICONS: [u32; 2] = [ ICON_ADD, ICON_SETTINGS ];
		let fg = if sel { COLOR_SEL } else { LCD_WHITE };
		let x = i * (ICON_BOX + 1) + LCD_WIDTH - 2 * (ICON_BOX + 1) + 7;
		lcd_icon_color(x, 7, ICONS[i as usize], fg, LCD_BLACK);
	}

	fn ma_top_box(&mut self) {
		for i in 0..2 {
			lcd_vline(LCD_WIDTH - (i as u32 + 1) * (ICON_BOX + 1),
				0, ICON_BOX, LCD_WHITE);
			self.ma_update(i, i == self.ma_selected);
		}
	}

	fn ma_open(&mut self) {
		self.title_set("Logic Analyzer");
		self.da_selected = 0;
		self.ma_top_box();
	}

	fn ma_close(&mut self) {

	}

	fn ma_enter(&mut self) {
		match self.ma_selected {
			0 => { self.mode_switch(Mode::DecoderAdd); }
			1 => { self.mode_switch(Mode::Channels); }
			_ => {}
		}
	}

	fn ma_action(&mut self, action: Action) {
		match action {
			Action::Up => {
			}
			Action::Down => {
			}
			Action::Left => {
				if self.ma_selected > 0 {
					self.ma_update(self.ma_selected, false);
					self.ma_selected -= 1;
					self.ma_update(self.ma_selected, true);
				}
			}
			Action::Right => {
				if self.ma_selected < 1 {
					self.ma_update(self.ma_selected, false);
					self.ma_selected += 1;
					self.ma_update(self.ma_selected, true);
				}
			}
			Action::Enter => {
				self.ma_enter();
			}
			_ => {}
		}
	}

	/* === CH MODE === */
	fn check_render(&self, x: u32, y: u32, sel: bool, checked: bool) {
		let icon = if checked { ICON_CHECKED } else { ICON_UNCHECKED };
		let color = if sel { COLOR_SEL } else { LCD_WHITE };
		lcd_icon_color(x, y, icon, color, LCD_BLACK);
	}

	fn ch_update(&self, sel: bool) {
		let idx = self.ch_selected;
		let x = idx % 8;
		let y = idx / 8;
		let rx = (3 + x * 7) * TERMINUS16.width;
		let ry = CH_Y_BEGIN + y * 32;
		self.check_render(rx, ry, sel,
			self.visible_channels & (1 << idx) != 0);
	}

	fn ch_close(&mut self) {

	}

	fn ch_open(&mut self) {
		self.ch_selected = 0;
		self.ch_render();
	}

	fn ch_render(&mut self) {
		self.title_set("Visible Channels");
		for y in 0..2 {
			for x in 0..8 {
				let idx = y * 8 + x;
				let mut rx = (3 + x * 7) * TERMINUS16.width;
				let ry = CH_Y_BEGIN + y * 32;
				let mut buf: [u8; 2] = [0; 2];
				Self::channel_str(&mut buf, idx);
				self.check_render(rx, ry, self.ch_selected == idx,
					self.visible_channels & (1 << idx) != 0);

				rx += 21;
				lcd_str(rx, ry + 1, core::str::from_utf8(&buf).unwrap(),
					LCD_WHITE, LCD_BLACK, &TERMINUS16);
			}
		}
	}

	fn ch_action(&mut self, action: Action) {
		match action {
			Action::Down => {
				if self.ch_selected < 8 {
					self.ch_update(false);
					self.ch_selected += 8;
					self.ch_update(true);
				}
			},
			Action::Up => {
				if self.ch_selected >= 8 {
					self.ch_update(false);
					self.ch_selected -= 8;
					self.ch_update(true);
				}
			},
			Action::Left => {
				if self.ch_selected > 0 {
					self.ch_update(false);
					self.ch_selected -= 1;
					self.ch_update(true);
				}
			},
			Action::Right => {
				if self.ch_selected < 15 {
					self.ch_update(false);
					self.ch_selected += 1;
					self.ch_update(true);
				}
			},
			Action::Enter => {
				self.visible_channels ^= 1 << self.ch_selected;
				self.ch_update(true);
			},
			Action::Escape => {
				self.mode_switch(Mode::Main);
			},
		}
	}

	/* === DA MODE === */
	fn da_enter(&mut self) {
		match self.da_selected {
			0 => { self.mode_switch(Mode::DecoderUart); },
			1 => { self.mode_switch(Mode::DecoderSpi); },
			2 => { self.mode_switch(Mode::DecoderI2C); },
			3 => { self.mode_switch(Mode::DecoderOneWire); },
			_ => {}
		}
	}

	fn da_button(&self, idx: u32) -> Button {
		const LABELS: [&'static str; 4] = [ "UART", "SPI", "I2C", "OneWire" ];
		Button {
			x: DA_PADDING,
			y: idx * (BUTTON_HEIGHT + DA_PADDING) + ICON_BOX + 1 + DA_PADDING,
			w: DA_BTN_WIDTH,
			text: LABELS[idx as usize]
		}
	}

	fn da_open(&mut self) {
		self.title_set("Add Protocol Decoder");
		for i in 0..DECODER_COUNT {
			self.da_button(i).render(i == self.da_selected);
		}
	}

	fn da_close(&mut self) {
		for i in 0..DECODER_COUNT {
			self.da_button(i).undraw();
		}
	}

	fn da_switch(&mut self, prev: u32) {
		self.da_button(prev).deselect();
		self.da_button(self.da_selected).select();
	}

	fn da_action(&mut self, action: Action) {
		match action {
			Action::Up => {
				let prev = self.da_selected;
				if self.da_selected > 0 {
					self.da_selected -= 1;
				}
				else {
					self.da_selected = DECODER_COUNT - 1;
				}

				self.da_switch(prev);
			},
			Action::Down => {
				let prev = self.da_selected;
				if self.da_selected < DECODER_COUNT - 1 {
					self.da_selected += 1;
				}
				else {
					self.da_selected = 0;
				}

				self.da_switch(prev);
			},
			Action::Enter => {
				self.da_enter();
			},
			Action::Escape => {
				self.mode_switch(Mode::Main);
			},
			_ => {}
		};
	}

	/* === Helper === */
	fn channel_str(out: &mut [u8], channel: u32) {
		out[0] = (channel / 10) as u8 + b'0';
		out[1] = (channel % 10) as u8 + b'0';
	}
}
