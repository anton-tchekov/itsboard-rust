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
- Protocol Decoder GUIs
- SD Card Driver + SD Info on init screen
- Blocking Sample Loop 16 channels
- Select Sampling Frequency in GUI
- Start capture on Start + trigger
- Zoom in / Zoom out + Move forward / backward on timeline
- Send Raw Captured data over UART

### Ferien
- Protocol decoding
	1. I2C
	2. UART
	3. SPI
	4. OneWire

### Im Semester (Wenn Zeit ist)

- FAT File System Driver
- Concurrent Sample Loop
- Start / Stop capture
- Measure Time with cursors
- Send all data over UART
- Save capture to SD card
- Save screenshots to SD card
- Save Load Capture GUI
