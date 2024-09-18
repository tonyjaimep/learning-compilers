use std::iter::Peekable;

use crate::{
    statement::parse_statement,
    token::{expect_token, Token, TokenType},
};

use super::{AbstractSyntaxTree, SyntaxComponent};

pub fn syntax_analysis(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    let mut root = AbstractSyntaxTree::new(SyntaxComponent::Sequence);

    while let Some(token) = tokens.peek() {
        if token.token_type == TokenType::EOF {
            break;
        }
        root.push_back(parse_statement(tokens)?);
    }

    expect_token(tokens, TokenType::EOF);

    Ok(root)
}
