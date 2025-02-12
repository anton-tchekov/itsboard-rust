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

### Ferien
- Remove Unsafe Code
- GUI Emulator
- Control using buttons
- Bottom bar with Icons/Current function mapping for buttons
- Sampling / Buffering GPIO Port
- 8 Channels
- Select Sampling Frequency in GUI
- Blocking Sample Loop
- Start capture on trigger
- Hide / Show Channels with Checkboxes
- Zoom in / Zoom out
- Move forward / backward on timeline
- Protocol decoding
	1. I2C
	2. UART
	3. SPI
	4. OneWire
- Send Raw Captured data over UART

### Im Semester (Wenn Zeit ist)

- SD Card Driver
- FAT File System Driver
- 16 Channels
- Concurrent Sample Loop
- Start / Stop capture
- Measure Time with cursors
- Send more data over UART
- Save capture to SD card
- Save screenshots to SD card
