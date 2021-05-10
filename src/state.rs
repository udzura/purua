use crate::value::*;
use crate::{function::*, parser::Rule};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LuaError {
    pub message: String,
}
pub type LuaResult<T> = Result<T, LuaError>;

impl std::fmt::Display for LuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VM error: {}", self.message)
    }
}
impl std::error::Error for LuaError {}

pub struct Global {
    pub global: HashMap<String, Value>,
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

    #[allow(dead_code)]
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
        let value = &self.array[pos];
        value.to_int().ok_or(LuaError {
            message: "TypeError: cannot cast into int".to_string(),
        })
    }

    pub fn to_string(&self, pos: usize) -> Result<String, LuaError> {
        let value = &self.array[pos];
        value.to_string().ok_or(LuaError {
            message: "TypeError: cannot cast into str".to_string(),
        })
    }

    pub fn to_value(&self, pos: usize) -> Result<Value, LuaError> {
        Ok((&self.array[pos]).to_owned())
    }
}

pub struct LuaState {
    pub g: Global,
    pub reg: Registry,
    pub frame_stack: Vec<CallFrame>,
}

impl LuaState {
    pub fn new(reg_size: usize) -> Self {
        let global = HashMap::new();
        let g = Global { global };
        let reg = Registry {
            array: Vec::with_capacity(reg_size),
            top: 0,
            max_size: reg_size,
        };
        let frame_stack = Vec::new();

        Self {
            g,
            reg,
            frame_stack,
        }
    }

    pub fn arg_index2pos(&self, pos: usize) -> LuaResult<usize> {
        Ok(self.ensure_current_frame()?.local_base + pos - 1)
    }

    pub fn arg_int(&self, pos: usize) -> Result<i64, LuaError> {
        self.reg.to_int(self.arg_index2pos(pos)?)
    }

    pub fn arg_string(&self, pos: usize) -> Result<String, LuaError> {
        self.reg.to_string(self.arg_index2pos(pos)?)
    }

    pub fn arg_value(&self, pos: usize) -> Result<Value, LuaError> {
        self.reg.to_value(self.arg_index2pos(pos)?)
    }

    pub fn assign_global(&mut self, name: impl Into<String>, value: Value) {
        let name: String = name.into();
        if self.g.global.contains_key(&name) {
            self.g.global.remove(&name);
        }
        self.g.global.insert(name, value);
    }

    pub fn assign_local(&mut self, name: impl Into<String>, value: Value) {
        if let Some(_) = self.current_frame() {
            let name: String = name.into();
            let idx = self.reg.push(value) - 1;
            self.frame_stack.last_mut().unwrap().env.insert(name, idx);
        }
    }

    pub fn get_global(&self, name: impl Into<String>) -> Option<Value> {
        let name: String = name.into();
        self.g.global.get(&name).map(|v| v.clone())
    }

    pub fn register_global_fn(&mut self, name: impl Into<String>, func: LuaFn) {
        let name: String = name.into();
        self.g
            .global
            .insert(name, Value::Function(LuaFunction::from_fn(func)));
    }

    pub fn register_global_code(
        &mut self,
        name: impl Into<String>,
        params: Vec<String>,
        block: &Rule,
    ) {
        let name: String = name.into();
        self.g
            .global
            .insert(name, Value::Function(LuaFunction::from_code(params, block)));
    }

    pub fn start_block_raw(&mut self) -> usize {
        let oldtop = self.reg.top;
        let frame = CallFrame {
            args_nr: 0,
            ret_nr: 0,
            env: Default::default(),
            to_return: false,
            local_base: oldtop,
        };
        self.frame_stack.push(frame);
        oldtop
    }

    pub fn end_block_raw(&mut self, oldtop: usize) -> LuaResult<()> {
        self.frame_stack.pop();

        while oldtop < self.reg.top {
            let _ = self.reg.ensure_pop()?;
        }
        Ok(())
    }

    pub fn funcall(&mut self, func: Value, params: Vec<Value>) -> LuaResult<Vec<Value>> {
        let oldtop = self.reg.top;
        let params_n = params.len();
        for arg in params.into_iter().rev() {
            self.reg.push(arg);
        }

        let func = if let Value::Function(func) = func {
            func.clone()
        } else {
            return Err(self.error(format!("Specified value is not func {:?}", func)));
        };
        let retnr = func.do_call((self,))?;

        let mut ret = Vec::with_capacity(params_n);
        if retnr > 0 {
            for _ in 0..retnr {
                ret.push(self.reg.ensure_pop()?);
            }
            ret = ret.into_iter().rev().collect();
        }

        while oldtop < self.reg.top {
            let _ = self.reg.ensure_pop()?; // remove arg from stack - 1 time
        }

        Ok(ret)
    }

