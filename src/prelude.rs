use crate::state::{LuaError, LuaState};
use crate::value::Value;

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

pub fn prelude(l: &mut LuaState) {
    // register fn
    l.register_global_fn("print", lua_print);
    l.register_global_fn("fib", lua_fib);
    l.register_global_fn("globalset", lua_global_set);
    l.register_global_fn("globalget", lua_global_get);
}
