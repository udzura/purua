#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate combine;

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
    print!("Hello, Purua!\n");
    1
}

fn do_main<'a>(text: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
    //let mut parser = myparser();
    // let mut parser = (spaces(), parser::block());

    // let pos = position::Stream::new(text);
    // let res = parser.easy_parse(pos)?.0;
    // println!("parsed: {:?}", res.1);

    let mut l = LuaState::new(65535);
    l.g.global
        .insert("print", Value::Function(Box::new(lua_print)));

    if let Value::Function(f) = l.g.global.get("print").expect("function not found") {
        let _ = f.call((&l,));
    }
    Ok(())
}
