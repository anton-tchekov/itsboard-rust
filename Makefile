build:
	cargo build --release
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/itsboard-rust out.hex

dump:
	arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/release/itsboard-rust

clean:
	cargo clean
	rm -f out.hex

flash:
	st-flash --format ihex write out.hex
