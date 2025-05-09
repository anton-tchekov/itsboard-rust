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

### Praktikum 3
- Simple Polling Sample Loop
- Start/Stop Recording
- Render Waveform + Zoom in / Zoom out + Move forward / backward on timeline
	- Display all visible waveforms
	- Waveform undraw
	- Waveform rendering optimization (Vertical line overlap)
	- Zoom In / Out Steps (window size)
		- Display in Bottom Bar
		- 5 s, 2 s, 1 s
		- 500 ms, 200 ms, 100 ms
		- 50 ms, 20 ms, 10 ms
		- 5 ms, 2 ms, 1 ms
		- 500 us, 200 us, 100 us
		- 50 us, 20 us, 10 us
		- 5 us, 2 us, 1 us

	- Forward / backward
		- Quarter of current window size

- UART Decoding und Tests

### Praktikum 4
- Render Protocol Decoder Outputs
- I2C

### Praktikum 5
- Measure Time with cursors
- SPI
- OneWire

### Praktikum 6
- Dokumentation
- Code aufräumen
- Puffer
