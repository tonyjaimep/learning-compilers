use regex::Regex;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
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
    Constant(f32),
    Identifier(String),
    CurlyOpening,
    CurlyClosing,
    True,
    False,
    Not,
    NumType,
    BoolType,
    EOF,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::If => "IF".to_string(),
                Token::Semicolon => ";".to_string(),
                Token::For => "FOR".to_string(),
                Token::ParenthesisOpening => "(".to_string(),
                Token::ParenthesisClosing => ")".to_string(),
                Token::OperatorAssignment => "=".to_string(),
                Token::OperatorMultiplication => "*".to_string(),
                Token::OperatorMultiplyBy => "*=".to_string(),
                Token::OperatorDivision => "/".to_string(),
                Token::OperatorDivideBy => "/=".to_string(),
                Token::OperatorSubtraction => "-".to_string(),
                Token::OperatorAddition => "+".to_string(),
                Token::OperatorIncrement => "++".to_string(),
                Token::OperatorDecrement => "--".to_string(),
                Token::OperatorIncreaseBy => "+=".to_string(),
                Token::OperatorDecreaseBy => "-=".to_string(),
                Token::OperatorLessThan => "LT".to_string(),
                Token::OperatorLessThanOrEqual => "LTE".to_string(),
                Token::OperatorGreaterThan => "GT".to_string(),
                Token::OperatorGreaterThanOrEqual => "GTE".to_string(),
                Token::OperatorEqual => "EQ".to_string(),
                Token::OperatorNotEqual => "NE".to_string(),
                Token::CurlyOpening => "{".to_string(),
                Token::CurlyClosing => "}".to_string(),
                Token::Constant(value) => format!("Constant({value})"),
                Token::Identifier(name) => format!("Identifier({name})"),
                Token::True => "true".to_string(),
                Token::False => "false".to_string(),
                Token::NumType => "num".to_string(),
                Token::BoolType => "bool".to_string(),
                Token::Not => "NOT".to_string(),
                Token::EOF => "EOF".to_string(),
            }
        )
    }
}

impl Token {
    pub fn is_binary_operator(self: &Self) -> bool {
        match self {
            Token::OperatorMultiplication
            | Token::OperatorDivision
            | Token::OperatorAddition
            | Token::OperatorSubtraction
            | Token::OperatorIncreaseBy
            | Token::OperatorDecreaseBy
            | Token::OperatorAssignment
            | Token::OperatorLessThan
            | Token::OperatorLessThanOrEqual
            | Token::OperatorGreaterThan
            | Token::OperatorGreaterThanOrEqual
            | Token::OperatorEqual
            | Token::OperatorNotEqual => true,
            Token::OperatorIncrement | Token::OperatorDecrement | Token::Not => false,
            _ => panic!("Token is not operator: {}", self),
        }
    }
}

impl TryFrom<String> for Token {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ok_value = match value.as_str() {
            "for" => Token::For,
            "if" => Token::If,
            "true" => Token::True,
            "false" => Token::False,
            "num" => Token::NumType,
            "bool" => Token::BoolType,
            ";" => Token::Semicolon,
            "!" => Token::Not,
            "(" => Token::ParenthesisOpening,
            ")" => Token::ParenthesisClosing,
            "*" => Token::OperatorMultiplication,
            "*=" => Token::OperatorMultiplyBy,
            "/" => Token::OperatorDivision,
            "/=" => Token::OperatorDivideBy,
            "-" => Token::OperatorSubtraction,
            "--" => Token::OperatorDecrement,
            "-=" => Token::OperatorDecreaseBy,
            "++" => Token::OperatorIncrement,
            "+=" => Token::OperatorIncreaseBy,
            "+" => Token::OperatorAddition,
            ">" => Token::OperatorGreaterThan,
            "<" => Token::OperatorLessThan,
            ">=" => Token::OperatorGreaterThanOrEqual,
            "<=" => Token::OperatorLessThanOrEqual,
            "==" => Token::OperatorEqual,
            "!=" => Token::OperatorNotEqual,
            "=" => Token::OperatorAssignment,
            "{" => Token::CurlyOpening,
            "}" => Token::CurlyClosing,
            _ => {
                if Regex::new(r"^[0-9]+(\.[0-9]+)?$").unwrap().is_match(&value) {
                    Token::Constant(value.parse().unwrap())
                } else if Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$")
                    .unwrap()
                    .is_match(&value)
                {
                    Token::Identifier(value)
                } else {
                    return Err(format!("Invalid token '{value}'"));
                }
            }
        };

        Ok(ok_value)
    }
}

pub fn expect_token(
    mut input: impl Iterator<Item = Token>,
    expected_token: Token,
) -> Result<Token, String> {
    match input.next() {
        Some(next_token) => {
            if next_token != expected_token {
                Err(format!(
                    "Unexpected token {}, expected {}",
                    next_token, expected_token
                ))
            } else {
                Ok(next_token)
            }
        }
        None => Err(format!(
            "Unexpected end of token stream, expected {}",
            expected_token
        )),
    }
}
