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

SAMPLE LOOP

let prev = port_get();
for(;;)
{
	let port = port_get();
	let buttons = buttons_get();
	if(buttons != 0xFF)
	{
		break;
	}

	if(port != prev)
	{
		prev = port;
		let ts = timer_get();
		arr[i++] = { ts, port };
	}
}




- Datenaufnahme
	- Pin Change Sample Loop: Save Port State and Timestamp, exit on any Button press (Haron)
	- Start / Stop Capture

- Darstellung
	- Render Waveform + Zoom in / Zoom out + Move forward / backward on timeline
	- Measure Time with cursors
	- Render Protocol Decoder Outputs

- Protokoll Decoding
	- UART (Joel, bis nächstes Praktikum + Tests)
	- I2C (Joel, anfangen)
	- SPI
	- OneWire
