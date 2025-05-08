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

- Datenaufnahme
	- Pin Change Interrupt: Save Port State and Timestamp
	- Start / Stop Capture

- Darstellung
	- Render Waveform + Zoom in / Zoom out + Move forward / backward on timeline
	- Measure Time with cursors
	- Render Protocol Decoder Outputs

- Protokoll Decoding
	- I2C
	- UART
	- SPI
	- OneWire

- Misc
	- Unsafe entfernen so weit wie möglich
