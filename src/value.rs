use crate::state::LuaState;
use std::fmt;
pub type LuaFn = Box<dyn Fn(&LuaState) -> i32>;

pub enum Value<'a> {
    Nil,
    Bool(bool),
    Number(i64),
    LuaString(&'a str),
    Function(LuaFn),
}

impl Value<'_> {
    pub fn to_int(&self) -> Option<i64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            Value::LuaString(s) => Some(s.to_string()),
            _ => None,
        }
    }
}

impl fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => f.write_str("Value::Nil"),
            Value::Bool(b) => f.debug_tuple("Value::Bool").field(b).finish(),
            Value::Number(n) => f.debug_tuple("Value::Number").field(n).finish(),
            Value::LuaString(s) => f.debug_tuple("Value::LuaString").field(s).finish(),
            Value::Function(_) => f.write_str("Value::Function(LuaFn)"),
        }
    }
}
