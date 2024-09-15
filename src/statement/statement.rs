use std::{fmt, iter::Peekable};

use crate::{
    statement::optional_expression_to_string,
    token::{expect_token, Token, TokenType},
};

use super::{
    parse_block, parse_for, parse_if, parse_optional_expression, BlockStatement, ForStatement,
    IfStatement, OptionalExpression,
};

pub enum Statement {
    If(IfStatement),
    For(ForStatement),
    Block(BlockStatement),
    OptionalExpression(OptionalExpression),
}

pub fn parse_statement(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, String> {
    if let Some(next_token) = input.peek() {
        match next_token.token_type {
            TokenType::For => match parse_for(input) {
                Ok(for_statement) => Ok(Statement::For(for_statement)),
                Err(message) => Err(message),
            },
            TokenType::If => match parse_if(input) {
                Ok(if_statement) => Ok(Statement::If(if_statement)),
                Err(message) => Err(message),
            },
            TokenType::CurlyOpening => match parse_block(input) {
                Ok(block_statement) => Ok(Statement::Block(block_statement)),
                Err(message) => Err(message),
            },
            _ => match parse_optional_expression(input) {
                Ok(optional_expression) => {
                    expect_token(input, TokenType::Semicolon)?;
                    Ok(Statement::OptionalExpression(optional_expression))
                }
                Err(message) => Err(message),
            },
        }
    } else {
        return Ok(Statement::OptionalExpression(None));
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body = match self {
            Statement::If(if_statement) => {
                format!(
                    "{}
                    ",
                    if_statement
                )
            }
            Statement::For(for_statement) => format!("{}", for_statement),
            Statement::Block(block_statement) => format!("{}", block_statement),
            Statement::OptionalExpression(optional_expression) => {
                optional_expression_to_string(optional_expression)
            }
        };

        write!(f, "{}", body)
    }
}
