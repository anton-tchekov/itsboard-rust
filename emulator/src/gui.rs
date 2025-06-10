use crate::decoder;
use crate::decoder_framebuffer::DecoderFrameBuffer;
use crate::delay::delay_ms;
use crate::hw::HW;
use crate::lcd::*;
use crate::font::*;
use crate::terminus16_bold::*;
use crate::terminus16::*;
use crate::tinyfont::*;
use crate::decoder_uart::*;
use crate::decoder_spi::*;
use crate::decoder_i2c::*;
use crate::decoder_onewire::*;
use crate::decoder::*;
use crate::sampler;
use crate::sample::SampleBuffer;
use core::str;
use core::fmt::Write;
use crate::bytewriter::ByteMutWriter;
use crate::hw;
use crate::positionindicator::PositionIndicator;
use crate::waveform::*;
use crate::decoder_storage::{DecoderUnion, DecoderStorage};

const BUTTON_COUNT: usize = 8;
const ICON_BOX: u32 = 30;

const COLOR_SEL: u16 = lcd_color(0, 128, 255);
const BORDER_SEL: u32 = 2;
const BORDER_DEFAULT: u32 = 1;
const TITLE_FONT: &Font = &TERMINUS16_BOLD;
const BUTTON_HEIGHT: u32 = 26;
const BUTTON_FONT: &Font = &TERMINUS16_BOLD;
const DECODER_COUNT: u32 = 5;

const MA_BOTTOM_TEXT_X: u32 = 26;

const TITLE_Y: u32 = ICON_BOX / 2 - TITLE_FONT.width;
const TITLE_X: u32 = TITLE_Y;

const DA_PADDING: u32 = 10;
const Y_BEGIN: u32 = ICON_BOX + 1;
const DA_BTN_WIDTH: u32 = 100;

const MA_ICONS: u32 = 3;
const ICON_PADDING: u32 = 7;

const ACTION_ICONS_SKIP: u32 = ICON_BOX + 1;
const ACTION_ICONS_X: u32 = LCD_WIDTH - 8 * (ICON_BOX + 1) + ICON_PADDING;
const ACTION_ICONS_Y: u32 = LCD_HEIGHT - ICON_BOX + ICON_PADDING;

const INPUT_Y_SKIP: u32 = 40;
const INPUT_WIDTH: u32 = 120;
const INPUT_HEIGHT: u32 = 20;

const INPUT_LABEL_Y: u32 = Y_BEGIN + DA_PADDING;
const INPUT_BOX_Y: u32 = Y_BEGIN + DA_PADDING + 16;

const INPUT_TEXT_Y: u32 = Y_BEGIN + DA_PADDING + 18;
const TERM_Y: u32 = 40;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Action
{
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
	Cycle,
	Stop
}

enum Mode
{
	Main,
	Info,
	DecoderAdd,
	DecoderUart,
	DecoderSpi,
	DecoderOneWire,
	DecoderI2C
}

fn boxsel(x: u32, y: u32, w: u32, h: u32)
{
	lcd_rect_border(x, y, w, h, BORDER_SEL, COLOR_SEL);
}

fn boxdesel(x: u32, y: u32, w: u32, h: u32)
{
	lcd_rect_border(x, y, w, h, BORDER_DEFAULT, LCD_BLACK);
	lcd_rect_border(x + 1, y + 1, w - 2, h - 2, BORDER_DEFAULT, LCD_WHITE);
}

fn boxundraw(x: u32, y: u32, w: u32, h: u32)
{
	lcd_rect_border(x, y, w, h, BORDER_SEL, LCD_BLACK);
}

pub struct Button
{
	x: u32,
	y: u32,
	w: u32,
	text: &'static str
}

impl Button
{
	fn render(&self, sel: bool)
	{
		let text_x = self.x + self.w / 2;
		let text_y = self.y + BUTTON_HEIGHT / 2;
		lcd_str_center(text_x, text_y, self.text,
				LCD_WHITE, LCD_BLACK, BUTTON_FONT);

		if sel { self.select(); } else { self.deselect(); }
	}

	fn undraw(&self)
	{
		let text_w = BUTTON_FONT.width(self.text);
		let text_h = BUTTON_FONT.height;
		let text_x = self.x + self.w / 2 - text_w / 2;
		let text_y = self.y + BUTTON_HEIGHT / 2 - text_h / 2;
		lcd_rect(text_x, text_y, text_w, text_h, LCD_BLACK);
		boxundraw(self.x, self.y, self.w, BUTTON_HEIGHT);
	}

	fn select(&self)
	{
		boxsel(self.x, self.y, self.w, BUTTON_HEIGHT);
	}

	fn deselect(&self)
	{
		boxdesel(self.x, self.y, self.w, BUTTON_HEIGHT);
	}
}

fn input_select(y: u32)
{
	boxsel(DA_PADDING, INPUT_BOX_Y + y * INPUT_Y_SKIP,
		INPUT_WIDTH, INPUT_HEIGHT);
}

fn input_deselect(y: u32)
{
	boxdesel(DA_PADDING, INPUT_BOX_Y + y * INPUT_Y_SKIP,
		INPUT_WIDTH, INPUT_HEIGHT);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Align
{
	Left,
	Right
}

pub struct Select
{
	align: Align,
	options: &'static [&'static str]
}

pub struct Input
{
	select: &'static Select,
	label: &'static str,
	default_val: u8
}

