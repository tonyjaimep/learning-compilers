use std::iter::Peekable;

use crate::{
    statement::{parse_statement, BlockStatement, Statement},
    token::{expect_token, Token, TokenType},
};

pub fn syntax_analysis(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, String> {
    let statement = Statement::Block(BlockStatement {
        body: vec![Box::new(parse_statement(tokens)?)],
    });
    expect_token(tokens, TokenType::EOF)?;
    Ok(statement)
}
