use crate::lcd::*;
use crate::font::*;
use crate::terminus16_bold::*;
use crate::terminus16::*;
use crate::decoder_uart::*;
use crate::decoder_spi::*;
use crate::decoder_i2c::*;
use crate::decoder_onewire::*;
use crate::decoder::*;
use crate::sample;
use crate::sampler;
use crate::sample::SampleBuffer;
use core::str;

const BUTTON_COUNT: usize = 8;
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

const CH_ROWS: u32 = 2;
const CH_COLS: u32 = 8;
const CH_LABEL_X: u32 = 21;
const CH_LABEL_Y: u32 = 1;

const MA_ICONS: u32 = 4;
const ICON_PADDING: u32 = 7;

const ACTION_ICONS_SKIP: u32 = ICON_BOX + 1;
const ACTION_ICONS_X: u32 = LCD_WIDTH - 8 * (ICON_BOX + 1) + ICON_PADDING;
const ACTION_ICONS_Y: u32 = LCD_HEIGHT - ICON_BOX + ICON_PADDING;

const INPUT_Y_SKIP: u32 = 40;
const INPUT_WIDTH: u32 = 100;
const INPUT_HEIGHT: u32 = 20;

const INPUT_LABEL_Y: u32 = Y_BEGIN + DA_PADDING;
const INPUT_BOX_Y: u32 = Y_BEGIN + DA_PADDING + 16;

const INPUT_TEXT_Y: u32 = Y_BEGIN + DA_PADDING + 18;
const TERM_Y: u32 = 40;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Action {
	None,
	Up,
	Down,
	Left,
	Right,
	Enter,
	Escape,
	Check,
	ZoomIn,
	ZoomOut,
	Stop,
	Screenshot
}

enum Mode {
	Main,
	Info,
	DecoderAdd,
	DecoderUart,
	DecoderSpi,
	DecoderOneWire,
	DecoderI2C,
	Channels
}

fn boxsel(x: u32, y: u32, w: u32, h: u32) {
	lcd_rect_border(x, y, w, h, BORDER_SEL, COLOR_SEL);
}

fn boxdesel(x: u32, y: u32, w: u32, h: u32) {
	lcd_rect_border(x, y, w, h, BORDER_DEFAULT, LCD_BLACK);
	lcd_rect_border(x + 1, y + 1, w - 2, h - 2, BORDER_DEFAULT, LCD_WHITE);
}

fn boxundraw(x: u32, y: u32, w: u32, h: u32) {
	lcd_rect_border(x, y, w, h, BORDER_SEL, LCD_BLACK);
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
				LCD_WHITE, LCD_BLACK, BUTTON_FONT);

		if sel { self.select(); } else { self.deselect(); }
	}

	fn undraw(&self) {
		let text_w = BUTTON_FONT.width(self.text);
		let text_h = BUTTON_FONT.height;
		let text_x = self.x + self.w / 2 - text_w / 2;
		let text_y = self.y + BUTTON_HEIGHT / 2 - text_h / 2;
		lcd_rect(text_x, text_y, text_w, text_h, LCD_BLACK);
		boxundraw(self.x, self.y, self.w, BUTTON_HEIGHT);
	}

	fn select(&self) {
		boxsel(self.x, self.y, self.w, BUTTON_HEIGHT);
	}

	fn deselect(&self) {
		boxdesel(self.x, self.y, self.w, BUTTON_HEIGHT);
	}
}

fn input_select(y: u32) {
	boxsel(DA_PADDING, INPUT_BOX_Y + y * INPUT_Y_SKIP,
		INPUT_WIDTH, INPUT_HEIGHT);
}

fn input_deselect(y: u32) {
	boxdesel(DA_PADDING, INPUT_BOX_Y + y * INPUT_Y_SKIP,
		INPUT_WIDTH, INPUT_HEIGHT);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Align {
	Left,
	Right
}

pub struct Select {
	align: Align,
	options: &'static [&'static str]
}

pub struct Input {
	select: &'static Select,
	label: &'static str
}

const SELECT_PIN_LIST: [&str; 17] = [
	"/", "0", "1", "2", "3", "4", "5", "6", "7",
	"8", "9", "10", "11", "12", "13", "14", "15"
];

const SELECT_PARITY_LIST: [&str; 3] = [
	"None", "Even", "Odd"
];

