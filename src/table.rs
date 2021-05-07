use std::collections::HashMap;

use crate::value::Value;

#[derive(Clone, Default, Debug)]
pub struct LuaTable {
    pub metatable: Box<LuaTable>,

    pub vec: Vec<Value>,
    pub strdict: HashMap<String, Value>,
}
