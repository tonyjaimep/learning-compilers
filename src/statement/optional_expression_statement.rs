use std::iter::Peekable;

use crate::{
    expression::{expression_to_string, parse_expression, Expression},
    token::{Token, TokenType},
};

pub type OptionalExpression = Option<Expression>;

pub fn optional_expression_to_string(optional_expression: &OptionalExpression) -> String {
    match optional_expression {
        Some(expression) => expression_to_string(expression),
        None => String::from("<Empty>"),
    }
}

pub fn parse_optional_expression(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<OptionalExpression, String> {
    log::trace!("Parsing Optional Expression");
    if let Some(next_token) = input.peek() {
        match next_token.token_type {
            // empty expression
            TokenType::EOF | TokenType::Semicolon | TokenType::ParenthesisClosing => {
                log::trace!("Got to end of expression");
                Ok(None)
            }
            _ => {
                let parsed_expression = parse_expression(input)?;
                Ok(Some(parsed_expression))
            }
        }
    } else {
        log::trace!("Got to end of token iterator while parsing optional expression");
        Ok(None)
    }
}
