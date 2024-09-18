use regex::Regex;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    If,
    Semicolon,
    For,
    ParenthesisOpening,
    ParenthesisClosing,
    OperatorMultiplication,
    OperatorDivision,
    OperatorAddition,
    OperatorSubtraction,
    OperatorIncrement,
    OperatorIncrementBy,
    OperatorDecrement,
    OperatorDecrementBy,
    OperatorAssignment,
    OperatorLessThan,
    OperatorLessThanOrEqual,
    OperatorGreaterThan,
    OperatorGreaterThanOrEqual,
    OperatorEqual,
    Constant,
    Identifier,
    CurlyOpening,
    CurlyClosing,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::If => "IF",
                TokenType::Semicolon => ";",
                TokenType::For => "FOR",
                TokenType::ParenthesisOpening => "(",
                TokenType::ParenthesisClosing => ")",
                TokenType::OperatorAssignment => "=",
                TokenType::OperatorMultiplication => "*",
                TokenType::OperatorDivision => "/",
                TokenType::OperatorSubtraction => "-",
                TokenType::OperatorAddition => "+",
                TokenType::OperatorIncrement => "++",
                TokenType::OperatorDecrement => "--",
                TokenType::OperatorIncrementBy => "+=",
                TokenType::OperatorDecrementBy => "-=",
                TokenType::OperatorLessThan => "LT",
                TokenType::OperatorLessThanOrEqual => "LTE",
                TokenType::OperatorGreaterThan => "GT",
                TokenType::OperatorGreaterThanOrEqual => "GTE",
                TokenType::OperatorEqual => "EQ",
                TokenType::CurlyOpening => "{",
                TokenType::CurlyClosing => "}",
                TokenType::Constant => "Constant",
                TokenType::Identifier => "Identifier",
                TokenType::EOF => "EOF",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Lexeme(String),
    Float(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<TokenValue>,
}

impl Token {
    pub fn is_binary_operator(self: &Self) -> bool {
        match self.token_type {
            TokenType::OperatorMultiplication
            | TokenType::OperatorDivision
            | TokenType::OperatorAddition
            | TokenType::OperatorSubtraction
            | TokenType::OperatorIncrementBy
            | TokenType::OperatorDecrementBy
            | TokenType::OperatorAssignment
            | TokenType::OperatorLessThan
            | TokenType::OperatorLessThanOrEqual
            | TokenType::OperatorGreaterThan
            | TokenType::OperatorGreaterThanOrEqual
            | TokenType::OperatorEqual => true,
            TokenType::OperatorIncrement | TokenType::OperatorDecrement => false,
            _ => panic!("Token is not operator: {}", self),
        }
    }
}

impl TryFrom<String> for Token {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let token_type = match value.as_str() {
            "for" => TokenType::For,
            "if" => TokenType::If,
            ";" => TokenType::Semicolon,
            "(" => TokenType::ParenthesisOpening,
            ")" => TokenType::ParenthesisClosing,
            "*" => TokenType::OperatorMultiplication,
            "/" => TokenType::OperatorDivision,
            "-" => TokenType::OperatorSubtraction,
            "--" => TokenType::OperatorDecrement,
            "-=" => TokenType::OperatorDecrementBy,
            "++" => TokenType::OperatorIncrement,
            "+=" => TokenType::OperatorIncrementBy,
            "+" => TokenType::OperatorAddition,
            ">" => TokenType::OperatorGreaterThan,
            "<" => TokenType::OperatorLessThan,
            ">=" => TokenType::OperatorGreaterThanOrEqual,
            "<=" => TokenType::OperatorLessThanOrEqual,
            "==" => TokenType::OperatorEqual,
            "=" => TokenType::OperatorAssignment,
            "{" => TokenType::CurlyOpening,
            "}" => TokenType::CurlyClosing,
            _ => {
                if Regex::new(r"^[0-9]+(\.[0-9]+)?$").unwrap().is_match(&value) {
                    TokenType::Constant
                } else if Regex::new(r"^[a-zA-Z]+$").unwrap().is_match(&value) {
                    TokenType::Identifier
                } else {
                    return Err(format!("Invalid token '{value}'"));
                }
            }
        };

        let token_value = match token_type {
            TokenType::Constant => Some(TokenValue::Float(value.parse().unwrap())),
            TokenType::Identifier => Some(TokenValue::Lexeme(value)), // TODO: use an actual value from the symbol table
            _ => None,
        };

        Ok(Token {
            token_type,
            value: token_value,
        })
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_some() {
            write!(f, "{}<{:?}>", self.token_type, self.value.clone().unwrap())
        } else {
            write!(f, "<{}>", self.token_type)
        }
    }
}

pub fn expect_token(
    mut input: impl Iterator<Item = Token>,
    token_type: TokenType,
) -> Result<Token, String> {
    match input.next() {
        Some(token) => {
            if token.token_type != token_type {
                Err(format!(
                    "Unexpected token {}, expected {}",
                    token.token_type, token_type
                ))
            } else {
                Ok(token)
            }
        }
        None => Err(format!(
            "Unexpected end of token stream, expected {}",
            token_type
        )),
    }
}
