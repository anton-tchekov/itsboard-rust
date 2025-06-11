use eframe::egui::{self, ComboBox, RichText, Visuals};
use std::{process::{Command, Stdio}, sync::{Arc, Mutex}, thread::{self, sleep}};

use crate::program_options::ProgramOptions;

pub struct Gui
{
	selected_program: ProgramOptions,
	uploading: Arc<Mutex<bool>>,
}

const LINUX_COMMAND: &str = "../st-flash/linux/st-flash";
const WIN_COMMAND: &str = r"..\st-flash\windows\st-flash.exe";

fn upload(program: &str)
{
	let command;
	if cfg!(target_os = "linux")
	{
    	command = LINUX_COMMAND;
    }
    else if cfg!(target_os = "windows")
    {
     	command = WIN_COMMAND;
    }
    else
    {
        println!("Unsupported OS");
        std::process::exit(1);
    }

	let args = ["--connect-under-reset", "--format", "ihex", "write", program];

	let full_command = format!("{} {}", command, args.join(" "));
	println!("Executing: {}", full_command);

	let mut process;
	if cfg!(target_os = "linux")
	{
    	process = std::process::Command::new(command)
		.args(&args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()
		.expect("Failed to execute st-flash");
    }
    else if cfg!(target_os = "windows")
    {
     	process = std::process::Command::new(command)
     	.env("STLINK_CHIPS", "..\\st-flash\\windows")
		.args(&args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()
		.expect("Failed to execute st-flash");
    }
    else
    {
        println!("Unsupported OS");
        std::process::exit(1);
    }

	let status = process.wait().expect("Failed to wait on st-flash");
}

impl Gui
{
	pub fn default() -> Self
	{
		Self
		{
			selected_program: ProgramOptions::None,
			uploading: Arc::new(Mutex::new(false))
		}
	}

	fn start_uploading(&self, program: String) -> bool
	{
		if program == ""
		{
			return false;
		}

		let uploading_flag = self.uploading.clone();

		thread::spawn(move ||
		{
			*uploading_flag.lock().unwrap() = true;
			upload(&program);
			*uploading_flag.lock().unwrap() = false;
		});

		return true;
	}
}

impl eframe::App for Gui 
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ctx, |ui| 
		{
			ui.heading("ITS-Board Uploader v1.0");
			ctx.set_visuals(Visuals::dark());

			ComboBox::from_label("Choose a Programm to Upload")
				.selected_text(format!("{:?}", self.selected_program))
				.show_ui(ui, |ui|
				{
					ui.selectable_value(&mut self.selected_program, ProgramOptions::LogicAnalyzer, "Logic Analyzer")
				});
			
			ui.add_space(50.0);

			ui.horizontal(|ui|
			{
				if ui.button(RichText::new("Upload").size(20.0)).clicked()
				{
					if !self.start_uploading(self.selected_program.to_string())
					{
						println!("No Program was selected");
					}
				}

				ui.add_space(20.0);

				if *self.uploading.lock().unwrap() == true
				{
					ui.label(RichText::new("Uploading Programm to the ITS-Board").size(20.0));
				}
			});

			ctx.request_repaint(); 
        });
    }
}