use native_dialog::{MessageDialog, MessageType};

fn main() {
	let yes = MessageDialog::new()
		.set_type(MessageType::Info)
		.set_title("Imagine getting hacked!")
		.set_text("Clown!")
		.show_confirm()
		.unwrap();
}