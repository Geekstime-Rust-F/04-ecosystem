use anyhow::Context;
use ecosystem::MyError;
use std::{fs, mem::size_of};

fn main() -> Result<(), anyhow::Error> {
    println!("size of IO Error: {}", size_of::<std::io::Error>());
    println!("size of String: {}", size_of::<String>());
    println!("size of MyError: {}", size_of::<MyError>());

    let filename = "./lib.rs";
    let _io_err =
        fs::File::open(filename).with_context(|| format!("can not find file {} ", filename))?;

    fail_with_my_error()?;

    println!("Hello, world! F");

    Ok(())
}

fn fail_with_my_error() -> Result<(), MyError> {
    Err(MyError::Custom("custom error".to_string()))
}
