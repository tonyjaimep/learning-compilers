use std::{fmt, iter::Peekable};

use crate::{
    statement::{optional_expression_to_string, parse_optional_expression, parse_statement},
    token::{expect_token, Token, TokenType},
};

use super::{OptionalExpression, Statement};

pub struct ForStatement {
    pub pre_loop: OptionalExpression,
    pub condition: OptionalExpression,
    pub post_loop: OptionalExpression,
    pub body: Box<Statement>,
}

impl fmt::Display for ForStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "For({}; {}; {})\n{}",
            optional_expression_to_string(&self.pre_loop),
            optional_expression_to_string(&self.condition),
            optional_expression_to_string(&self.post_loop),
            self.body
        )
    }
}

/**
 * for statement
 * for (<optexpr>; <optexpr>; <optexpr>) <stmt>
 *
 * Example:
 * for (i = 0; i < 100; i++) do_something();
 */
pub fn parse_for(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<ForStatement, String> {
    log::trace!("Parsing For");
    expect_token(&mut *input, TokenType::For)?;

    // (
    expect_token(&mut *input, TokenType::ParenthesisOpening)?;

    // initial expression. e.g: i = 0
    let pre_loop = parse_optional_expression(&mut *input)?;

    // ;
    expect_token(&mut *input, TokenType::Semicolon)?;

    // condition. e.g: i < 100
    let condition = parse_optional_expression(&mut *input)?;

    // ;
    expect_token(&mut *input, TokenType::Semicolon)?;

    // post loop expression. e.g. i++
    let post_loop = parse_optional_expression(&mut *input)?;

    // )
    expect_token(&mut *input, TokenType::ParenthesisClosing)?;

    // loop body
    let body = parse_statement(&mut *input)?;

    let for_statement = ForStatement {
        pre_loop,
        condition,
        post_loop,
        body: Box::new(body),
    };

    Ok(for_statement)
}
