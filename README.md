# Rust on ITS-Board

## Build and run

Build:

`make build`

Clean:

`make clean`

Flash:

`make flash`

ASM Dump:

`make dump`

## TODO

- Remove Unsafe Code
- GUI Emulator
- SD Card Driver
- FAT File System Driver
- Control using buttons
- Bottom bar with Icons/Current function mapping for buttons
- Sampling / Buffering GPIO Port
- 8/16 Channels
- Start capture on trigger
- Select Sampling Frequency
- Start / Stop capture
- Measure Time with cursors

- Hide / Show Channels
	- Checkboxes

- Zoom in / Zoom out
- Move forward / backward on timeline

- Protocol decoding
	1. I2C
	2. UART
	3. SPI
	4. OneWire

- Send data over USB UART
- Save capture to SD card
- Save screenshots to SD card
