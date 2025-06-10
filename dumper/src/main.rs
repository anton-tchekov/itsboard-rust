use std::env;
use std::time::Duration;
use std::io;
use std::io::Write;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 && args.len() != 4
	{
		eprintln!("Usage: ./serial-dump `port` `baudrate` [`file`]");
		let ports = serialport::available_ports()?;
		for p in ports
		{
			println!("{}", p.port_name);
		}

		return Ok(());
	}

	let mut file = if args.len() == 4
	{
		let filename = &args[3];
		Some(File::create(filename)?)
	}
	else { None };

	let mut port = serialport::new(&args[1], args[2].parse()?)
		.timeout(Duration::from_millis(1000))
		.open()?;

	loop
	{
		let mut serial_buf: Vec<u8> = vec![0; 1024];
		match port.read(serial_buf.as_mut_slice())
		{
			Ok(n) =>
			{
				if let Some(ref mut file) = file
				{
					file.write_all(&serial_buf[0..n])?;
				}

				io::stdout().write_all(&serial_buf[0..n])?;
				io::stdout().flush()?;
			}
			Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
			Err(e) =>
			{
				eprintln!("{:?}", e);
				break;
			},
		};
	}

	Ok(())
}
