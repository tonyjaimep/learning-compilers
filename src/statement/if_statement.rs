use std::iter::Peekable;

use crate::{
    expression::parse_expression,
    syntax_analysis::{AbstractSyntaxTree, SyntaxComponent},
    token::{expect_token, Token},
};

use super::parse_statement;

/**
 * if statement
 * if (<expr>) <stmt>
 *
 * Example:
 * if (i < 5) i = i + 5;
 */
pub fn parse_if(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    log::trace!("Parsing If");
    expect_token(&mut *input, Token::If)?;
    let mut node = AbstractSyntaxTree::new(SyntaxComponent::If);

    expect_token(&mut *input, Token::ParenthesisOpening)?;

    // condition
    node.push_back(parse_expression(&mut *input)?);

    expect_token(&mut *input, Token::ParenthesisClosing)?;

    // body
    node.push_back(parse_statement(&mut *input)?);

    Ok(node)
}
