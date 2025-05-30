use crate::font::Font;

const TERMINUS16_BITMAP: [u8; 1536] =
[
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x00,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x66,
	0x66,
	0x66,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x6C,
	0x6C,
	0x6C,
	0xFE,
	0x6C,
	0x6C,
	0xFE,
	0x6C,
	0x6C,
	0x6C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x10,
	0x10,
	0x7C,
	0xD6,
	0xD0,
	0xD0,
	0x7C,
	0x16,
	0x16,
	0xD6,
	0x7C,
	0x10,
	0x10,
	0x00,
	0x00,

	0x00,
	0x00,
	0x66,
	0xD6,
	0x6C,
	0x0C,
	0x18,
	0x18,
	0x30,
	0x36,
	0x6B,
	0x66,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x38,
	0x6C,
	0x6C,
	0x38,
	0x76,
	0xDC,
	0xCC,
	0xCC,
	0xDC,
	0x76,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x18,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x0C,
	0x18,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x18,
	0x0C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x30,
	0x18,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x18,
	0x30,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x6C,
	0x38,
	0xFE,
	0x38,
	0x6C,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x7E,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x30,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x06,
	0x06,
	0x0C,
	0x0C,
	0x18,
	0x18,
	0x30,
	0x30,
	0x60,
	0x60,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xCE,
	0xDE,
	0xF6,
	0xE6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x18,
	0x38,
	0x78,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x7E,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0x06,
	0x0C,
	0x18,
	0x30,
	0x60,
	0xC0,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0x06,
	0x3C,
	0x06,
	0x06,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x06,
	0x0E,
	0x1E,
	0x36,
	0x66,
	0xC6,
	0xFE,
	0x06,
	0x06,
	0x06,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFE,
	0xC0,
	0xC0,
	0xC0,
	0xFC,
	0x06,
	0x06,
	0x06,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x3C,
	0x60,
	0xC0,
	0xC0,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFE,
	0x06,
	0x06,
	0x0C,
	0x0C,
	0x18,
	0x18,
	0x30,
	0x30,
	0x30,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x06,
	0x06,
	0x0C,
	0x78,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x18,
	0x18,
	0x30,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x06,
	0x0C,
	0x18,
	0x30,
	0x60,
	0x30,
	0x18,
	0x0C,
	0x06,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFE,
	0x00,
	0x00,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x60,
	0x30,
	0x18,
	0x0C,
	0x06,
	0x0C,
	0x18,
	0x30,
	0x60,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0x0C,
	0x18,
	0x18,
	0x00,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xCE,
	0xD6,
	0xD6,
	0xD6,
	0xD6,
	0xCE,
	0xC0,
	0x7E,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFE,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFC,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xF8,
	0xCC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xCC,
	0xF8,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFE,
	0xC0,
	0xC0,
	0xC0,
	0xF8,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFE,
	0xC0,
	0xC0,
	0xC0,
	0xF8,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC0,
	0xC0,
	0xDE,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFE,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x3C,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x3C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x1E,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0xCC,
	0xCC,
	0x78,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0xCC,
	0xD8,
	0xF0,
	0xF0,
	0xD8,
	0xCC,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x82,
	0xC6,
	0xEE,
	0xFE,
	0xD6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xE6,
	0xF6,
	0xDE,
	0xCE,
	0xC6,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFC,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xDE,
	0x7C,
	0x06,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFC,
	0xF0,
	0xD8,
	0xCC,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC0,
	0xC0,
	0x7C,
	0x06,
	0x06,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFF,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x6C,
	0x6C,
	0x6C,
	0x38,
	0x38,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xD6,
	0xFE,
	0xEE,
	0xC6,
	0x82,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC6,
	0xC6,
	0x6C,
	0x6C,
	0x38,
	0x38,
	0x6C,
	0x6C,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC3,
	0xC3,
	0x66,
	0x66,
	0x3C,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFE,
	0x06,
	0x06,
	0x0C,
	0x18,
	0x30,
	0x60,
	0xC0,
	0xC0,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x3C,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x3C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x60,
	0x60,
	0x30,
	0x30,
	0x18,
	0x18,
	0x0C,
	0x0C,
	0x06,
	0x06,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x3C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x0C,
	0x3C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x18,
	0x3C,
	0x66,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFE,
	0x00,
	0x00,

	0x30,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7C,
	0x06,
	0x7E,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xC0,
	0xC0,
	0xC0,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFC,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC0,
	0xC0,
	0xC0,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x06,
	0x06,
	0x06,
	0x7E,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xFE,
	0xC0,
	0xC0,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x1E,
	0x30,
	0x30,
	0xFC,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7E,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x06,
	0x06,
	0x7C,
	0x00,

	0x00,
	0x00,
	0xC0,
	0xC0,
	0xC0,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x18,
	0x18,
	0x00,
	0x38,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x3C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x06,
	0x06,
	0x00,
	0x0E,
	0x06,
	0x06,
	0x06,
	0x06,
	0x06,
	0x06,
	0x66,
	0x66,
	0x3C,
	0x00,

	0x00,
	0x00,
	0xC0,
	0xC0,
	0xC0,
	0xC6,
	0xCC,
	0xD8,
	0xF0,
	0xD8,
	0xCC,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x38,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x3C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFC,
	0xD6,
	0xD6,
	0xD6,
	0xD6,
	0xD6,
	0xD6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7C,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFC,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFC,
	0xC0,
	0xC0,
	0xC0,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7E,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x06,
	0x06,
	0x06,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xDE,
	0xF0,
	0xE0,
	0xC0,
	0xC0,
	0xC0,
	0xC0,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x7E,
	0xC0,
	0xC0,
	0x7C,
	0x06,
	0x06,
	0xFC,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x30,
	0x30,
	0x30,
	0xFC,
	0x30,
	0x30,
	0x30,
	0x30,
	0x30,
	0x1E,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0x6C,
	0x6C,
	0x38,
	0x38,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xC6,
	0xC6,
	0xD6,
	0xD6,
	0xD6,
	0xD6,
	0x7C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xC6,
	0xC6,
	0x6C,
	0x38,
	0x6C,
	0xC6,
	0xC6,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0x7E,
	0x06,
	0x06,
	0x7C,
	0x00,

	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0xFE,
	0x0C,
	0x18,
	0x30,
	0x60,
	0xC0,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x1C,
	0x30,
	0x30,
	0x30,
	0x60,
	0x30,
	0x30,
	0x30,
	0x30,
	0x1C,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x18,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0x70,
	0x18,
	0x18,
	0x18,
	0x0C,
	0x18,
	0x18,
	0x18,
	0x18,
	0x70,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x73,
	0xDB,
	0xCE,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,
	0x00,

	0x00,
	0x00,
	0xFE,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xC6,
	0xFE,
	0x00,
	0x00,
	0x00,
	0x00,
];

pub const TERMINUS16_BOLD: Font = Font
{
	horizontal: true,
	width: 8,
	height: 16,
	bitmap: &TERMINUS16_BITMAP
};
