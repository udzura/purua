use crate::parser::Rule;
use crate::state::{LuaError, LuaState};
use crate::{eval::eval_block, value::Value};
pub type LuaFn = fn(&mut LuaState) -> Result<i32, LuaError>;

#[derive(Clone)]
pub struct FunctionProto {
    pub parameters: Vec<String>,
    pub code: Box<Rule>,
}

#[derive(Clone)]
pub struct LuaFunction {
    is_global: bool,
    pub proto: Option<FunctionProto>,
    pub luafn: Option<LuaFn>,
}

impl LuaFunction {
    pub fn from_fn(func: LuaFn) -> Self {
        LuaFunction {
            is_global: true,
            proto: None,
            luafn: Some(func),
        }
    }

    pub fn from_code(params: Vec<String>, block: &Rule) -> Self {
        let proto = FunctionProto {
            parameters: params,
            code: Box::new(block.to_owned()),
        };

        LuaFunction {
            is_global: true,
            proto: Some(proto),
            luafn: None,
        }
    }

    pub fn do_call(&self, args: (&mut LuaState,)) -> Result<i32, LuaError> {
        if let Some(luafn) = self.luafn {
            luafn.call(args)
        } else {
            let l = args.0;
            let mut i: usize = 0;
            for name in self.proto.as_ref().unwrap().parameters.iter() {
                i += 1;
                let idx = l.reg.top - i;
                let value = l.reg.array[idx].clone();
                l.assign_global(name, value);
            }

            let v = eval_block(l, self.proto.as_ref().unwrap().code.as_ref())?;
            l.returns(v);

            for name in self.proto.as_ref().unwrap().parameters.iter() {
                l.assign_global(name, Value::Nil);
            }

            Ok(1)
        }
    }
}
