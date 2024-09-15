use std::{fmt, iter::Peekable};

use crate::token::{expect_token, Token, TokenType};

use super::{parse_statement, Statement};

pub struct BlockStatement {
    pub body: Vec<Box<Statement>>,
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockStart\n{}\nBlockEnd",
            self.body
                .iter()
                .fold(String::new(), |s, c| format!("{}\n{}", s, c))
        )
    }
}

/**
 * block statement
 * { <statement>; }
 *
 * Example:
 * {
 *      i = 0;
 *      i++;
 * }
 */
pub fn parse_block(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<BlockStatement, String> {
    log::trace!("Parsing block");
    expect_token(&mut *input, TokenType::CurlyOpening)?;

    let mut body = Vec::new();

    while let Some(token) = input.peek() {
        if token.token_type == TokenType::CurlyClosing {
            break;
        }
        let statement = parse_statement(input)?;
        body.push(Box::new(statement));
    }

    expect_token(input, TokenType::CurlyClosing)?;

    Ok(BlockStatement { body })
}
