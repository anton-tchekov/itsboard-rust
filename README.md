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
3. SD Card Info

### Ferien
- Start capture on Start + trigger
- Zoom in / Zoom out + Move forward / backward on timeline
- Send Raw Captured data over UART
- Blocking Sample Loop 16 channels
- Select Sampling Frequency in GUI
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
