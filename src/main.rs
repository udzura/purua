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
use state::{LuaError, LuaState};
mod value;
use value::Value;
mod eval;

fn lua_print(l: &mut LuaState) -> Result<i32, LuaError> {
    let v = l.arg_string(1)?;
    print!("{}", v);
    Ok(0)
}

fn lua_global_set(l: &mut LuaState) -> Result<i32, LuaError> {
    let v = l.arg_string(1)?;
    println!("set foo={}", v);
    l.assign_global("foo", Value::LuaString(v));
    Ok(0)
}

fn lua_global_get(l: &mut LuaState) -> Result<i32, LuaError> {
    if let Some(v) = l.get_global("foo") {
        println!("get foo={:?}", v);
    }
    Ok(0)
}

fn lua_fib(l: &mut LuaState) -> Result<i32, LuaError> {
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

fn main() {
    let mut builder = env_logger::Builder::from_env("PULUA_LOG");
    builder.init();
    let result = match env::args().nth(1) {
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
    read.read_to_string(&mut text);

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

    // register fn
    l.register_global_fn("print", lua_print);
    l.register_global_fn("fib", lua_fib);
    l.register_global_fn("globalset", lua_global_set);
    l.register_global_fn("globalget", lua_global_get);
    l.assign_global("foo", Value::LuaString("bar".to_string()));

    let mut parser = (spaces(), parser::chunk());

    let pos = position::Stream::new(text);
    let res = parser.easy_parse(pos)?.0;
    let chunk = res.1;
    debug!("parsed: {:?}", &chunk);

    eval::eval_chunk(&mut l, chunk.as_ref())?;
    l.assign_global("foo", Value::LuaString("buz".to_string()));

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
