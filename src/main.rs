#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate combine;

use std::env;
use std::fs::File;
use std::io::{self, Read};

use combine::parser::char::spaces;
use combine::stream::position;
use combine::EasyParser;

use env_logger;
use log::*;

mod parser;
mod state;
use state::LuaState;
mod eval;
mod prelude;
mod value;

fn main() {
    let mut builder = env_logger::Builder::from_env("PULUA_LOG");
    builder.init();
    match env::args().nth(1) {
        Some(file) => {
            let f = File::open(file).expect("Cannot open file");
            io2main(f)
        }
        None => io2main(io::stdin()),
    };
}

fn io2main<R>(mut read: R)
where
    R: Read,
{
    let mut text = String::new();
    read.read_to_string(&mut text).expect("read from IO failed");

    match do_main(text.as_str()) {
        Ok(_) => info!("Purua exited successfully"),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    //let mut parser = myparser();
    let mut l = LuaState::new(65535);
    prelude::prelude(&mut l);

    let mut parser = (spaces(), parser::chunk());

    let pos = position::Stream::new(text);
    let res = parser.easy_parse(pos)?.0;
    let chunk = res.1;
    debug!("parsed: {:?}", &chunk);

    eval::eval_chunk(&mut l, chunk.as_ref())?;
    //l.assign_global("foo", Value::LuaString("buz".to_string()));

    // // calling print()
    // let ret = l.global_funcall1(
    //     "print",
    //     Value::LuaString("Hello, Purua! This is arguement you specified\n".to_string()),
    // )?;
    // eprintln!("return value of print(): {:?}", ret);

    // // calling fib()
    // let ret = l.global_funcall1("fib", Value::Number(4))?;
    // eprintln!("return value of fib(4): {:?}", ret);

    // let ret = l.global_funcall1("fib", Value::Number(8))?;
    // eprintln!("return value of fib(8): {:?}", ret);

    // let ret = l.global_funcall1("fib", Value::Number(12))?;
    // eprintln!("return value of fib(12): {:?}", ret);

    // let ret = l.global_funcall1("fib", Value::Number(30))?;
    // eprintln!("return value of fib(30): {:?}", ret);

    Ok(())
}
