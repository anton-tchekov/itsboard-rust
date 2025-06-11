all: build-linux build-windows copy-bins

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

copy-bins:
	cp target/x86_64-pc-windows-gnu/release/uploader.exe uploader.exe
	cp target/x86_64-unknown-linux-gnu/release/uploader uploader