    pub fn global_funcall1(
        &mut self,
        name: impl Into<String>,
        arg1: Value,
    ) -> Result<Value, LuaError> {
        let name: String = name.into();
        let oldtop = self.reg.top;
        let _params_n = 1;
        self.reg.push(arg1);
        let func = {
            let g = &self.g;
            let val = g
                .global
                .get(&name)
                .ok_or(self.error(format!("Specified func {} not found", name)))?;

            if let Value::Function(func) = val {
                func.clone()
            } else {
                return Err(self.error(format!("Specified name {} is not func {:?}", name, val)));
            }
        };

        let retnr = func.do_call((self,))?;
        // if oldtop + params_n + retnr as usize != self.reg.top {
        //     return Err(self.error(format!("func {} should be return {} values", name, retnr)));
        // }

        // TODO: multireturn
        let vret = if retnr == 1 {
            self.reg.ensure_pop()? // get function return value
        } else {
            Value::Nil
        };
        while oldtop < self.reg.top {
            let _ = self.reg.ensure_pop()?; // remove arg from stack - 1 time
        }

        Ok(vret)
    }

    pub fn process_op(
        &self,
        op: &combine::lib::primitive::char,
        lvalue: Value,
        rvalue: Value,
    ) -> Result<Value, LuaError> {
        match (lvalue, rvalue) {
            (Value::Number(n), Value::Number(m)) => {
                self.process_op_number(op, n.to_owned(), m.to_owned())
            }
            (Value::Bool(n), Value::Bool(m)) => {
                self.process_op_bool(op, n.to_owned(), m.to_owned())
            }
            (Value::LuaString(n), Value::LuaString(m)) => self.process_op_str(op, &n, &m),
            _ => Err(self.error("type error")),
        }
    }

    pub fn process_op_number(
        &self,
        op: &combine::lib::primitive::char,
        l: i64,
        r: i64,
    ) -> Result<Value, LuaError> {
        let ret = match op {
            '+' => Value::Number(l + r),
            '-' => Value::Number(l - r),
            '*' => Value::Number(l * r),
            '/' => Value::Number(l / r),
            'l' => Value::Bool(l <= r),
            '<' => Value::Bool(l < r),
            'g' => Value::Bool(l >= r),
            '>' => Value::Bool(l > r),
            'e' => Value::Bool(l == r),
            'n' => Value::Bool(l != r),
            _ => return Err(self.error("unsupported op")),
        };
        Ok(ret)
    }

    pub fn process_op_bool(
        &self,
        op: &combine::lib::primitive::char,
        l: bool,
        r: bool,
    ) -> Result<Value, LuaError> {
        let ret = match op {
            '&' => Value::Bool(l && r),
            '|' => Value::Bool(l || r),
            _ => return Err(self.error("unsupported op")),
        };
        Ok(ret)
    }

    pub fn process_op_str(
        &self,
        op: &combine::lib::primitive::char,
        l: &str,
        r: &str,
    ) -> Result<Value, LuaError> {
        let ret = match op {
            'e' => Value::Bool(l == r),
            'n' => Value::Bool(l != r),
            _ => return Err(self.error("unsupported op")),
        };
        Ok(ret)
    }

    pub fn current_frame(&self) -> Option<&CallFrame> {
        self.frame_stack.last()
    }

    pub fn ensure_current_frame(&self) -> LuaResult<&CallFrame> {
        self.current_frame()
            .ok_or(self.error("not calledin function"))
    }

    pub fn has_local_name(&self, name: impl Into<String>) -> bool {
        let name: String = name.into();
        self.current_frame()
            .map_or(false, |f| f.env.contains_key(&name))
    }

    pub fn get_local(&self, name: impl Into<String>) -> Option<Value> {
        let name: String = name.into();
        let idx = self.current_frame()?.env.get(&name)?.to_owned();
        (&self.reg.array[idx]).clone().into()
    }

    pub fn set_to_return(&mut self, to_return: bool) {
        let mut f = self.frame_stack.last_mut().unwrap();
        f.to_return = to_return;
    }

    pub fn to_return(&mut self) -> bool {
        match self.current_frame() {
            Some(f) => f.to_return,
            None => false,
        }
    }

    // TODO: this should be called after params are all refered
    pub fn returns(&mut self, retval: Value) {
        self.reg.push(retval);
    }

    pub fn error(&self, msg: impl Into<String>) -> LuaError {
        LuaError {
            message: msg.into(),
        }
    }
}
