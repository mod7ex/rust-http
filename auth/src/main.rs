#![allow(unused)]

use reqwest::Error;
use reqwest::blocking::Client;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let repsonse = client.get("http://httpbin.org/get")
                                    .basic_auth(user, passwd)
                                    .send()
                                    .unwrap();

    println!("{:#?}", repsonse);

    Ok(())
}