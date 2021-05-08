use crate::state::{LuaError, LuaResult, LuaState};
use crate::value::Value;

fn lua_print(l: &mut LuaState) -> Result<i32, LuaError> {
    let v = l.arg_string(1)?;
    print!("{}", v);
    Ok(0)
}

fn lua_pairs(l: &mut LuaState) -> LuaResult<i32> {
    let tbl = l.arg_value(1)?;

    l.returns(l.get_global("next").unwrap());
    l.returns(tbl);
    l.returns(Value::Nil);
    Ok(3)
}

fn lua_next(l: &mut LuaState) -> LuaResult<i32> {
    let tbl = l.arg_value(1)?;
    let t = tbl.ensure_table()?;
    let index = l.arg_value(2)?;
    match index {
        Value::Nil => {
            l.returns(Value::Number(1));
            l.returns(t.vec.borrow()[0].clone());
            Ok(2)
        }
        Value::Number(i) => {
            if t.vec.borrow().len() as i64 <= i {
                l.returns(Value::Nil);
                Ok(1)
            } else {
                l.returns(Value::Number(i + 1));
                let index = i as usize;
                l.returns(t.vec.borrow()[index].clone());
                Ok(2)
            }
        }
        _ => Err(l.error(format!("invalid argument {:?}", index))),
    }
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
    } else {
        println!("foo is not set");
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

fn lua_set_array(l: &mut LuaState) -> LuaResult<i32> {
    let v = Value::newtable();
    let t = v.ensure_table()?;
    let mut t = t.vec.borrow_mut();
    t.push(Value::Number(1));
    t.push(Value::Number(2));
    t.push(Value::Number(3));

    l.assign_global("myarray", v);
    Ok(0)
}

fn lua_update_array(l: &mut LuaState) -> LuaResult<i32> {
    let v = l.get_global("myarray").ok_or(l.error("Variable not set"))?;
    let t = v.ensure_table()?;
    let mut t = t.vec.borrow_mut();
    t.push(Value::Number(4));
    t.push(Value::Number(5));
    t.push(Value::Number(6));

    Ok(0)
}

fn lua_print_array(l: &mut LuaState) -> LuaResult<i32> {
    let v = l.get_global("myarray").ok_or(l.error("Variable not set"))?;
    let t = v.ensure_table()?;
    let t = t.vec.borrow();

    for elm in t.iter() {
        println!("elm: {:?}", elm);
    }

    Ok(0)
}

pub fn prelude(l: &mut LuaState) {
    // register fn
    l.register_global_fn("print", lua_print);
    l.register_global_fn("pairs", lua_pairs);
    l.register_global_fn("next", lua_next);

    l.register_global_fn("fib", lua_fib);
    l.register_global_fn("globalset", lua_global_set);
    l.register_global_fn("globalget", lua_global_get);

    l.register_global_fn("setarray", lua_set_array);
    l.register_global_fn("updatearray", lua_update_array);
    l.register_global_fn("printarray", lua_print_array);
}
