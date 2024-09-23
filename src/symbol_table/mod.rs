use std::collections::HashMap;

use crate::code_generation::Address;

#[derive(Clone, Debug)]
pub enum DataType {
    Boolean,
    Number,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub location: Option<Address>,
    pub data_type: DataType,
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    temp_count: u32,
    for_count: u32,
    if_count: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        let table = HashMap::new();
        Self {
            table,
            temp_count: 0,
            for_count: 0,
            if_count: 0,
        }
    }

    pub fn get(&self, key: &String) -> Option<Symbol> {
        self.table.get(key).cloned()
    }

    pub fn insert(&mut self, key: String, value: Symbol) -> Option<Symbol> {
        self.table.insert(key, value)
    }

    pub fn new_temp(&mut self) -> u32 {
        self.temp_count += 1;
        self.temp_count
    }

    pub fn new_if(&mut self) -> u32 {
        self.if_count += 1;
        self.if_count
    }

    pub fn new_for(&mut self) -> u32 {
        self.for_count += 1;
        self.for_count
    }
}