const SELECT_STOP_BITS_LIST: [&str; 3] = [
	"1", "1.5", "2"
];

const SELECT_DATA_BITS_LIST: [&str; 5] = [
	"8", "9", "5", "6", "7"
];

const SELECT_BAUDRATE_LIST: [&str; 11] = [
	"9600",
	"19200",
	"38400",
	"57600",
	"115200",
	"300",
	"600",
	"1200",
	"1800",
	"4800",
	"2400",
];

static BAUDRATES: [u32; 11] = [
	9600,
	19200,
	38400,
	57600,
	115200,
	300,
	600,
	1200,
	1800,
	2400,
	4800
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
		2 => DataBits::Five,
		3 => DataBits::Six,
		4 => DataBits::Seven,
		1 => DataBits::Nine,
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

fn cycle_fwd(idx: u32, count: u32) -> u32 {
	if idx == count - 1 { 0 } else { idx + 1 }
}

fn cycle_bwd(idx: u32, count: u32) -> u32 {
	if idx == 0 { count - 1 } else { idx - 1 }
}

const ACTIONS_INFO: [Action; 8] = [
	Action::None, Action::None, Action::None, Action::None,
	Action::None, Action::None, Action::None, Action::Enter
];

const ACTIONS_SAMPLING: [Action; 8] = [
	Action::Stop, Action::None, Action::None, Action::None,
	Action::None, Action::None, Action::None, Action::None
];

const ACTIONS_MAIN: [Action; 8] = [
	Action::Up, Action::Down, Action::Left, Action::Right,
	Action::ZoomIn, Action::ZoomOut, Action::Screenshot, Action::Enter
];

const ACTIONS_DA: [Action; 8] = [
	Action::Up, Action::Down, Action::None, Action::None,
	Action::None, Action::None, Action::Escape, Action::Enter
];

const ACTIONS_CH: [Action; 8] = [
	Action::Up, Action::Down, Action::Left, Action::Right,
	Action::None, Action::None, Action::Escape, Action::Enter
];

const ACTIONS_CD: [Action; 8] = [
	Action::Up, Action::Down, Action::Left, Action::Right,
	Action::None, Action::None, Action::Escape, Action::Check
];

pub struct Gui {
	actions: &'static [Action],
	visible_channels: u32,
	cur_title: &'static str,
	mode: Mode,
	ma_selected: u32,
	ch_selected: u32,
	da_selected: u32,
	cd_selected: u32,
	sels: [u8; 8],
	inputs: &'static [&'static Input],
	term_rows: u32,
	term_lens: [u8; 16],
	buf: SampleBuffer
}

impl Gui {
	fn top_divider() {
		lcd_hline(0, ICON_BOX, LCD_WIDTH, LCD_WHITE);
	}

	fn bottom_divider() {
		lcd_hline(0, LCD_HEIGHT - ICON_BOX - 1, LCD_WIDTH, LCD_WHITE);
	}

	pub fn term_print(&mut self, s: &str) {
		lcd_str(TITLE_X, TERM_Y + self.term_rows * TERMINUS16.height,
			s, LCD_WHITE, LCD_BLACK, &TERMINUS16);

		self.term_lens[self.term_rows as usize] = s.len() as u8;
		self.term_rows += 1;
	}

	pub fn term_undraw(&mut self) {
		for i in 0..self.term_rows {
			lcd_rect(TITLE_X, TERM_Y + i * TERMINUS16.height,
				self.term_lens[i as usize] as u32 * TERMINUS16.width,
				TERMINUS16.height, LCD_BLACK);
		}

		self.term_rows = 0;
	}

	pub fn init() -> Self {
		Self::top_divider();
		Self::bottom_divider();

		let mut gui = Gui {
			actions: &ACTIONS_MAIN,
			visible_channels: 0xAA55,
			cur_title: "",
			mode: Mode::Main,
			ma_selected: 0,
			da_selected: 0,
			ch_selected: 0,
			cd_selected: 0,
			sels: [0; 8],
			inputs: &UART_INPUTS,
			term_rows: 0,
			term_lens: [0; 16],
			buf: SampleBuffer {
				samples: [0; sample::BUF_SIZE],
				timestamps: [0; sample::BUF_SIZE],
				len: 0
			}
		};

		gui.icon_box();
		gui.actions_render();
		gui.ma_open();
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
	}

