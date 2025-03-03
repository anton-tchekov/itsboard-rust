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

### Anton Heute
- Control using buttons on ITS Board
- Bottom bar with Icons/Current function mapping for buttons
- Hide / Show Channels with Checkboxes
- Protocol Decoder GUIs
- SD Info on init screen

### Ferien
- Sampling / Buffering GPIO Port
- 8 Channels
- Select Sampling Frequency in GUI
- Blocking Sample Loop
- Start capture on Start + trigger
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
- Send all data over UART
- Save capture to SD card
- Save screenshots to SD card
