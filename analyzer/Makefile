build:
	clear
	cargo build --release
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/itsboard-rust out.hex

emu:
	cp src/gui.rs emulator/src/gui.rs
	cp src/font.rs emulator/src/font.rs
	cp src/terminus16_bold.rs emulator/src/terminus16_bold.rs
	cp src/terminus16.rs emulator/src/terminus16.rs
	cd emulator && cargo run

dump:
	arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/release/itsboard-rust

clean:
	cargo clean
	rm -f out.hex

flash:
	st-flash --connect-under-reset --format ihex write out.hex
