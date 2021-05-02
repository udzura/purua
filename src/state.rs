use crate::value::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LuaError {
    pub message: &'static str,
}

impl std::fmt::Display for LuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VM error: {}", self.message)
    }
}

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

    pub fn pop(&mut self) -> Option<Value> {
        self.top -= 1;
        self.array.pop()
    }

    pub fn to_int(&self, pos: usize) -> Result<i64, LuaError> {
        let idx = self.top - pos;
        let value = &self.array[idx];
        value.to_int().ok_or(LuaError {
            message: "TypeError: cannot cast into int",
        })
    }

    pub fn to_string(&self, pos: usize) -> Result<String, LuaError> {
        let idx = self.top - pos;
        let value = &self.array[idx];
        value.to_string().ok_or(LuaError {
            message: "TypeError: cannot cast into int",
        })
    }
}

pub struct LuaState<'a, 'b> {
    pub g: RefCell<Global<'a, 'b>>,
    pub reg: RefCell<Registry<'b>>,
}

impl<'a, 'b> LuaState<'a, 'b> {
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
}
