#![windows_subsystem = "windows"]

use clippers::{Clipboard, ClipperData};
use counter::{read_counter, write_counter};
use error::{Error, Result};
use image::{save_buffer_with_format, ExtendedColorType, ImageFormat};
use notify::{notify, notify_err};
use std::env::var;
use std::sync::LazyLock;

mod counter;
mod error;
mod notify;

static SIFCB_SAVE_DIR: LazyLock<String> = LazyLock::new(|| {
    var("SIFCB_SAVE_DIR")
        .inspect_err(|e| notify_err(&e.to_string()))
        .unwrap()
});

fn get_next_filename() -> Result<String> {
    let current_counter = read_counter()?;

    Ok(format!(
        "{}/saved-clipboard-{}.jpg",
        SIFCB_SAVE_DIR.as_str(),
        current_counter
    ))
}

fn save_image_from_clipboard() -> Result<String> {
    let mut clipboard = Clipboard::get();
    if let Some(ClipperData::Image(image_data)) = clipboard.read() {
        let path = get_next_filename()?;

        save_buffer_with_format(
            path.clone(),
            image_data.as_raw().rgba(),
            image_data.width(),
            image_data.height(),
            ExtendedColorType::Rgba8,
            ImageFormat::Png,
        )?;

        let current_counter = read_counter()?;
        let next_counter = current_counter + 1;
        write_counter(next_counter)?;

        Ok(format!("Image saved successfully as {}", path))
    } else {
        Err(Error::NoImageInClipboard)
    }
}

fn main() {
    match save_image_from_clipboard() {
        Ok(msg) => {
            notify(&msg);
        }
        Err(err) => notify_err(&format!("Failed to save image: {:?}", err)),
    }
}
