use std::iter::Peekable;

use crate::{
    statement::{parse_optional_expression, parse_statement},
    syntax_analysis::{AbstractSyntaxTree, SyntaxComponent},
    token::{expect_token, Token, TokenType},
};

/**
 * for statement
 * for (<optexpr>; <optexpr>; <optexpr>) <stmt>
 *
 * Example:
 * for (i = 0; i < 100; i++) do_something();
 */
pub fn parse_for(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    log::trace!("Parsing For");
    let mut node = AbstractSyntaxTree::new(SyntaxComponent::For);

    expect_token(&mut *input, TokenType::For)?;

    // (
    expect_token(&mut *input, TokenType::ParenthesisOpening)?;

    // initial expression. e.g: i = 0
    node.push_back(parse_optional_expression(&mut *input)?);

    // ;
    expect_token(&mut *input, TokenType::Semicolon)?;

    // condition. e.g: i < 100
    node.push_back(parse_optional_expression(&mut *input)?);

    // ;
    expect_token(&mut *input, TokenType::Semicolon)?;

    // post loop expression. e.g. i++
    node.push_back(parse_optional_expression(&mut *input)?);

    // )
    expect_token(&mut *input, TokenType::ParenthesisClosing)?;

    // loop body
    node.push_back(parse_statement(&mut *input)?);

    Ok(node)
}
