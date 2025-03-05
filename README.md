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

### Diese Woche
1. Start capture on Start + trigger + Blocking Sample Loop 16 channels
2. Send Raw Captured data over UART
3. SD Card Driver + Info / Error Msg

### Later
4. Select Sampling Frequency in GUI
5. Render Waveform + Zoom in / Zoom out + Move forward / backward on timeline

### Wenn Protocol Decoder Fertig
6. Render Protocol Decoder Outputs

### Ferien
- Protocol decoding
	1. I2C
	2. UART
	3. SPI
	4. OneWire

### Im Semester
- FAT File System Driver
- Concurrent Sample Loop
- Start / Stop capture
- Measure Time with cursors
- Save capture to SD card
- Save screenshots to SD card
- Save Load Capture GUI