	fn action_icon_render(x: u32, y: u32, action: Action) {
		match action {
			Action::Left => lcd_icon_bw(x, y, ICON_LEFT),
			Action::Right => lcd_icon_bw(x, y, ICON_RIGHT),
			Action::Up => lcd_icon_bw(x, y, ICON_UP),
			Action::Down => lcd_icon_bw(x, y, ICON_DOWN),
			Action::Enter => lcd_icon_bw(x, y, ICON_ENTER),
			Action::Escape => lcd_icon_bw(x, y, ICON_EXIT),
			Action::Check => lcd_icon_bw(x, y, ICON_CHECK),
			Action::Screenshot => lcd_icon_bw(x, y, ICON_SCREENSHOT),
			Action::ZoomIn => lcd_icon_bw(x, y, ICON_EXPAND),
			Action::ZoomOut => lcd_icon_bw(x, y, ICON_SHRINK),
			Action::Stop => lcd_icon_bw(x, y, ICON_STOP),
			_ => lcd_icon_undraw(x, y)
		}
	}

	fn actions_set(&mut self, a: &'static [Action]) {
		let mut x = ACTION_ICONS_X;
		for i in 0..BUTTON_COUNT {
			if a[i] != self.actions[i] {
				Self::action_icon_render(x, ACTION_ICONS_Y, a[i]);
			}

			x += ACTION_ICONS_SKIP;
		}

		self.actions = a;
	}

	fn actions_render(&self) {
		let mut x = ACTION_ICONS_X;
		for i in 0..BUTTON_COUNT {
			if self.actions[i] != Action::None {
				Self::action_icon_render(x, ACTION_ICONS_Y, self.actions[i]);
			}

			x += ACTION_ICONS_SKIP;
		}
	}

	fn button_to_action(&self, key: i32) -> Action {
		self.actions[(BUTTON_COUNT - 1) - key as usize]
	}

	pub fn action(&mut self, action: Action) {
		match self.mode {
			Mode::Info => { self.info_action(action); }
			Mode::Main => { self.ma_action(action); }
			Mode::Channels => { self.ch_action(action); }
			Mode::DecoderAdd => { self.da_action(action); }
			Mode::DecoderUart => { self.u_action(action); }
			Mode::DecoderSpi => { self.s_action(action); }
			Mode::DecoderI2C => { self.i_action(action); }
			Mode::DecoderOneWire => { self.o_action(action); }
		};
	}

	pub fn key(&mut self, key: i32) {
		self.action(self.button_to_action(key));
	}

	fn mode_switch(&mut self, new_mode: Mode) {
		match self.mode {
			Mode::Main => self.ma_close(),
			Mode::Channels => self.ch_close(),
			Mode::DecoderAdd => self.da_close(),
			Mode::DecoderUart => self.cd_undraw(),
			Mode::DecoderSpi => self.cd_undraw(),
			Mode::DecoderI2C => self.cd_undraw(),
			Mode::DecoderOneWire => self.cd_undraw(),
			Mode::Info => self.info_close()
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
			Mode::Info => self.info_open()
		};
	}

	/* === INFO === */
	fn info_action(&mut self, action: Action) {
		if action == Action::Enter {
			self.mode_switch(Mode::Main);
		}
	}

	fn info_close(&mut self) {
		self.term_undraw();
	}

	fn info_open(&mut self) {
		self.actions_set(&ACTIONS_INFO);
		self.title_set("About");
		self.term_print("ITS-Board Logic Analyzer V0.1");
		self.term_print("Created by Joel Kypke, Haron Nazari, Anton Tchekov");
		self.term_print("");
		self.term_print("Press any key to continue ...");
	}

	/* === CD COMMON === */
	fn cd_up(&mut self) {
		let prev = self.cd_selected;
		self.cd_selected = cycle_bwd(self.cd_selected, self.inputs.len() as u32);
		self.cd_update(prev);
	}

	fn cd_down(&mut self) {
		let prev = self.cd_selected;
		self.cd_selected = cycle_fwd(self.cd_selected, self.inputs.len() as u32);
		self.cd_update(prev);
	}

	fn cur_num_options(&self) -> u32 {
		self.inputs[self.cd_selected as usize].select.options.len() as u32
	}

