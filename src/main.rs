#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate combine;

use std::cell::RefCell;
use std::env;

use combine::parser::char::spaces;
use combine::stream::position;
use combine::EasyParser;

use env_logger;
use log::*;

mod parser;
mod state;
use state::{LuaError, LuaState};
mod value;
use value::Value;
mod eval;

fn main() {
    let mut builder = env_logger::Builder::from_env("PULUA_LOG");
    builder.init();

    let text = env::args()
        .nth(1)
        .unwrap_or("print(\"Hello, Purua!\\n\")".to_string());
    match do_main(&text) {
        Ok(_) => info!("Purua exited successfully"),
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    };
}

fn lua_print(l: &LuaState) -> Result<i32, LuaError> {
    let v = l.arg_string(1)?;
    print!("{}", v);
    Ok(0)
}

fn lua_fib(l: &LuaState) -> Result<i32, LuaError> {
    let v = l.arg_int(1)?;

    if v <= 1 {
        l.returns(Value::Number(1));
    } else {
        let mut r0 = 0;
        let mut r1 = 0;

        let ret = l.global_funcall1("fib", Value::Number(v - 2))?;
        if let Value::Number(r) = ret {
            r0 = r;
        }

        let ret = l.global_funcall1("fib", Value::Number(v - 1))?;
        if let Value::Number(r) = ret {
            r1 = r;
        }

        l.returns(Value::Number(r0 + r1));
    }
    Ok(1)
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    //let mut parser = myparser();
    let l = LuaState::new(65535);

    // register fn
    l.register_global_fn("print", lua_print);
    l.register_global_fn("fib", lua_fib);

    let mut parser = (spaces(), parser::chunk());

    let pos = position::Stream::new(text);
    let res = parser.easy_parse(pos)?.0;
    let chunk = res.1;
    // println!("parsed: {:?}", &chunk);

    eval::eval_chunk(&l, chunk.as_ref())?;

    // // calling print()
    // let ret = l.global_funcall1(
    //     "print",
    //     Value::LuaString("Hello, Purua! This is arguement you specified\n"),
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
