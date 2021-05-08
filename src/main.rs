extern crate combine;
extern crate purua;

use std::fs::File;
use std::io::{self, Read};

use combine::parser::char::spaces;
use combine::stream::position;
use combine::EasyParser;

use env_logger;
use log::*;
use structopt::StructOpt;

use purua::state::LuaState;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "file")]
    file: Option<String>,
    #[structopt(short = "e")]
    eval: Option<String>,
}

fn main() {
    let mut builder = env_logger::Builder::from_env("PULUA_LOG");
    builder.init();

    let args = Command::from_args();

    let ret = if let Some(eval) = args.eval {
        do_main(eval.as_bytes())
    } else if let Some(file) = args.file {
        let f = File::open(file).expect("Cannot open file");
        do_main(f)
    } else {
        do_main(io::stdin())
    };

    match ret {
        Ok(_) => info!("Purua exited successfully"),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}

fn do_main<R>(mut read: R) -> Result<(), purua::state::LuaError>
where
    R: Read,
{
    let mut l = LuaState::new(65535);

    let mut text = String::new();
    read.read_to_string(&mut text)
        .map_err(|e| l.error(format!("Reading text error: {}", e.to_string())))?;

    //let mut parser = myparser();
    purua::prelude::prelude(&mut l);

    let mut parser = (spaces(), purua::parser::chunk());

    let pos = position::Stream::new(text.as_str());
    let res = parser
        .easy_parse(pos)
        .map_err(|e| l.error(format!("Parse error: {}", e.to_string())))?
        .0;
    let chunk = res.1;
    debug!("parsed: {:?}", &chunk);

    purua::eval::eval_chunk(&mut l, chunk.as_ref())?;
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
