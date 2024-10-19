use std::{fmt::Display, io};

use image::ImageError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoImageInClipboard,
    Io(io::Error),
    Image(image::error::ImageError),
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Error::Image(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoImageInClipboard => write!(f, "No image in clipboard"),
            Error::Io(err) => err.fmt(f),
            Error::Image(err) => err.fmt(f),
        }
    }
}
