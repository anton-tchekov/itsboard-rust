use crate::decoder::*;
use crate::decoder_spi::*;
use crate::decoder_uart::*;
use crate::decoder_i2c::*;
use crate::decoder_onewire::*;

static DECODERS: [&ProtocolDecoder; 4] = [
	&DECODER_SPI,
	&DECODER_UART,
	&DECODER_I2C,
	&DECODER_ONEWIRE
];