const SELECT_PIN_LIST: [&str; 8] =
[
	"0", "1", "2", "3", "4", "5", "6", "7"
];

const SELECT_PARITY_LIST: [&str; 3] =
[
	"None", "Even", "Odd"
];

const SELECT_STOP_BITS_LIST: [&str; 3] =
[
	"1", "1.5", "2"
];

const SELECT_DATA_BITS_LIST: [&str; 5] =
[
	"8", "9", "5", "6", "7"
];

const SELECT_BAUDRATE_LIST: [&str; 11] =
[
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

static BAUDRATES: [u32; 11] =
[
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

const SELECT_PIN: Select = Select
{
	align: Align::Right,
	options: &SELECT_PIN_LIST
};

const SELECT_BAUDRATE: Select = Select
{
	align: Align::Right,
	options: &SELECT_BAUDRATE_LIST
};

const SELECT_DATA_BITS: Select = Select
{
	align: Align::Right,
	options: &SELECT_DATA_BITS_LIST
};

const SELECT_PARITY: Select = Select
{
	align: Align::Left,
	options: &SELECT_PARITY_LIST
};

const SELECT_STOP_BITS: Select = Select
{
	align: Align::Right,
	options: &SELECT_STOP_BITS_LIST
};

/* UART */
const UART_RX: Input = Input
{
	select: &SELECT_PIN,
	label: "RX Pin",
	default_val: 0
};

const UART_TX: Input = Input
{
	select: &SELECT_PIN,
	label: "TX Pin",
	default_val: 1
};

const UART_BAUDRATE: Input = Input
{
	select: &SELECT_BAUDRATE,
	label: "Baudrate",
	default_val: 0
};

const UART_DATABITS: Input = Input
{
	select: &SELECT_DATA_BITS,
	label: "Data Bits",
	default_val: 0
};

const UART_PARITY: Input = Input
{
	select: &SELECT_PARITY,
	label: "Parity",
	default_val: 0
};

const UART_STOPBITS: Input = Input
{
	select: &SELECT_STOP_BITS,
	label: "Stop Bits",
	default_val: 0
};

const UART_INPUTS: [&Input; 6] =
[
	&UART_RX,
	&UART_TX,
	&UART_BAUDRATE,
	&UART_DATABITS,
	&UART_PARITY,
	&UART_STOPBITS
];

/* SPI */
const SPI_MOSI: Input = Input
{
	select: &SELECT_PIN,
	label: "MOSI Pin",
	default_val: 0
};

const SPI_MISO: Input = Input
{
	select: &SELECT_PIN,
	label: "MISO Pin",
	default_val: 1
};

const SPI_SCK: Input = Input
{
	select: &SELECT_PIN,
	label: "SCK Pin",
	default_val: 2
};

const SPI_CS: Input = Input
{
	select: &SELECT_PIN,
	label: "CS Pin",
	default_val: 3
};

const SELECT_MODE: Select = Select
{
	align: Align::Left,
	options: &["CPOL=0,CPHA=0", "CPOL=0,CPHA=1", "CPOL=1,CPHA=0", "CPOL=1,CPHA=1"]
};

const SPI_MODE: Input = Input
{
	select: &SELECT_MODE,
	label: "Mode",
	default_val: 0
};

const SELECT_BITORDER: Select = Select
{
	align: Align::Left,
	options: &["MSB First", "LSB First"]
};

const SPI_BITORDER: Input = Input
{
	select: &SELECT_BITORDER,
	label: "Bit Order",
	default_val: 0
};

const SPI_INPUTS: [&Input; 6] =
[
	&SPI_MOSI,
	&SPI_MISO,
	&SPI_SCK,
	&SPI_CS,
	&SPI_MODE,
	&SPI_BITORDER
];

/* I2C */
const I2C_SDA: Input = Input
{
	select: &SELECT_PIN,
	label: "SDA Pin",
	default_val: 0
};

const I2C_SCL: Input = Input
{
	select: &SELECT_PIN,
	label: "SCL Pin",
	default_val: 1
};

const I2C_INPUTS: [&Input; 2] =
[
	&I2C_SDA,
	&I2C_SCL
];

/* OneWire */
const ONEWIRE_PIN: Input = Input
{
	select: &SELECT_PIN,
	label: "OneWire Pin",
	default_val: 0
};

const ONEWIRE_INPUTS: [&Input; 1] =
[
	&ONEWIRE_PIN
];

/* Get value */
fn item_to_baudrate(idx: usize) -> u32
{
	BAUDRATES[idx]
}

fn item_to_pin(idx: usize) -> DecoderPin
{
	idx as DecoderPin
}

fn item_to_bitorder(idx: usize) -> BitOrder
{
	if idx == 0 { BitOrder::MsbFirst } else { BitOrder::LsbFirst }
}

fn item_to_spimode(idx: usize) -> u8
{
	idx as u8
}

fn item_to_databits(idx: usize) -> DataBits
{
	match idx
	{
		2 => DataBits::Five,
		3 => DataBits::Six,
		4 => DataBits::Seven,
		1 => DataBits::Nine,
		_ => DataBits::Eight
	}
}

fn item_to_parity(idx: usize) -> Parity
{
	match idx
	{
		1 => Parity::Even,
		2 => Parity::Odd,
		_ => Parity::None
	}
}

fn item_to_stopbits(idx: usize) -> StopBits
{
	match idx
	{
		1 => StopBits::OneAndHalf,
		2 => StopBits::Two,
		_ => StopBits::One
	}
}

fn cycle_fwd(idx: u32, count: u32) -> u32
{
	if idx == count - 1 { 0 } else { idx + 1 }
}

fn cycle_bwd(idx: u32, count: u32) -> u32
{
	if idx == 0 { count - 1 } else { idx - 1 }
}

macro_rules! limit_inc
{
	($value:expr, $n:expr) =>
	{
		if $value < $n
		{
			$value += 1;
		}
	};
}

macro_rules! limit_dec
{
	($value:expr, $n:expr) =>
	{
		if $value > $n
		{
			$value -= 1;
		}
	};
}

const ACTIONS_INFO: [Action; 8] =
[
	Action::None, Action::None, Action::None, Action::None,
	Action::None, Action::None, Action::None, Action::Enter
];

const ACTIONS_SAMPLING: [Action; 8] =
[
	Action::Stop, Action::None, Action::None, Action::None,
	Action::None, Action::None, Action::None, Action::None
];

const ACTIONS_MAIN: [Action; 8] =
[
	Action::None, Action::None, Action::Left, Action::Right,
	Action::ZoomIn, Action::ZoomOut, Action::Cycle, Action::Enter
];

const ACTIONS_DA: [Action; 8] =
[
	Action::Up, Action::Down, Action::None, Action::None,
	Action::None, Action::None, Action::Escape, Action::Enter
];

const ACTIONS_CD: [Action; 8] =
[
	Action::Up, Action::Down, Action::Left, Action::Right,
	Action::None, Action::None, Action::Escape, Action::Check
];

enum TimeUnit
{
	Second,
	Millisecond,
	Microsecond
}

struct ZoomLevel
{
	value: u32,
	unit: TimeUnit
}

const ZOOM_LEVELS: [ZoomLevel; 21] =
[
	ZoomLevel { value:   5, unit: TimeUnit::Second },
	ZoomLevel { value:   2, unit: TimeUnit::Second },
	ZoomLevel { value:   1, unit: TimeUnit::Second },
	ZoomLevel { value: 500, unit: TimeUnit::Millisecond },
	ZoomLevel { value: 200, unit: TimeUnit::Millisecond },
	ZoomLevel { value: 100, unit: TimeUnit::Millisecond },
	ZoomLevel { value:  50, unit: TimeUnit::Millisecond },
	ZoomLevel { value:  20, unit: TimeUnit::Millisecond },
	ZoomLevel { value:  10, unit: TimeUnit::Millisecond },
	ZoomLevel { value:   5, unit: TimeUnit::Millisecond },
	ZoomLevel { value:   2, unit: TimeUnit::Millisecond },
	ZoomLevel { value:   1, unit: TimeUnit::Millisecond },
	ZoomLevel { value: 500, unit: TimeUnit::Microsecond },
	ZoomLevel { value: 200, unit: TimeUnit::Microsecond },
	ZoomLevel { value: 100, unit: TimeUnit::Microsecond },
	ZoomLevel { value:  50, unit: TimeUnit::Microsecond },
	ZoomLevel { value:  20, unit: TimeUnit::Microsecond },
	ZoomLevel { value:  10, unit: TimeUnit::Microsecond },
	ZoomLevel { value:   5, unit: TimeUnit::Microsecond },
	ZoomLevel { value:   2, unit: TimeUnit::Microsecond },
	ZoomLevel { value:   1, unit: TimeUnit::Microsecond }
];

fn map(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64
{
	(x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn t_to_x(t: u32, start: u32, end: u32) -> u32
{
	let max = (WAVEFORM_W - 1) as f64;
	let x = map(t.into(), start.into(), end.into(), 0.0, max);
	f64::min(f64::max(x, 0.0), max) as u32
}

pub struct Gui
{
	actions: &'static [Action],
	cur_title: &'static str,
	mode: Mode,
	ma_selected: u32,
	da_selected: u32,
	cd_selected: u32,
	sels: [u8; 8],
	inputs: &'static [&'static Input],
	term_rows: u32,
	term_lens: [u8; 16],
	pub buf: SampleBuffer,
	sec_buf: SectionBuffer,
	cur_decoder: DecoderUnion,
	decoder_framebuf: DecoderFrameBuffer<WAVEFORM_W_USIZE>,
	t_start: u32,
	t_end: u32,
	hw: HW,
	zoom: usize,
	pi: PositionIndicator,
	wf: WaveformBuffer
}

impl Gui
{
	fn top_divider()
	{
		lcd_hline(0, ICON_BOX, LCD_WIDTH, LCD_WHITE);
	}

	fn bottom_divider()
	{
		lcd_hline(0, LCD_HEIGHT - ICON_BOX - 1, LCD_WIDTH, LCD_WHITE);
	}

	pub fn term_print(&mut self, s: &str)
	{
		lcd_str(TITLE_X, TERM_Y + self.term_rows * TERMINUS16.height,
			s, LCD_WHITE, LCD_BLACK, &TERMINUS16);

		self.term_lens[self.term_rows as usize] = s.len() as u8;
		self.term_rows += 1;
	}

	pub fn term_undraw(&mut self)
	{
		for i in 0..self.term_rows
		{
			lcd_rect(TITLE_X, TERM_Y + i * TERMINUS16.height,
				self.term_lens[i as usize] as u32 * TERMINUS16.width,
				TERMINUS16.height, LCD_BLACK);
		}

		self.term_rows = 0;
	}

	pub fn init(mut hw: HW) -> Self
	{
		Self::top_divider();
		Self::bottom_divider();

		/* Borrow flash Temporarily to get the Saved Decoder */
		let (decoder, sels) = DecoderStorage::load(&hw.user_flash);

		let mut gui = Gui
		{
			actions: &ACTIONS_MAIN,
			cur_title: "",
			mode: Mode::Main,
			ma_selected: 0,
			da_selected: 0,
			cd_selected: 0,
			sels: sels,
			inputs: &UART_INPUTS,
			term_rows: 0,
			term_lens: [0; 16],
			buf: SampleBuffer::new(),
			sec_buf: SectionBuffer
			{
				sections: [Section::default(); decoder::SECBUF_SIZE],
				len: 0
			},
			cur_decoder: decoder,
			decoder_framebuf: DecoderFrameBuffer::new(),
			t_start: 0,
			t_end: 5 * 1_000_000 * hw::TICKS_PER_US,
			hw: hw,
			zoom: 0,
			pi: PositionIndicator::new(),
			wf: WaveformBuffer::new()
		};

		gui.icon_box();
		gui.actions_render();
		gui.ma_open();
		gui
	}

	fn title_set(&mut self, new_title: &'static str)
	{
		lcd_str(TITLE_X, TITLE_Y, new_title,
			LCD_WHITE, LCD_BLACK, TITLE_FONT);

		let len_diff = self.cur_title.len() as i32 - new_title.len() as i32;
		if len_diff > 0
		{
			let x = TITLE_FONT.width(new_title);
			let w = len_diff as u32 * TITLE_FONT.width;
			lcd_rect(TITLE_X + x, TITLE_Y, w, TITLE_FONT.height, LCD_BLACK);
		}

		self.cur_title = new_title;
	}

	fn icon_box(&self)
	{
		lcd_rect(0, LCD_HEIGHT - ICON_BOX, LCD_WIDTH, ICON_BOX, LCD_BLACK);
		for i in 0..BUTTON_COUNT
		{
			lcd_vline(LCD_WIDTH - (i as u32 + 1) * (ICON_BOX + 1),
				LCD_HEIGHT - ICON_BOX, ICON_BOX, LCD_WHITE);
		}
	}

	fn action_icon_render(x: u32, y: u32, action: Action)
	{
		match action
		{
			Action::Left => lcd_icon_bw(x, y, ICON_LEFT),
			Action::Right => lcd_icon_bw(x, y, ICON_RIGHT),
			Action::Up => lcd_icon_bw(x, y, ICON_UP),
			Action::Down => lcd_icon_bw(x, y, ICON_DOWN),
			Action::Enter => lcd_icon_bw(x, y, ICON_ENTER),
			Action::Escape => lcd_icon_bw(x, y, ICON_EXIT),
			Action::Check => lcd_icon_bw(x, y, ICON_CHECK),
			Action::ZoomIn => lcd_icon_bw(x, y, ICON_TIME_EXPAND),
			Action::ZoomOut => lcd_icon_bw(x, y, ICON_TIME_SHRINK),
			Action::Cycle => lcd_icon_bw(x, y, ICON_CYCLE),
			Action::Stop => lcd_icon_bw(x, y, ICON_STOP),
			_ => lcd_icon_undraw(x, y)
		}
	}

	fn actions_set(&mut self, a: &'static [Action])
	{
		let mut x = ACTION_ICONS_X;
		for i in 0..BUTTON_COUNT
		{
			if a[i] != self.actions[i]
			{
				Self::action_icon_render(x, ACTION_ICONS_Y, a[i]);
			}

			x += ACTION_ICONS_SKIP;
		}

		self.actions = a;
	}

	fn actions_render(&self)
	{
		let mut x = ACTION_ICONS_X;
		for i in 0..BUTTON_COUNT
		{
			if self.actions[i] != Action::None
			{
				Self::action_icon_render(x, ACTION_ICONS_Y, self.actions[i]);
			}

			x += ACTION_ICONS_SKIP;
		}
	}

	fn button_to_action(&self, key: i32) -> Action
	{
		self.actions[(BUTTON_COUNT - 1) - key as usize]
	}

	pub fn action(&mut self, action: Action)
	{
		match self.mode
		{
			Mode::Info => { self.info_action(action); }
			Mode::Main => { self.ma_action(action); }
			Mode::DecoderAdd => { self.da_action(action); }
			Mode::DecoderUart => { self.u_action(action); }
			Mode::DecoderSpi => { self.s_action(action); }
			Mode::DecoderI2C => { self.i_action(action); }
			Mode::DecoderOneWire => { self.o_action(action); }
		};
	}

	pub fn key(&mut self, key: i32)
	{
		self.action(self.button_to_action(key));
	}

	fn mode_switch(&mut self, new_mode: Mode)
	{
		match self.mode
		{
			Mode::Main => self.ma_close(),
			Mode::DecoderAdd => self.da_close(),
			Mode::DecoderUart => self.cd_undraw(),
			Mode::DecoderSpi => self.cd_undraw(),
			Mode::DecoderI2C => self.cd_undraw(),
			Mode::DecoderOneWire => self.cd_undraw(),
			Mode::Info => self.info_close()
		};

		self.mode = new_mode;
		match self.mode
		{
			Mode::Main => self.ma_open(),
			Mode::DecoderAdd => self.da_open(),
			Mode::DecoderUart => self.u_open(),
			Mode::DecoderSpi => self.s_open(),
			Mode::DecoderI2C => self.i_open(),
			Mode::DecoderOneWire => self.o_open(),
			Mode::Info => self.info_open()
		};
	}

	/* === INFO === */
	fn info_action(&mut self, action: Action)
	{
		if action == Action::Enter
		{
			self.mode_switch(Mode::Main);
		}
	}

	fn info_close(&mut self)
	{
		self.term_undraw();
	}

	fn info_open(&mut self)
	{
		self.actions_set(&ACTIONS_INFO);
		self.title_set("About");
		self.term_print("ITS-Board Logic Analyzer V0.1");
		self.term_print("Created by Joel Kypke, Haron Nazari, Anton Tchekov");
		self.term_print("");
		self.term_print("Press any key to continue ...");
	}

	/* === CD COMMON === */
	fn cd_up(&mut self)
	{
		let prev = self.cd_selected;
		self.cd_selected = cycle_bwd(self.cd_selected, self.inputs.len() as u32);
		self.cd_update(prev);
	}

	fn cd_down(&mut self)
	{
		let prev = self.cd_selected;
		self.cd_selected = cycle_fwd(self.cd_selected, self.inputs.len() as u32);
		self.cd_update(prev);
	}

	fn cur_num_options(&self) -> u32
	{
		self.inputs[self.cd_selected as usize].select.options.len() as u32
	}

	fn cd_left(&mut self)
	{
		let idx = self.cd_selected as usize;
		let prev = self.sels[idx];
		self.sels[idx] = cycle_bwd(self.sels[idx].into(), self.cur_num_options()) as u8;
		self.cd_sel_update(prev);
	}

	fn cd_right(&mut self)
	{
		let idx = self.cd_selected as usize;
		let prev = self.sels[idx];
		self.sels[idx] = cycle_fwd(self.sels[idx].into(), self.cur_num_options()) as u8;
		self.cd_sel_update(prev);
	}

	fn cd_sel_update(&mut self, prev_idx: u8)
	{
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
		if wdiff > 0
		{
			let udiff = wdiff as u32;
			let rx = if align == Align::Right { x - udiff } else { x + w };
			lcd_rect(rx, ry, udiff, TERMINUS16.height, LCD_BLACK);
		}

		lcd_str(x, ry, text, LCD_WHITE, LCD_BLACK, &TERMINUS16);
	}

	fn cd_update(&mut self, prev: u32)
	{
		input_deselect(prev);
		input_select(self.cd_selected);
	}

	fn input_text_x(align: Align, w: u32) -> u32
	{
		DA_PADDING + if align == Align::Right { INPUT_WIDTH - 2 - w } else { 2 }
	}

	fn input_undraw(&mut self, input: &Input, y: u32)
	{
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

	fn input_render(&mut self, input: &Input, y: u32)
	{
		lcd_str(DA_PADDING, INPUT_LABEL_Y + y * INPUT_Y_SKIP,
			input.label, LCD_WHITE, LCD_BLACK, &TERMINUS16);

		if y == self.cd_selected
		{
			input_select(y);
		}
		else
		{
			input_deselect(y);
		}

		let text = input.select.options[self.sels[y as usize] as usize];
		lcd_str(Self::input_text_x(input.select.align, TERMINUS16.width(text)),
			INPUT_TEXT_Y + y * INPUT_Y_SKIP,
			text, LCD_WHITE, LCD_BLACK, &TERMINUS16);
	}

	fn cd_undraw(&mut self)
	{
		let mut y = 0;
		for input in self.inputs
		{
			self.input_undraw(input, y);
			y += 1;
		}
	}

	fn cd_render(&mut self, inputs: &'static [&Input])
	{
		self.cd_selected = 0;
		let same_inputs = core::ptr::addr_eq(self.inputs as *const _, inputs as *const _);
		self.inputs = inputs;
		self.actions_set(&ACTIONS_CD);
		for (y, input) in inputs.iter().enumerate()
		{
			if !same_inputs || matches!(self.cur_decoder, DecoderUnion::None)
			{
				self.sels[y] = input.default_val;
			}

			self.input_render(input, y as u32);
		}
	}

	fn cd_action(&mut self, action: Action)
	{
		match action
		{
			Action::Up => self.cd_up(),
			Action::Down => self.cd_down(),
			Action::Left => self.cd_left(),
			Action::Right => self.cd_right(),
			Action::Escape => self.mode_switch(Mode::Main),
			_ => {}
		}
	}

	fn draw_config_saved(i: u32, color: u16, s: &str)
	{
		let x = 200;
		let y = 200 - (i * 10);
		lcd_str(x, y, s, color, LCD_BLACK, TITLE_FONT);
	}

	fn draw_config_saved_animation()
	{
		let s = "Configuration Saved";
		for i in 1..4
		{
			Self::draw_config_saved(i, LCD_GREEN, s);
			delay_ms(10);
			Self::draw_config_saved(i, LCD_BLACK, s);
		}
	}

	fn decoder_done(&mut self, decoder: DecoderUnion)
	{
		let s = "Saving ...";
		Self::draw_config_saved(0, LCD_GREEN, s);
		DecoderStorage::save(&mut self.hw.user_flash, &decoder, &self.sels);
		self.cur_decoder = decoder;
		Self::draw_config_saved(0, LCD_BLACK, s);

		Self::draw_config_saved_animation();
		self.run_decoder();
		self.mode_switch(Mode::Main);
	}

	fn invalid_input()
	{
		let s = "Invalid Input";
		for i in 0..3
		{
			Self::draw_config_saved(0, LCD_RED, s);
			delay_ms(200);
			Self::draw_config_saved(0, LCD_BLACK, s);
			delay_ms(200);
		}
	}

	/* === UART (U) MODE === */
	fn u_open(&mut self)
	{
		self.title_set("UART Decoder");
		self.cd_render(&UART_INPUTS);
	}

	fn u_action(&mut self, action: Action)
	{
		match action
		{
			Action::Check => self.u_save(),
			_ => self.cd_action(action)
		};
	}

	fn u_save(&mut self)
	{
		let d = DecoderUart
		{
			rx_pin: item_to_pin(self.sels[0].into()),
			tx_pin: item_to_pin(self.sels[1].into()),
			databits: item_to_databits(self.sels[3].into()),
			parity: item_to_parity(self.sels[4].into()),
			stopbits: item_to_stopbits(self.sels[5].into()),
			baudrate: item_to_baudrate(self.sels[2].into())
		};

		if !d.is_valid() { Self::invalid_input(); return; }
		let x = DecoderUnion::Uart(d);
		self.decoder_done(x);
	}

	/* === SPI (S) MODE === */
	fn s_open(&mut self)
	{
		self.title_set("SPI Decoder");
		self.cd_render(&SPI_INPUTS);
	}

	fn s_action(&mut self, action: Action)
	{
		match action
		{
			Action::Check => self.s_save(),
			_ => self.cd_action(action)
		};
	}

	fn s_save(&mut self)
	{
		let d = DecoderSPI
		{
			miso_pin: item_to_pin(self.sels[0].into()),
			mosi_pin: item_to_pin(self.sels[1].into()),
			sck_pin: item_to_pin(self.sels[2].into()),
			cs_pin: item_to_pin(self.sels[3].into()),
			mode: item_to_spimode(self.sels[4].into()),
			bitorder: item_to_bitorder(self.sels[5].into()),
		};

		if !d.is_valid() { Self::invalid_input(); return; }
		let x = DecoderUnion::SPI(d);
		self.decoder_done(x);
	}

	/* === I2C (I) MODE === */
	fn i_open(&mut self)
	{
		self.title_set("I2C Decoder");
		self.cd_render(&I2C_INPUTS);
	}

	fn i_action(&mut self, action: Action)
	{
		match action
		{
			Action::Check => self.i_save(),
			_ => self.cd_action(action)
		};
	}

	fn i_save(&mut self)
	{
		let d = DecoderI2C
		{
			sda_pin: item_to_pin(self.sels[0].into()),
			scl_pin: item_to_pin(self.sels[1].into())
		};

		if !d.is_valid() { Self::invalid_input(); return; }
		let x = DecoderUnion::I2C(d);
		self.decoder_done(x);
	}

	/* === ONEWIRE (O) MODE === */
	fn o_open(&mut self)
	{
		self.title_set("OneWire Decoder");
		self.cd_render(&ONEWIRE_INPUTS);
	}

	fn o_action(&mut self, action: Action)
	{
		match action
		{
			Action::Check => self.o_save(),
			_ => self.cd_action(action)
		};
	}

	fn o_save(&mut self)
	{
		let d = DecoderOneWire
		{
			onewire_pin: item_to_pin(self.sels[0].into())
		};

		if !d.is_valid() { Self::invalid_input(); return; }
		let x = DecoderUnion::OneWire(d);
		self.decoder_done(x);
	}

	/* === MAIN (MA) MODE === */
	fn zoomlevel_draw(&self)
	{
		let mut a: [u8; 16] = [0; 16];
		let mut buf = ByteMutWriter::new(&mut a);
		let l = &ZOOM_LEVELS[self.zoom];
		write!(buf, "{:>3} {}", l.value,
			match l.unit
			{
				TimeUnit::Second => "s ",
				TimeUnit::Millisecond => "ms",
				TimeUnit::Microsecond => "µs"
			}).unwrap();

		lcd_str(MA_BOTTOM_TEXT_X, ACTION_ICONS_Y + 1, buf.as_str(),
			LCD_WHITE, LCD_BLACK, &TERMINUS16);
	}

	fn zoomlevel_undraw(&self)
	{
		lcd_rect(MA_BOTTOM_TEXT_X, ACTION_ICONS_Y + 1,
			TERMINUS16.width * 6, TERMINUS16.height, LCD_BLACK);
	}

	fn sidebar_clear(&self)
	{
		lcd_rect(0, ICON_BOX + 1, CHANNEL_LABEL_WIDTH,
			LCD_HEIGHT - ((ICON_BOX + 1) * 2), LCD_BLACK);
	}

	fn sidebar_render_decoder_pins(&self)
	{
		let decoder: &dyn Decoder = match &self.cur_decoder
		{
			DecoderUnion::None => return,
			DecoderUnion::Uart(dcd) => dcd,
			DecoderUnion::SPI(dcd) => dcd,
			DecoderUnion::I2C(dcd) => dcd,
			DecoderUnion::OneWire(dcd) => dcd
		};

		let mut i = 0;
		while let Some((text, pin_num)) = decoder.get_pin(i)
		{
			let y = WAVEFORMS_Y + WAVEFORM_PIN_Y + (pin_num as u32) * WAVEFORM_SPACING;
			lcd_str(0, y as u32, text, LCD_WHITE, LCD_BLACK, &TINYFONT);
			i += 1;
		}
	}

	fn sidebar_render(&self)
	{
		for i in 0..8
		{
			let y = WAVEFORMS_Y + i * WAVEFORM_SPACING;
			lcd_char(CHANNEL_LABEL_WIDTH / 2, y, '0' as u32 + i, LCD_WHITE, LCD_BLACK, &TERMINUS16);
			lcd_hline(0, y, CHANNEL_LABEL_WIDTH, LCD_WHITE);
		}

		self.sidebar_render_decoder_pins();

		lcd_vline(CHANNEL_LABEL_WIDTH - 1, ICON_BOX + 1,
			LCD_HEIGHT - ((ICON_BOX + 1) * 2), LCD_WHITE);
	}

	fn waveform_render(&mut self, s: usize, e: usize, ch: u32)
	{
		let mut prev = self.buf.get(s, ch);
		for i in s..=e
		{
			let cur = self.buf.get(i, ch);
			let x0 = t_to_x(prev.1, self.t_start, self.t_end);
			let x1 = t_to_x(cur.1, self.t_start, self.t_end);
			self.wf.line(ch, x0, x1, prev.0);
			prev = cur;
		}
	}

	fn waveforms_render(&mut self)
	{
		if self.buf.len < 1
		{
			return;
		}

		self.decoder_framebuf.render(&self.sec_buf, self.t_start, self.t_end);

		let s = self.buf.find_start(self.t_start);
		let e = self.buf.find_end(self.t_end);
		for ch in 0..8
		{
			self.waveform_render(s, e, ch);
		}

		self.wf.update();
	}

	fn ma_render(&mut self, i: u32, sel: bool)
	{
		const ICONS: [u32; MA_ICONS as usize] = [ ICON_START, ICON_ADD, ICON_INFO ];
		let fg = if sel { COLOR_SEL } else { LCD_WHITE };
		let x = LCD_WIDTH - (MA_ICONS - i) * (ICON_BOX + 1) + ICON_PADDING;
		lcd_icon_color(x, ICON_PADDING, ICONS[i as usize], fg, LCD_BLACK);
	}

	fn ma_top_box(&mut self)
	{
		for i in 0..MA_ICONS
		{
			lcd_vline(LCD_WIDTH - (i + 1) * (ICON_BOX + 1),
				0, ICON_BOX, LCD_WHITE);
			self.ma_render(i, i == self.ma_selected);
		}
	}

	fn ma_update(&mut self, prev: u32)
	{
		self.ma_render(prev, false);
		self.ma_render(self.ma_selected, true);
	}

	fn update_indicator(&mut self)
	{
		self.pi.show(self.t_start, self.t_end, self.last_ts());
	}

	fn ma_open(&mut self)
	{
		self.title_set("Logic Analyzer");
		self.actions_set(&ACTIONS_MAIN);
		self.da_selected = 0;
		self.ma_top_box();
		self.sidebar_render();
		self.waveforms_render();
		self.zoomlevel_draw();
		self.update_indicator();
	}

	fn ma_close(&mut self)
	{
		self.decoder_framebuf.clear();
		self.sidebar_clear();
		self.zoomlevel_undraw();
		self.wf.undraw();
		self.pi.hide();
		for i in 0..MA_ICONS
		{
			lcd_vline(LCD_WIDTH - (i + 1) * (ICON_BOX + 1),
				0, ICON_BOX, LCD_BLACK);
			lcd_icon_undraw(
				LCD_WIDTH - (MA_ICONS - i) * (ICON_BOX + 1) + ICON_PADDING,
				ICON_PADDING);
		}
	}

	fn ma_running(&mut self)
	{
		lcd_icon_color(4, ACTION_ICONS_Y, ICON_DOT, LCD_GREEN, LCD_BLACK);
		lcd_str(MA_BOTTOM_TEXT_X, ACTION_ICONS_Y + 1, "RUNNING",
			LCD_WHITE, LCD_BLACK, &TERMINUS16_BOLD);
	}

	fn ma_running_undraw(&mut self)
	{
		lcd_rect(4, ACTION_ICONS_Y,
			16 + 6 + TERMINUS16_BOLD.width("RUNNING"), 16, LCD_BLACK);
	}

	fn run_decoder(&mut self)
	{
		self.sec_buf.clear();
		let decoder: &dyn Decoder = match &self.cur_decoder
		{
			DecoderUnion::None => return,
			DecoderUnion::Uart(dcd) => dcd,
			DecoderUnion::SPI(dcd) => dcd,
			DecoderUnion::I2C(dcd) => dcd,
			DecoderUnion::OneWire(dcd) => dcd
		};

		let _ = decoder.decode(&self.buf, &mut self.sec_buf);
	}

	fn ma_run(&mut self)
	{
		self.ma_running();
		self.actions_set(&ACTIONS_SAMPLING);
		sampler::sample_blocking(&mut self.buf);
		self.actions_set(&ACTIONS_MAIN);
		self.ma_running_undraw();
		self.run_decoder();
		self.zoom = 0;
		self.t_start = 0;
		self.zoomlevel_update();
		self.update_indicator();
		self.write_buf_as_csv();
	}

	fn write_buf_as_csv(&mut self)
	{
		writeln!(self.hw.tx, "Timestamp,Data").unwrap();
		for i in 0..self.buf.len
		{
			let sample_data = self.buf.samples[i];
			let sample_ts = self.buf.timestamps[i];

			writeln!(self.hw.tx, "{},{}", sample_ts, sample_data).unwrap();
		}
	}

	fn ma_enter(&mut self)
	{
		match self.ma_selected
		{
			0 => { self.ma_run(); }
			1 => { self.mode_switch(Mode::DecoderAdd); }
			2 => { self.mode_switch(Mode::Info); }
			_ => {}
		}
	}

	fn zoomlevel_to_ticks(&self) -> u32
	{
		let l = &ZOOM_LEVELS[self.zoom];
		l.value * match l.unit
		{
			TimeUnit::Second => hw::TICKS_PER_US * 1_000_000,
			TimeUnit::Millisecond => hw::TICKS_PER_US * 1_000,
			TimeUnit::Microsecond => hw::TICKS_PER_US
		}
	}

	fn zoomlevel_update(&mut self)
	{
		self.zoomlevel_draw();
		self.t_end = self.t_start + self.zoomlevel_to_ticks();
		self.waveforms_render();
	}

	fn last_ts(&self) -> u32
	{
		if self.buf.len == 0 { 0 } else { self.buf.timestamps[self.buf.len - 1] }
	}

	fn max_horizontal_scroll(&self) -> u32
	{
		let last = self.last_ts();
		if last < self.t_end { 0 } else { last - self.t_end }
	}

	fn horizontal_scroll_amount(&self) -> u32
	{
		(self.t_end - self.t_start) / 4
	}

	fn ma_action(&mut self, action: Action)
	{
		match action
		{
			Action::Up =>
			{
			}
			Action::Down =>
			{
			}
			Action::Left =>
			{
				let amount = u32::min(self.horizontal_scroll_amount(), self.t_start);
				self.t_start -= amount;
				self.t_end -= amount;
				self.waveforms_render();
				self.update_indicator();
			}
			Action::Right =>
			{
				let amount = u32::min(self.horizontal_scroll_amount(), self.max_horizontal_scroll());
				self.t_start += amount;
				self.t_end += amount;
				self.waveforms_render();
				self.update_indicator();
			},
			Action::ZoomIn =>
			{
				limit_inc!(self.zoom, ZOOM_LEVELS.len() - 1);
				self.zoomlevel_update();
				self.update_indicator();
			},
			Action::ZoomOut =>
			{
				limit_dec!(self.zoom, 0);
				self.zoomlevel_update();
				self.update_indicator();
			},
			Action::Cycle =>
			{
				let prev = self.ma_selected;
				self.ma_selected = cycle_fwd(self.ma_selected, MA_ICONS);
				self.ma_update(prev);
			}
			Action::Enter =>
			{
				self.ma_enter();
			}
			_ => {}
		}
	}

	/* === DECODER ADD (DA) MODE === */
	fn da_enter(&mut self)
	{
		match self.da_selected
		{
			0 => { self.mode_switch(Mode::DecoderUart); 	},
			1 => { self.mode_switch(Mode::DecoderSpi); 		},
			2 => { self.mode_switch(Mode::DecoderI2C); 		},
			3 => { self.mode_switch(Mode::DecoderOneWire); 	},
			4 => { self.decoder_done(DecoderUnion::None); 	},
			_ => {}
		}
	}

	fn da_button(&self, idx: u32) -> Button
	{
		const LABELS: [&str; DECODER_COUNT as usize] = [ "UART", "SPI", "I2C", "OneWire", "None" ];
		Button
		{
			x: DA_PADDING,
			y: idx * (BUTTON_HEIGHT + DA_PADDING) + ICON_BOX + 1 + DA_PADDING,
			w: DA_BTN_WIDTH,
			text: LABELS[idx as usize]
		}
	}

	fn da_open(&mut self)
	{
		self.title_set("Select Protocol Decoder");
		self.actions_set(&ACTIONS_DA);
		for i in 0..DECODER_COUNT
		{
			self.da_button(i).render(i == self.da_selected);
		}
	}

	fn da_close(&mut self)
	{
		for i in 0..DECODER_COUNT
		{
			self.da_button(i).undraw();
		}
	}

	fn da_switch(&mut self, prev: u32)
	{
		self.da_button(prev).deselect();
		self.da_button(self.da_selected).select();
	}

	fn da_action(&mut self, action: Action)
	{
		match action
		{
			Action::Up =>
			{
				let prev = self.da_selected;
				self.da_selected = cycle_bwd(self.da_selected, DECODER_COUNT);
				self.da_switch(prev);
			},
			Action::Down =>
			{
				let prev = self.da_selected;
				self.da_selected = cycle_fwd(self.da_selected, DECODER_COUNT);
				self.da_switch(prev);
			},
			Action::Enter =>
			{
				self.da_enter();
			},
			Action::Escape =>
			{
				self.mode_switch(Mode::Main);
			},
			_ => {}
		};
	}
}
