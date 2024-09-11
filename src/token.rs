pub enum TokenType {
    Constant,
    Operator,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: char,
}
