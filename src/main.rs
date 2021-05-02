#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate combine;

use std::cell::RefCell;
use std::env;

// use combine::parser::char::spaces;
// use combine::stream::position;
// use combine::EasyParser;

mod parser;
mod state;
use state::{LuaError, LuaState};
mod value;
use value::Value;

fn main() {
    let text = env::args().nth(1).unwrap_or("sample = 1".to_string());
    match do_main(&text) {
        Ok(_) => eprintln!("Purua exited successfully"),
        Err(err) => {
            eprintln!("{}", err);
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
    // let mut parser = (spaces(), parser::block());

    // let pos = position::Stream::new(text);
    // let res = parser.easy_parse(pos)?.0;
    // println!("parsed: {:?}", res.1);

    let l = LuaState::new(65535);

    // register fn
    l.register_global_fn("print", lua_print);
    l.register_global_fn("fib", lua_fib);

    // calling print()
    let ret = l.global_funcall1(
        "print",
        Value::LuaString("Hello, Purua! This is arguement you specified\n"),
    )?;
    eprintln!("return value of print(): {:?}", ret);

    // calling fib()
    let ret = l.global_funcall1("fib", Value::Number(4))?;
    eprintln!("return value of fib(4): {:?}", ret);

    let ret = l.global_funcall1("fib", Value::Number(8))?;
    eprintln!("return value of fib(8): {:?}", ret);

    let ret = l.global_funcall1("fib", Value::Number(12))?;
    eprintln!("return value of fib(12): {:?}", ret);

    let ret = l.global_funcall1("fib", Value::Number(30))?;
    eprintln!("return value of fib(30): {:?}", ret);

    Ok(())
}
