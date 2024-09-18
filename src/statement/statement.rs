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
    let value = match input.peek() {
        Some(next_token) => match next_token.token_type {
            TokenType::For => Statement::For(parse_for(input)?),
            TokenType::If => Statement::If(parse_if(input)?),
            TokenType::CurlyOpening => Statement::Block(parse_block(input)?),
            _ => {
                let optional_expression = parse_optional_expression(input)?;
                expect_token(input, TokenType::Semicolon)?;
                Statement::OptionalExpression(optional_expression)
            }
        },
        None => Statement::OptionalExpression(None),
    };

    Ok(value)
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
