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

### HW Basics

- UART works
- SPI works

- Simple Demo:
	- Test Both fonts
	- Test LCD callback
	- UART send message
	- Rectangles on Screen
	- Label that shows seconds counter
	- Button reset counter to zero
	- Button move rectangle around
	- LED show counter in seconds

### Project

- SD Card, FAT32 Library / Driver

- Control using buttons

- Bottom bar with Icons/Current function mapping for buttons
- Sampling / Buffering GPIO Port

- 8 Channels
- Start capture on trigger
- Select Sampling Frequency
- Start / Stop capture
- Measure Time with cursors
- Hide / Show Channels
- Zoom in / Zoom out
- Move forward / backward on timeline

- Protocol decoding
	- SPI
	- I2C
	- UART
	- OneWire

- Send data over USB UART
- Save capture/screenshots to SD Card
