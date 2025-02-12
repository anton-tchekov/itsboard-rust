use std::sync::{LazyLock, Mutex};

pub const LCD_WIDTH: u32 = 480;
pub const LCD_HEIGHT: u32 = 320;

pub const LCD_BLACK: u16 = 0x0000;
pub const LCD_WHITE: u16 = 0xFFFF;

pub const LCD_SIZE: usize = LCD_WIDTH as usize * LCD_HEIGHT as usize;

pub struct PrivDat {
	pub image: Box<[u32; LCD_SIZE]>,
	wx: u32,
	wy: u32,
	ww: u32,
	wh: u32,
	px: u32,
	py: u32
}

pub static IMAGE: LazyLock<Mutex<PrivDat>> =
	LazyLock::new(|| Mutex::new(
		PrivDat {
			image: Box::new([0; LCD_SIZE]),
			wx: 0,
			wy: 0,
			ww: 0,
			wh: 0,
			px: 0,
			py: 0
		}));

const fn rgb565_to_rgba(rgb565: u16) -> u32 {
	let r = (rgb565 >> 11) & 0x1F;
	let g = (rgb565 >> 5) & 0x3F;
	let b = rgb565 & 0x1F;

	let r8 = (r << 3) | (r >> 2);
	let g8 = (g << 2) | (g >> 4);
	let b8 = (b << 3) | (b >> 2);

	(0xFF << 24) | ((r8 as u32) << 16) | ((g8 as u32) << 8) | (b8 as u32)
}

pub fn lcd_emit(color: u16) {
	let mut v = IMAGE.lock().unwrap();
	let idx = (v.wy as usize + v.py as usize) * (LCD_WIDTH as usize) +
		(v.wx as usize + v.px as usize);
	v.image[idx] = rgb565_to_rgba(color);
	v.px += 1;
	if v.px == v.ww {
		v.px = 0;
		v.py += 1;
	}
}

pub fn lcd_window_start(x: u32, y: u32, w: u32, h: u32) {
	let mut v = IMAGE.lock().unwrap();
	v.wx = x;
	v.wy = y;
	v.ww = w;
	v.wh = h;
	v.px = 0;
	v.py = 0;
}

pub fn lcd_window_end() {}

pub fn lcd_rect(x: u32, y: u32, w: u32, h: u32, color: u16) {
	let mut count = w * h;
	lcd_window_start(x, y, w, h);
	while count > 0 {
		count -= 1;
		lcd_emit(color);
	}

	lcd_window_end();
}

pub fn lcd_callback(x: u32, y: u32, w: u32, h: u32,
	callback: &dyn Fn(u32, u32) -> u16) {
	lcd_window_start(x, y, w, h);
	let mut y0 = 0;
	while y0 < h {
		let mut x0 = 0;
		while x0 < w {
			lcd_emit(callback(x0, y0));
			x0 += 1;
		}

		y0 += 1;
	}

	lcd_window_end();
}

pub fn lcd_clear(color: u16) {
	lcd_rect(0, 0, LCD_WIDTH, LCD_HEIGHT, color);
}

pub fn lcd_init(color: u16) {
	lcd_clear(color);
}

pub const fn lcd_color(r: u8, g: u8, b: u8) -> u16 {
	((r as u16 & 0xF8) << 8) |
		((g as u16 & 0xFC) << 3) |
		(b as u16 >> 3)
}

pub fn lcd_vline(x: u32, y: u32, h: u32, color: u16) {
	lcd_rect(x, y, 1, h, color);
}

pub fn lcd_hline(x: u32, y: u32, w: u32, color: u16) {
	lcd_rect(x, y, w, 1, color);
}
