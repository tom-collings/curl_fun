use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    //println!("Hello, world!");

    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());

}