	fn cd_left(&mut self) {
		let idx = self.cd_selected as usize;
		let prev = self.sels[idx];
		self.sels[idx] = cycle_bwd(self.sels[idx].into(), self.cur_num_options()) as u8;
		self.cd_sel_update(prev);
	}

	fn cd_right(&mut self) {
		let idx = self.cd_selected as usize;
		let prev = self.sels[idx];
		self.sels[idx] = cycle_fwd(self.sels[idx].into(), self.cur_num_options()) as u8;
		self.cd_sel_update(prev);
	}

	fn cd_sel_update(&mut self, prev_idx: u8) {
		let y = self.cd_selected;
		let input = self.inputs[y as usize];
		let select = input.select;
		let align = select.align;
		let options = select.options;
		let cur_idx = self.sels[y as usize];
		let prev_text = options[prev_idx as usize];
		let text = options[cur_idx as usize];
		let w = TERMINUS16.width(text);
		let x = Self::input_text_x(align, w);
		let ry = INPUT_TEXT_Y + y * INPUT_Y_SKIP;

		let wdiff = TERMINUS16.width(prev_text) as i32 - w as i32;
		if wdiff > 0 {
			let udiff = wdiff as u32;
			let rx = if align == Align::Right { x - udiff } else { x + w };
			lcd_rect(rx, ry, udiff, TERMINUS16.height, LCD_BLACK);
		}

		lcd_str(x, ry, text, LCD_WHITE, LCD_BLACK, &TERMINUS16);
	}

	fn cd_update(&mut self, prev: u32) {
		input_deselect(prev);
		input_select(self.cd_selected);
	}

	fn input_text_x(align: Align, w: u32) -> u32 {
		DA_PADDING + if align == Align::Right { INPUT_WIDTH - 2 - w } else { 2 }
	}

	fn input_undraw(&mut self, input: &Input, y: u32) {
		// Undraw Label
		lcd_rect(DA_PADDING, Y_BEGIN + DA_PADDING + y * INPUT_Y_SKIP,
			TERMINUS16.width(input.label), TERMINUS16.height, LCD_BLACK);

		// Undraw Box
		boxundraw(DA_PADDING, INPUT_BOX_Y + y * INPUT_Y_SKIP,
			INPUT_WIDTH, INPUT_HEIGHT);

		// Undraw Content
		let text = input.select.options[self.sels[y as usize] as usize];
		let w = TERMINUS16.width(text);
		let y = INPUT_TEXT_Y + y * INPUT_Y_SKIP;
		let x = Self::input_text_x(input.select.align, w);
		lcd_rect(x, y, w, TERMINUS16.height, LCD_BLACK);
	}

	fn input_render(&mut self, input: &Input, y: u32) {
		lcd_str(DA_PADDING, INPUT_LABEL_Y + y * INPUT_Y_SKIP,
			input.label, LCD_WHITE, LCD_BLACK, &TERMINUS16);

		if y == self.cd_selected {
			input_select(y);
		}
		else {
			input_deselect(y);
		}

		let text = input.select.options[self.sels[y as usize] as usize];
		lcd_str(Self::input_text_x(input.select.align, TERMINUS16.width(text)),
			INPUT_TEXT_Y + y * INPUT_Y_SKIP,
			text, LCD_WHITE, LCD_BLACK, &TERMINUS16);
	}

	fn cd_undraw(&mut self) {
		let mut y = 0;
		for input in self.inputs {
			self.input_undraw(input, y);
			y += 1;
		}
	}

	fn cd_render(&mut self, inputs: &'static [&Input]) {
		self.cd_selected = 0;
		self.inputs = inputs;
		self.actions_set(&ACTIONS_CD);
		let mut y = 0;
		for input in inputs {
			self.sels[y] = 0;
			self.input_render(input, y as u32);
			y += 1;
		}
	}

	fn cd_action(&mut self, action: Action) {
		match action {
			Action::Up => self.cd_up(),
			Action::Down => self.cd_down(),
			Action::Left => self.cd_left(),
			Action::Right => self.cd_right(),
			Action::Escape => self.mode_switch(Mode::DecoderAdd),
			_ => {}
		}
	}

	/* === UART (U) MODE === */
	fn u_open(&mut self) {
		self.title_set("UART Decoder");
		self.cd_render(&UART_INPUTS);
	}

