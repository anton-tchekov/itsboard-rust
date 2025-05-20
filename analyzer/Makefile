build:
	clear
	cargo build --release
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/itsboard-rust out.hex

emu:
	clear
	cp src/gui.rs ../emulator/src/gui.rs
	cp src/font.rs ../emulator/src/font.rs
	cp src/terminus16_bold.rs ../emulator/src/terminus16_bold.rs
	cp src/terminus16.rs ../emulator/src/terminus16.rs
	cp src/tinyfont.rs ../emulator/src/tinyfont.rs
	cp src/decoder_framebuffer.rs ../emulator/src/decoder_framebuffer.rs
	cp src/positionindicator.rs ../emulator/src/positionindicator.rs
	cp src/waveform.rs ../emulator/src/waveform.rs
	cp src/sample.rs ../emulator/src/sample.rs
	cp src/decoder.rs ../emulator/src/decoder.rs
	cp src/decoder_uart.rs ../emulator/src/decoder_uart.rs
	cp src/decoder_spi.rs ../emulator/src/decoder_spi.rs
	cp src/decoder_i2c.rs ../emulator/src/decoder_i2c.rs
	cp src/decoder_onewire.rs ../emulator/src/decoder_onewire.rs
	cp src/bytewriter.rs ../emulator/src/bytewriter.rs
	cd ../emulator && cargo run

dump:
	arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/release/itsboard-rust

clean:
	cargo clean
	rm -f out.hex

flash:
	st-flash --connect-under-reset --format ihex write out.hex
