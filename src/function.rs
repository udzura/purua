use crate::eval::eval_block;
use crate::parser::Rule;
use crate::state::{LuaError, LuaState};
pub type LuaFn = fn(&mut LuaState) -> Result<i32, LuaError>;

#[derive(Clone)]
pub struct FunctionProto {
    pub parameters_nr: u8,
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

    pub fn from_code(params: &Rule, block: &Rule) -> Self {
        let proto = FunctionProto {
            parameters_nr: 1,
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
            let v = eval_block(l, self.proto.as_ref().unwrap().code.as_ref())?;
            l.returns(v);
            Ok(1)
        }
    }
}
