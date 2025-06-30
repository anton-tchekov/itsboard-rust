build:
	clear
	cargo build --release --target=thumbv7em-none-eabihf
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/itsboard-rust out.hex

sim:
	cargo run --features simulator

dump:
	arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/release/itsboard-rust

clean:
	cargo clean
	rm -f out.hex

flash:
	st-flash --connect-under-reset --format ihex write out.hex
