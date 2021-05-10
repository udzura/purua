// use log::*;
use std::collections::HashMap;

use crate::eval::eval_block;
use crate::parser::Rule;
use crate::state::{LuaError, LuaState};
pub type LuaFn = fn(&mut LuaState) -> Result<i32, LuaError>;

#[derive(Clone)]
pub struct FunctionProto {
    pub parameters: Vec<String>,
    pub params_nr: i32,
    pub code: Box<Rule>,
}

#[derive(Clone)]
pub struct CallFrame {
    pub env: HashMap<String, usize>,
    pub to_return: bool,
    pub args_nr: usize,
    pub ret_nr: usize,
    pub local_base: usize,
}

#[derive(Clone)]
pub struct LuaFunction {
    is_global: bool,
    pub proto: FunctionProto,
    pub luafn: Option<LuaFn>,
}

impl LuaFunction {
    pub fn from_fn(func: LuaFn) -> Self {
        let proto = FunctionProto {
            params_nr: -1,
            parameters: vec![],
            code: Box::new(Rule::Nop),
        };

        LuaFunction {
            is_global: true,
            proto: proto,
            luafn: Some(func),
        }
    }

    pub fn from_code(params: Vec<String>, block: &Rule) -> Self {
        let proto = FunctionProto {
            params_nr: params.len() as i32,
            parameters: params,
            code: Box::new(block.to_owned()),
        };

        LuaFunction {
            is_global: true,
            proto: proto,
            luafn: None,
        }
    }

    pub fn do_call(&self, args: (&mut LuaState,)) -> Result<i32, LuaError> {
        let l = args.0;

        if let Some(luafn) = self.luafn {
            // Use fn_traits in the future
            let args_nr = self.proto.params_nr as usize;
            let frame = CallFrame {
                args_nr: args_nr,
                ret_nr: 1,
                env: Default::default(),
                to_return: false,
                local_base: l.reg.top - args_nr,
            };
            l.frame_stack.push(frame);
            let rn = luafn(l)?;
            l.frame_stack.pop();
            Ok(rn)
        } else {
            let args_nr = self.proto.params_nr as usize;
            let mut frame = CallFrame {
                args_nr: args_nr,
                ret_nr: 1,
                env: Default::default(),
                to_return: false,
                local_base: l.reg.top - args_nr,
            };

            for (i, name) in self.proto.parameters.iter().enumerate() {
                let i = i + 1;
                let idx = l.reg.top - i;
                frame.env.insert(name.to_string(), idx);
            }
            l.frame_stack.push(frame);

            let v = eval_block(l, self.proto.code.as_ref())?;

            l.frame_stack.pop();

            l.returns(v);
            Ok(1)
        }
    }
}
