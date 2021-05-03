use crate::value::*;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct LuaError {
    pub message: String,
}

impl std::fmt::Display for LuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VM error: {}", self.message)
    }
}
impl std::error::Error for LuaError {}

pub struct Global<'a> {
    pub global: HashMap<&'a str, Value>,
}

pub struct Registry {
    pub array: Vec<Value>,
    pub top: usize,
    pub max_size: usize,
}

impl Registry {
    pub fn push(&mut self, value: Value) -> usize {
        self.array.push(value);
        self.top += 1;
        self.top
    }

    pub fn last(&self) -> Option<&Value> {
        self.array.get(self.top - 1)
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.top -= 1;
        self.array.pop()
    }

    pub fn ensure_pop(&mut self) -> Result<Value, LuaError> {
        self.pop().ok_or(LuaError {
            message: "Cannot find value from regisrty, maybe empty".to_string(),
        })
    }

    pub fn to_int(&self, pos: usize) -> Result<i64, LuaError> {
        let idx = self.top - pos;
        let value = &self.array[idx];
        value.to_int().ok_or(LuaError {
            message: "TypeError: cannot cast into int".to_string(),
        })
    }

    pub fn to_string(&self, pos: usize) -> Result<String, LuaError> {
        let idx = self.top - pos;
        let value = &self.array[idx];
        value.to_string().ok_or(LuaError {
            message: "TypeError: cannot cast into str".to_string(),
        })
    }
}

pub struct LuaState<'a> {
    pub g: RefCell<Global<'a>>,
    pub reg: RefCell<Registry>,
}

impl<'a> LuaState<'a> {
    pub fn new(reg_size: usize) -> Self {
        let global = HashMap::new();
        let g = Global { global };
        let reg = Registry {
            array: Vec::with_capacity(reg_size),
            top: 0,
            max_size: reg_size,
        };

        Self {
            g: RefCell::new(g),
            reg: RefCell::new(reg),
        }
    }

    pub fn arg_int(&self, pos: usize) -> Result<i64, LuaError> {
        self.reg.borrow().to_int(pos)
    }

    pub fn arg_string(&self, pos: usize) -> Result<String, LuaError> {
        self.reg.borrow().to_string(pos)
    }

    pub fn assign_global(&self, name: &'a str, value: Value) {
        let mut g = self.g.borrow_mut();
        if g.global.contains_key(name) {
            g.global.remove(name);
        }
        g.global.insert(name, value);
    }

    pub fn get_global(&self, name: &'a str) -> Option<Value> {
        self.g.borrow().global.get(name).map(|v| match v {
            Value::Nil => Value::Nil,
            Value::Bool(b) => Value::Bool(b.to_owned()),
            Value::Number(n) => Value::Number(n.to_owned()),
            Value::LuaString(s) => Value::LuaString(s.clone()),
            Value::Function(f) => Value::Function(Rc::clone(f)),
        })
    }

    pub fn register_global_fn(
        &self,
        name: &'a str,
        func: impl Fn(&LuaState) -> Result<i32, LuaError> + 'static,
    ) {
        self.g
            .borrow_mut()
            .global
            .insert(name, Value::Function(Rc::new(func)));
    }

    pub fn global_funcall1(&self, name: &'_ str, arg1: Value) -> Result<Value, LuaError> {
        self.reg.borrow_mut().push(arg1);

        let g = self.g.borrow();
        let val = g
            .global
            .get(name)
            .ok_or(self.error(format!("Specified func {} not found", name)))?;

        let oldtop = self.reg.borrow().top;
        let mut retnr = 0;
        let func = if let Value::Function(func) = val {
            Rc::clone(func)
        } else {
            return Err(self.error(format!("Specified name {} is not func {:?}", name, val)));
        };
        std::mem::drop(g); // Release g's borrowing

        retnr = func.call((self,))?;
        if oldtop + retnr as usize != self.reg.borrow().top {
            return Err(self.error(format!("func {} should be return {} values", name, retnr)));
        }

        // TODO: multireturn
        let vret = if retnr == 1 {
            self.reg.borrow_mut().ensure_pop()? // get function return value
        } else {
            Value::Nil
        };
        let _ = self.reg.borrow_mut().ensure_pop()?; // remove arg from stack - 1 time

        Ok(vret)
    }

    pub fn returns(&self, retval: Value) {
        self.reg.borrow_mut().push(retval);
    }

    pub fn error(&self, msg: impl Into<String>) -> LuaError {
        LuaError {
            message: msg.into(),
        }
    }
}
