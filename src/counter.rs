use std::{
    env::var,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::LazyLock,
};

use crate::{error::Result, notify::notify_err};

static SIFCB_COUNTER_FILE_PATH: LazyLock<String> = LazyLock::new(|| {
    var("SIFCB_COUNTER_FILE_PATH")
        .inspect_err(|e| notify_err(&e.to_string()))
        .unwrap()
});

pub fn read_counter() -> Result<i32> {
    let counter_file = PathBuf::from(SIFCB_COUNTER_FILE_PATH.as_str());

    if counter_file.exists() {
        let mut file = File::open(counter_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents.trim().parse::<i32>().unwrap_or(0))
    } else {
        Ok(0)
    }
}

pub fn write_counter(counter: i32) -> Result<()> {
    let counter_file = PathBuf::from(SIFCB_COUNTER_FILE_PATH.as_str());
    let mut file = File::create(counter_file)?;
    file.write_all(counter.to_string().as_bytes())
        .map_err(Into::into)
}
