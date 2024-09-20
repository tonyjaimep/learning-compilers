use std::iter::Peekable;

use crate::{
    syntax_analysis::{AbstractSyntaxTree, SyntaxComponent},
    token::{expect_token, Token},
};

use super::parse_statement;

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
) -> Result<AbstractSyntaxTree, String> {
    log::trace!("Parsing block");
    expect_token(&mut *input, Token::CurlyOpening)?;

    let mut node = AbstractSyntaxTree::new(SyntaxComponent::Sequence);

    while let Some(token) = input.peek() {
        if *token == Token::CurlyClosing {
            break;
        }
        let statement = parse_statement(input)?;
        node.push_back(statement);
    }

    expect_token(input, Token::CurlyClosing)?;

    Ok(node)
}
