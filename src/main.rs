extern crate combine;

use std::env;

use combine::parser::char::spaces;
use combine::stream::position;
use combine::EasyParser;

mod parser;

fn main() {
    let text = env::args().nth(1).unwrap_or("sample = 1".to_string());
    match do_main(&text) {
        Ok(_) => println!("OK"),
        Err(err) => println!("{}", err),
    };
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    //let mut parser = myparser();
    let mut parser = (spaces(), parser::block());

    let pos = position::Stream::new(text);
    let res = parser.easy_parse(pos)?.0;
    println!("parsed: {:?}", res.1);

    Ok(())
}
