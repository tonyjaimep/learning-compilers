use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum DataType {
    Boolean,
    Number,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub data_type: DataType,
}

pub type SymbolTable = HashMap<String, Symbol>;
