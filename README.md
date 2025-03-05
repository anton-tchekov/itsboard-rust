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

- Start capture on Start + trigger + Blocking Sample Loop 16 channels
- Send Raw Captured data over UART
- SD Card Driver + Info / Error Msg

### Ferien
- Render Waveform + Zoom in / Zoom out + Move forward / backward on timeline
- Variable Sampling Frequency
- Render Protocol Decoder Outputs
- Protocol decoding
	1. I2C
	2. UART
	3. SPI
	4. OneWire

### Im Semester
- Cleanup Code
- FAT File System Driver
- Concurrent Sample Loop
- Start / Stop capture
- Measure Time with cursors
- Save screenshots
- Save and Load Capture
