trait Display {
	fn callback(&self, x: u32, y: u32, w: u32, h: u32,
		callback: &dyn Fn(u32, u32) -> u16);
}