	fn u_action(&mut self, action: Action) {
		match action {
			Action::Check => self.u_save(),
			_ => self.cd_action(action)
		};
	}

	fn u_save(&mut self) {
		let d = DecoderUart {
			rx_pin: item_to_pin(self.sels[0].into()),
			tx_pin: item_to_pin(self.sels[1].into()),
			databits: item_to_databits(self.sels[2].into()),
			parity: item_to_parity(self.sels[3].into()),
			stopbits: item_to_stopbits(self.sels[4].into()),
			baudrate: item_to_baudrate(self.sels[5].into())
		};

		// TODO: Store Decoder
	}

	/* === SPI (S) MODE === */
	fn s_open(&mut self) {
		self.title_set("SPI Decoder");
		self.cd_render(&SPI_INPUTS);
	}

	fn s_action(&mut self, action: Action) {
		match action {
			Action::Check => self.s_save(),
			_ => self.cd_action(action)
		};
	}

	fn s_save(&mut self) {
		let d = DecoderSPI {
			miso_pin: item_to_pin(self.sels[0].into()),
			mosi_pin: item_to_pin(self.sels[1].into()),
			sck_pin: item_to_pin(self.sels[2].into()),
			cs_pin: item_to_pin(self.sels[3].into())
		};

		// TODO: Store Decoder
	}

	/* === I2C (I) MODE === */
	fn i_open(&mut self) {
		self.title_set("I2C Decoder");
		self.cd_render(&I2C_INPUTS);
	}

	fn i_action(&mut self, action: Action) {
		match action {
			Action::Check => self.i_save(),
			_ => self.cd_action(action)
		};
	}

	fn i_save(&mut self) {
		let d = DecoderI2C {
			sda_pin: item_to_pin(self.sels[0].into()),
			scl_pin: item_to_pin(self.sels[1].into())
		};

		// TODO: Store Decoder
	}

	/* === ONEWIRE (O) MODE === */
	fn o_open(&mut self) {
		self.title_set("OneWire Decoder");
		self.cd_render(&ONEWIRE_INPUTS);
	}

	fn o_action(&mut self, action: Action) {
		match action {
			Action::Check => self.o_save(),
			_ => self.cd_action(action)
		};
	}

	fn o_save(&mut self) {
		let d = DecoderOneWire {
			onewire_pin: item_to_pin(self.sels[0].into())
		};

		// TODO: Store Decoder
	}

	/* === MAIN (MA) MODE === */
	fn ma_render(&mut self, i: u32, sel: bool) {
		const ICONS: [u32; MA_ICONS as usize] = [ ICON_START, ICON_ADD, ICON_SETTINGS, ICON_INFO ];
		let fg = if sel { COLOR_SEL } else { LCD_WHITE };
		let x = LCD_WIDTH - (MA_ICONS - i) * (ICON_BOX + 1) + ICON_PADDING;
		lcd_icon_color(x, ICON_PADDING, ICONS[i as usize], fg, LCD_BLACK);
	}

	fn ma_top_box(&mut self) {
		for i in 0..MA_ICONS {
			lcd_vline(LCD_WIDTH - (i + 1) * (ICON_BOX + 1),
				0, ICON_BOX, LCD_WHITE);
			self.ma_render(i, i == self.ma_selected);
		}
	}

	fn ma_update(&mut self, prev: u32) {
		self.ma_render(prev, false);
		self.ma_render(self.ma_selected, true);
	}

	fn ma_open(&mut self) {
		self.title_set("Logic Analyzer");
		self.actions_set(&ACTIONS_MAIN);
		self.da_selected = 0;
		self.ma_top_box();
	}

	fn ma_close(&mut self) {
		for i in 0..MA_ICONS {
			lcd_vline(LCD_WIDTH - (i + 1) * (ICON_BOX + 1),
				0, ICON_BOX, LCD_BLACK);
			lcd_icon_undraw(
				LCD_WIDTH - (MA_ICONS - i) * (ICON_BOX + 1) + ICON_PADDING,
				ICON_PADDING);
		}
	}

	fn ma_running(&mut self) {
		lcd_icon_color(4, ACTION_ICONS_Y, ICON_DOT, LCD_GREEN, LCD_BLACK);
		lcd_str(26, ACTION_ICONS_Y + 1, "RUNNING",
			LCD_WHITE, LCD_BLACK, &TERMINUS16_BOLD);
	}

