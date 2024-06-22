use anyhow::Context;
use std::{fs, mem::size_of};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Serialize json error: {0}")]
    SerializeJson(#[from] serde_json::Error),

    #[error("Error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

// 当一个error类型很大的时候, 可以把它单独定义一个struct, 然后在error enum中放一个box指针
#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: Vec<u8>,
}

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
