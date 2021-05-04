use crate::parser::*;
use crate::state::*;
use crate::value::Value;

use log::debug;

macro_rules! is_exact_rule1 {
    ($rule:path, $y:expr) => {
        match $y {
            $rule(val) => Ok(val),
            _ => Err(LuaError {
                message: format!("Invalid rule passed: {:?}", $y),
            }),
        }
    };
}

macro_rules! is_exact_rule2 {
    ($rule:path, $y:expr) => {
        match $y {
            $rule(val1, val2) => Ok((val1, val2)),
            _ => Err(LuaError {
                message: format!("Invalid rule passed: {:?}", $y),
            }),
        }
    };
}

pub fn eval_exp(_l: &mut LuaState, exp: &Rule) -> Result<Value, LuaError> {
    let exp_: &Box<Rule> = is_exact_rule1!(Rule::Exp, exp)?;
    match exp_.as_ref() {
        Rule::Numeral(n) => return Ok(Value::Number(n.to_owned() as i64)),
        Rule::LiteralString(s) => return Ok(Value::LuaString(s.to_string())),
        _ => Err(LuaError {
            message: format!("Unsupported rule: {:?}", exp_),
        }),
    }
}

pub fn eval_funcall(l: &mut LuaState, fc: &Rule) -> Result<Value, LuaError> {
    let (name, args) = is_exact_rule2!(Rule::FunctionCall, fc)?;
    let exp = is_exact_rule1!(Rule::Args, args.as_ref())?;
    let name = is_exact_rule1!(Rule::Symbol, name.as_ref())?;

    let arg1v = eval_exp(l, exp)?;
    debug!("get param {} {:?}", name, &arg1v);
    let ret = l.global_funcall1(name, arg1v)?;

    Ok(ret)
}

pub fn eval_chunk(l: &mut LuaState, chunk: &Rule) -> Result<(), LuaError> {
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

pub fn eval_stat(l: &mut LuaState, stat: &Rule) -> Result<(), LuaError> {
    match stat {
        Rule::Stat(kind, a, b, _c, _d, _e) => {
            match kind {
                StatKind::Sep => { /* nop */ }
                StatKind::VarAssign => {
                    let var = is_exact_rule1!(Rule::Var, a.as_ref().unwrap().as_ref())?;
                    let name = is_exact_rule1!(Rule::Symbol, var.as_ref())?;
                    let value = eval_exp(l, b.as_ref().unwrap())?;

                    l.assign_global(name, value);
                }
                StatKind::FunctionCall => {
                    eval_funcall(l, a.as_ref().unwrap())?;
                }
                _ => unimplemented!("Pull request is welcomed!"),
            }
            Ok(())
        }
        _ => Err(l.error("Not a stat")),
    }
}
