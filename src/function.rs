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

    pub fn do_call(&self, args: (&mut LuaState,)) -> Result<i32, LuaError> {
        if let Some(luafn) = self.luafn {
            luafn.call(args)
        } else {
            Err(args.0.error("Code defined func not ywe implemented"))
        }
    }
}
