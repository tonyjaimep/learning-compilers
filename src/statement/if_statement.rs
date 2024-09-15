use std::{fmt, iter::Peekable};

use crate::{
    expression::{parse_expression, Expression},
    token::{expect_token, Token, TokenType},
};

use super::{parse_statement, Statement};

pub struct IfStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "If({})\n{}", self.condition, self.body)
    }
}
/**
 * if statement
 * if (<expr>) <stmt>
 *
 * Example:
 * if (i < 5) i = i + 5;
 */
pub fn parse_if(input: &mut Peekable<impl Iterator<Item = Token>>) -> Result<IfStatement, String> {
    log::trace!("Parsing If");
    expect_token(&mut *input, TokenType::If)?;

    expect_token(&mut *input, TokenType::ParenthesisOpening)?;

    let condition = parse_expression(&mut *input)?;

    expect_token(&mut *input, TokenType::ParenthesisClosing)?;

    let body = parse_statement(&mut *input)?;

    Ok(IfStatement {
        condition,
        body: Box::new(body),
    })
}
