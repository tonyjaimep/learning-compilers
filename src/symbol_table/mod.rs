use std::collections::HashMap;

pub enum SymbolType {
    Lexeme,
}

pub struct Symbol {
    symbol_type: SymbolType,
}

pub type SymbolTable = HashMap<String, Symbol>;
