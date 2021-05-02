use crate::value::*;
use std::cell::RefCell;
use std::collections::HashMap;

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

pub struct Global<'a, 'b> {
    pub global: HashMap<&'a str, Value<'b>>,
}

pub struct Registry<'a> {
    pub array: Vec<Value<'a>>,
    pub top: usize,
    pub max_size: usize,
}

impl<'a> Registry<'a> {
    pub fn push(&mut self, value: Value<'a>) -> usize {
        self.array.push(value);
        self.top += 1;
        self.top
    }

    pub fn top(&self) -> Option<&Value> {
        self.array.get(self.top - 1)
    }

    pub fn pop(&mut self) -> Option<Value<'a>> {
        self.top -= 1;
        self.array.pop()
    }

    pub fn ensure_pop(&mut self) -> Result<Value<'a>, LuaError> {
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

pub struct LuaState<'a, 'b, 'c> {
    pub g: RefCell<Global<'a, 'b>>,
    pub reg: RefCell<Registry<'c>>,
}

impl<'a, 'b, 'c> LuaState<'a, 'b, 'c> {
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

    pub fn register_global_fn(&self, name: &'a str, func: impl Fn(&LuaState) -> i32 + 'static) {
        self.g
            .borrow_mut()
            .global
            .insert(name, Value::Function(Box::new(func)));
    }

    pub fn global_funcall1(&self, name: &'a str, arg1: Value<'c>) -> Result<Value<'c>, LuaError> {
        self.reg.borrow_mut().push(arg1);

        let g = self.g.borrow();
        let val = g
            .global
            .get(name)
            .ok_or(LuaError {
                message: format!("Specified func {} not found", name),
            })?
            .clone();

        if let Value::Function(func) = val {
            let _ = func.call((self,));
        } else {
            return Err(LuaError {
                message: format!("Specified name {} is not func", name),
            });
        }

        let vret = self.reg.borrow_mut().ensure_pop()?; // get function return value
        let _ = self.reg.borrow_mut().ensure_pop()?; // remove arg from stack - 1 time

        Ok(vret)
    }
}
