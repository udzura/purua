use crate::state::{LuaError, LuaResult};
use crate::{function::LuaFunction, table::LuaTable};

use std::{fmt, rc::Rc};

#[allow(dead_code)]
#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(i64),
    LuaString(String),
    Table(Rc<LuaTable>),
    Function(LuaFunction),
}

macro_rules! assert_is_table {
    ($y:expr) => {
        match $y {
            Value::Table(rc) => Ok(rc),
            _ => Err(LuaError {
                message: format!("Asert this is a table: {:?}", $y),
            }),
        }
    };
}

impl Value {
    pub fn newtable() -> Self {
        let refc = Rc::new(LuaTable::empty());
        Value::Table(refc)
    }

    pub fn to_int(&self) -> Option<i64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            Value::LuaString(s) => Some(s.to_string()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        }
    }

    pub fn ensure_table(&self) -> LuaResult<Rc<LuaTable>> {
        let rc = assert_is_table!(self)?;
        Ok(Rc::clone(rc))
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => f.write_str("Value::Nil"),
            Value::Bool(b) => f.debug_tuple("Value::Bool").field(b).finish(),
            Value::Number(n) => f.debug_tuple("Value::Number").field(n).finish(),
            Value::LuaString(s) => f.debug_tuple("Value::LuaString").field(s).finish(),
            Value::Table(t) => f.debug_tuple("Value::LuaTable").field(t.as_ref()).finish(),
            Value::Function(_) => f.write_str("Value::Function(LuaFn)"),
        }
    }
}
