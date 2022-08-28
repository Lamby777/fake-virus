use native_dialog::{MessageDialog, MessageType};
use futures::executor::block_on;
use std::time::Duration;
use std::thread::sleep;

fn main() {
	block_on(main_async());
}

async fn main_async() {
	loop {
		open_dialog().await;
		wait(2000).await;
		//futures::join!(open_dialog(), wait(2000));
	}
}

async fn wait(time: u64) -> () {
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