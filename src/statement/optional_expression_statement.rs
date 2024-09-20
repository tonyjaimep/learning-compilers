use std::iter::Peekable;

use crate::{
    expression::parse_expression,
    syntax_analysis::{AbstractSyntaxTree, SyntaxComponent},
    token::Token,
};

pub fn parse_optional_expression(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    log::trace!("Parsing Optional Expression");

    let ok_value = match input.peek() {
        Some(next_token) => match next_token {
            // empty expression
            Token::EOF | Token::Semicolon | Token::ParenthesisClosing => {
                log::trace!("Got to end of expression");
                AbstractSyntaxTree::new(SyntaxComponent::Null)
            }
            _ => {
                let parsed_expression = parse_expression(input)?;
                parsed_expression
            }
        },
        None => {
            log::trace!("Got to end of token iterator while parsing optional expression");
            AbstractSyntaxTree::new(SyntaxComponent::Null)
        }
    };

    Ok(ok_value)
}
