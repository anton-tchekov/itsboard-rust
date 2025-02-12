fn fat_filename(name: &str, entry: &[u8]) {

}

fn create_dir_entry(name: &str, cluster: u32) -> [u8; 32] {
	let entry: [u8; 32] = [0; 32];

	fat_filename(name, &entry[0..10]);

	entry
}
