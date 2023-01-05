#![allow(unused)]

extern crate reqwest;

use std::io::Read;
use error_chain::error_chain;

const URL: &str = "https://jsonplaceholder.typicode.com/todos/1";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get(URL)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status :{}", res.status());
    println!("Header: \n {:#?}", res.headers());
    println!("Body: \n{}", body);  

    Ok(())
}
