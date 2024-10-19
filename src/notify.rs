use notify_rust::Notification;
use rfd::{MessageButtons, MessageDialog};

pub fn notify(msg: &str) {
    Notification::new()
        .summary("SIFCB")
        .body(msg)
        .timeout(1500)
        .show()
        .unwrap();
}

pub fn notify_err(err: &str) {
    MessageDialog::new()
        .set_title("Error")
        .set_description(err)
        .set_buttons(MessageButtons::Ok)
        .show();
}
