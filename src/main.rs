use native_dialog::{MessageDialog, MessageType};
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() {
	let mut iteration = 1;

	loop {
		let title = format!("Process #{}", iteration);
		tokio::spawn(open_dialog(title, "spoon :D"));
		wait(2000);
		iteration += 1;
	}
}

fn wait(time: u64) -> () {
	sleep(Duration::from_millis(time));
}

async fn open_dialog<Tt: AsRef<str>, Tx: AsRef<str>>(title: Tt, text: Tx) {
	let rtitle = title.as_ref();
	let rtext = text.as_ref();

	MessageDialog::new()
	.set_type(MessageType::Warning)
	.set_title(&rtitle)
	.set_text(&rtext)
	.show_confirm()
	.unwrap();
}