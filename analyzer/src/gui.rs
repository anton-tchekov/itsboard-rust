use crate::lcd::*;
use crate::font::*;
use crate::TERMINUS16_BOLD;
use crate::TERMINUS16;
use crate::terminus16::*;
use crate::sample::*;

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

pub struct Gui {
	icons: [KeyIcon; BUTTON_COUNT],
	visible_channels: Sample,
	sample_offset: u32,
	pixels_per_sample: u32
}

impl Gui {
	pub fn init() -> Self {
		lcd_str(0, 0, "ITS-Board Logic Analyzer V0.1",
			LCD_WHITE, LCD_BLACK, &TERMINUS16_BOLD);



		lcd_str(0, 304, "Created by Joel Kypke, Haron Nazari, Anton Tchekov",
			LCD_WHITE, LCD_BLACK, &TERMINUS16);

		Gui {
			icons: [KeyIcon::Disabled; 8],
			visible_channels: 0xFF,
			sample_offset: 0,
			pixels_per_sample: 5
		}
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

	fn dropdown() {

	}
}

fn channel_str(channel: u32, out: &mut [u8]) {
	out[0] = b'D';
	out[1] = (channel / 10) as u8 + b'0';
	out[2] = (channel % 10) as u8 + b'0';
}

fn channel_render() {
}
