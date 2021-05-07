use std::{cell::RefCell, collections::HashMap};

use crate::value::Value;

#[derive(Debug)]
pub struct LuaTable {
    pub metatable: Option<RefCell<Box<LuaTable>>>,

    pub vec: RefCell<Vec<Value>>,
    pub strdict: RefCell<HashMap<String, Value>>,
}

impl LuaTable {
    pub fn empty() -> Self {
        let mt = None; // in the future...
        let vec = Vec::new();
        let strdict = HashMap::new();

        LuaTable {
            metatable: mt,
            vec: RefCell::new(vec),
            strdict: RefCell::new(strdict),
        }
    }
}