	fn ma_running_undraw(&mut self) {
		lcd_rect(4, ACTION_ICONS_Y,
			16 + 6 + TERMINUS16_BOLD.width("RUNNING"), 16, LCD_BLACK);
	}

	fn ma_run(&mut self) {
		self.ma_running();
		self.actions_set(&ACTIONS_SAMPLING);
		sampler::sample_blocking(&mut self.buf);
		self.actions_set(&ACTIONS_MAIN);
		self.ma_running_undraw();
	}

	fn ma_enter(&mut self) {
		match self.ma_selected {
			0 => { self.ma_run(); }
			1 => { self.mode_switch(Mode::DecoderAdd); }
			2 => { self.mode_switch(Mode::Channels); }
			3 => { self.mode_switch(Mode::Info); }
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
				let prev = self.ma_selected;
				self.ma_selected = cycle_bwd(self.ma_selected, MA_ICONS);
				self.ma_update(prev);
			}
			Action::Right => {
				let prev = self.ma_selected;
				self.ma_selected = cycle_fwd(self.ma_selected, MA_ICONS);
				self.ma_update(prev);
			}
			Action::Enter => {
				self.ma_enter();
			}
			_ => {}
		}
	}

	/* === CHANNEL (CH) MODE === */
	fn check_render(&self, x: u32, y: u32, sel: bool, checked: bool) {
		let icon = if checked { ICON_CHECKED } else { ICON_UNCHECKED };
		let color = if sel { COLOR_SEL } else { LCD_WHITE };
		lcd_icon_color(x, y, icon, color, LCD_BLACK);
	}

	fn ch_pos(x: u32, y: u32) -> (u32, u32) {
		let rx = (3 + x * 7) * TERMINUS16.width;
		let ry = CH_Y_BEGIN + y * 32;
		(rx, ry)
	}

	fn ch_update(&self, sel: bool) {
		let idx = self.ch_selected;
		let x = idx % CH_COLS;
		let y = idx / CH_COLS;
		let (rx, ry) = Self::ch_pos(x, y);
		self.check_render(rx, ry, sel,
			self.visible_channels & (1 << idx) != 0);
	}

	fn ch_close(&mut self) {
		let w = 2 * TERMINUS16.width;
		let h = TERMINUS16.height;
		for y in 0..CH_ROWS {
			for x in 0..CH_COLS {
				let (rx, ry) = Self::ch_pos(x, y);
				lcd_icon_undraw(rx, ry);
				lcd_rect(rx + CH_LABEL_X, ry + CH_LABEL_Y, w, h, LCD_BLACK);
			}
		}
	}

	fn ch_open(&mut self) {
		self.ch_selected = 0;
		self.title_set("Visible Channels");
		self.actions_set(&ACTIONS_CH);
		for y in 0..CH_ROWS {
			for x in 0..CH_COLS {
				let idx = y * CH_COLS + x;
				let (rx, ry) = Self::ch_pos(x, y);
				let mut buf: [u8; 2] = [0; 2];
				Self::channel_str(&mut buf, idx);
				self.check_render(rx, ry, self.ch_selected == idx,
					self.visible_channels & (1 << idx) != 0);

				lcd_str(rx + CH_LABEL_X, ry + CH_LABEL_Y,
					core::str::from_utf8(&buf).unwrap(),
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
			_ => {}
		}
	}

	/* === DECODER ADD (DA) MODE === */
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
		const LABELS: [&str; DECODER_COUNT as usize] = [ "UART", "SPI", "I2C", "OneWire" ];
		Button {
			x: DA_PADDING,
			y: idx * (BUTTON_HEIGHT + DA_PADDING) + ICON_BOX + 1 + DA_PADDING,
			w: DA_BTN_WIDTH,
			text: LABELS[idx as usize]
		}
	}

	fn da_open(&mut self) {
		self.title_set("Add Protocol Decoder");
		self.actions_set(&ACTIONS_DA);
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
				self.da_selected = cycle_bwd(self.da_selected, DECODER_COUNT);
				self.da_switch(prev);
			},
			Action::Down => {
				let prev = self.da_selected;
				self.da_selected = cycle_fwd(self.da_selected, DECODER_COUNT);
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
