use crate::parser::*;
use crate::state::*;
use crate::value::Value;

pub fn funcall_to_name_args(fc: &Rule) -> Result<(String, Value), LuaError> {
    if let Rule::FunctionCall(name, args) = fc {
        if let Rule::Symbol(name) = name.as_ref() {
            if let Rule::Args(exp) = args.as_ref() {
                if let Rule::Exp(val) = exp.as_ref() {
                    let retname = name.to_string();
                    match val.as_ref() {
                        Rule::Numeral(n) => {
                            return Ok((retname, Value::Number(n.to_owned() as i64)))
                        }
                        Rule::LiteralString(s) => return Ok((retname, Value::LuaString(&s))),
                        _ => {}
                    }
                }
            }
        }
    }
    Err(LuaError {
        message: "Invalid ast form".to_string(),
    })
}

pub fn eval_chunk<'r>(l: &LuaState<'_, '_, 'r>, chunk: &'r Rule) -> Result<(), LuaError> {
    match chunk {
        Rule::Chunk(stats) => {
            for stat in stats.into_iter() {
                eval_stat(l, stat.as_ref());
            }
            Ok(())
        }
        _ => Err(l.error("Not a chunk")),
    }
}

pub fn eval_stat<'r>(l: &LuaState<'_, '_, 'r>, stat: &'r Rule) -> Result<(), LuaError> {
    match stat {
        Rule::Stat(kind, a, _b, _c, _d, _e) => {
            match kind {
                StatKind::Sep => { /* nop */ }
                StatKind::VarAssign => {
                    todo!("assign to global")
                }
                StatKind::FunctionCall => {
                    let (name, arg1) = funcall_to_name_args(a.as_ref().unwrap())?;
                    l.global_funcall1(&name, arg1)?;
                }
                _ => unimplemented!("Pull request is welcomed!"),
            }
            Ok(())
        }
        _ => Err(l.error("Not a stat")),
    }
}
