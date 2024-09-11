use crate::token::{Token, TokenType};

pub enum LexemeType {
    Constant,
    OperatorAddition,
    OperatorSubtraction,
    OperatorMultiplication,
    OperatorDivision,
}

pub struct Lexeme {
    #[allow(dead_code)]
    pub lexeme_type: LexemeType,
    pub value: char,
}

impl std::fmt::Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<&Token> for Lexeme {
    type Error = ();

    fn try_from(token: &Token) -> Result<Lexeme, Self::Error> {
        let lexeme_type = match token.token_type {
            TokenType::Operator => match token.value {
                '+' => LexemeType::OperatorAddition,
                '-' => LexemeType::OperatorSubtraction,
                '/' => LexemeType::OperatorDivision,
                '*' => LexemeType::OperatorMultiplication,
                _ => return Err(()),
            },
            TokenType::Constant => LexemeType::Constant,
        };

        Ok(Lexeme {
            value: token.value,
            lexeme_type,
        })
    }
}
