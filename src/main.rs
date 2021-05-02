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
use state::LuaState;
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

fn lua_print(l: &LuaState) -> i32 {
    let v = l.reg.borrow().to_string(1);
    match v {
        Ok(s) => {
            print!("{}", s);
            l.reg.borrow_mut().push(Value::Nil);
            1
        }
        Err(_) => -1,
    }
}

fn lua_fib(l: &LuaState) -> i32 {
    let v = l.reg.borrow().to_int(1);
    match v {
        Ok(v) => {
            if v <= 1 {
                l.reg.borrow_mut().push(Value::Number(1));
            } else {
                let mut r0 = 0;
                let mut r1 = 0;

                l.reg.borrow_mut().push(Value::Number(v - 2));
                let g = l.g.borrow();
                let val = g.global.get("fib").expect("function not found").clone();

                if let Value::Function(f) = val {
                    let _ = f.call((l,));
                    if let Value::Number(r) = l.reg.borrow_mut().pop().unwrap() {
                        r0 = r;
                    }
                }
                l.reg.borrow_mut().pop();

                l.reg.borrow_mut().push(Value::Number(v - 1));
                let g = l.g.borrow();
                let val = g.global.get("fib").expect("function not found").clone();

                if let Value::Function(f) = val {
                    let _ = f.call((l,));
                    if let Value::Number(r) = l.reg.borrow_mut().pop().unwrap() {
                        r1 = r;
                    }
                }
                l.reg.borrow_mut().pop();

                l.reg.borrow_mut().push(Value::Number(r0 + r1));
            }
            1
        }
        Err(_) => -1,
    }
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    //let mut parser = myparser();
    // let mut parser = (spaces(), parser::block());

    // let pos = position::Stream::new(text);
    // let res = parser.easy_parse(pos)?.0;
    // println!("parsed: {:?}", res.1);

    let l = LuaState::new(65535);

    l.g.borrow_mut()
        .global
        .insert("print", Value::Function(Box::new(lua_print)));
    l.g.borrow_mut()
        .global
        .insert("fib", Value::Function(Box::new(lua_fib)));

    // calling print()
    l.reg
        .borrow_mut()
        .push(Value::LuaString("Hello, Purua! This is from args\n"));
    let g = l.g.borrow();
    let val = g.global.get("print").expect("function not found").clone();

    if let Value::Function(f) = val {
        let _ = f.call((&l,));
        eprintln!("return value of print(): {:?}", l.reg.borrow_mut().pop());
    }
    l.reg.borrow_mut().pop(); // remove arg from stack

    // calling fib(8)
    l.reg.borrow_mut().push(Value::Number(8));
    let g = l.g.borrow();
    let val = g.global.get("fib").expect("function not found").clone();

    if let Value::Function(f) = val {
        let _ = f.call((&l,));
        eprintln!("return value of fib(8): {:?}", l.reg.borrow_mut().pop());
    }
    l.reg.borrow_mut().pop(); // remove arg from stack

    Ok(())
}
