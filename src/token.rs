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
    OperatorMultiplyBy,
    OperatorDivision,
    OperatorDivideBy,
    OperatorAddition,
    OperatorSubtraction,
    OperatorIncrement,
    OperatorIncreaseBy,
    OperatorDecrement,
    OperatorDecreaseBy,
    OperatorAssignment,
    OperatorLessThan,
    OperatorLessThanOrEqual,
    OperatorGreaterThan,
    OperatorGreaterThanOrEqual,
    OperatorEqual,
    OperatorNotEqual,
    Constant,
    Identifier,
    CurlyOpening,
    CurlyClosing,
    True,
    False,
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
                TokenType::OperatorMultiplyBy => "*=",
                TokenType::OperatorDivision => "/",
                TokenType::OperatorDivideBy => "/=",
                TokenType::OperatorSubtraction => "-",
                TokenType::OperatorAddition => "+",
                TokenType::OperatorIncrement => "++",
                TokenType::OperatorDecrement => "--",
                TokenType::OperatorIncreaseBy => "+=",
                TokenType::OperatorDecreaseBy => "-=",
                TokenType::OperatorLessThan => "LT",
                TokenType::OperatorLessThanOrEqual => "LTE",
                TokenType::OperatorGreaterThan => "GT",
                TokenType::OperatorGreaterThanOrEqual => "GTE",
                TokenType::OperatorEqual => "EQ",
                TokenType::OperatorNotEqual => "NE",
                TokenType::CurlyOpening => "{",
                TokenType::CurlyClosing => "}",
                TokenType::Constant => "Constant",
                TokenType::Identifier => "Identifier",
                TokenType::True => "true",
                TokenType::False => "false",
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
            | TokenType::OperatorIncreaseBy
            | TokenType::OperatorDecreaseBy
            | TokenType::OperatorAssignment
            | TokenType::OperatorLessThan
            | TokenType::OperatorLessThanOrEqual
            | TokenType::OperatorGreaterThan
            | TokenType::OperatorGreaterThanOrEqual
            | TokenType::OperatorEqual
            | TokenType::OperatorNotEqual => true,
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
            "true" => TokenType::True,
            "false" => TokenType::False,
            ";" => TokenType::Semicolon,
            "(" => TokenType::ParenthesisOpening,
            ")" => TokenType::ParenthesisClosing,
            "*" => TokenType::OperatorMultiplication,
            "*=" => TokenType::OperatorMultiplyBy,
            "/" => TokenType::OperatorDivision,
            "/=" => TokenType::OperatorDivideBy,
            "-" => TokenType::OperatorSubtraction,
            "--" => TokenType::OperatorDecrement,
            "-=" => TokenType::OperatorDecreaseBy,
            "++" => TokenType::OperatorIncrement,
            "+=" => TokenType::OperatorIncreaseBy,
            "+" => TokenType::OperatorAddition,
            ">" => TokenType::OperatorGreaterThan,
            "<" => TokenType::OperatorLessThan,
            ">=" => TokenType::OperatorGreaterThanOrEqual,
            "<=" => TokenType::OperatorLessThanOrEqual,
            "==" => TokenType::OperatorEqual,
            "!=" => TokenType::OperatorNotEqual,
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
            TokenType::Identifier => Some(TokenValue::Lexeme(value)),
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
