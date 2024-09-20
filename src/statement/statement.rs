use std::iter::Peekable;

use crate::{
    syntax_analysis::AbstractSyntaxTree,
    token::{expect_token, Token},
};

use super::{parse_block, parse_for, parse_if, parse_optional_expression};

pub fn parse_statement(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    let value = match input.peek() {
        Some(next_token) => match next_token {
            Token::For => parse_for(input)?,
            Token::If => parse_if(input)?,
            Token::CurlyOpening => parse_block(input)?,
            _ => {
                let optional_expression = parse_optional_expression(input)?;
                expect_token(input, Token::Semicolon)?;
                optional_expression
            }
        },
        None => return Err("Unexpected end of input while parsing statement".into()),
    };

    Ok(value)
}
