use native_dialog::{MessageDialog, MessageType};
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() {
	loop {
		tokio::spawn(open_dialog());
		wait(2000);
	}
}

fn wait(time: u64) -> () {
	sleep(Duration::from_millis(time));
}

async fn open_dialog() {
	MessageDialog::new()
	.set_type(MessageType::Warning)
	.set_title("Imagine getting hacked!")
	.set_text("Clown!")
	.show_confirm()
	.unwrap();
}