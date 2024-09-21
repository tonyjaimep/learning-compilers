use std::iter::Peekable;

use crate::{
    expression::parse_expression,
    syntax_analysis::{AbstractSyntaxTree, SyntaxComponent, Type},
    token::{expect_token, Token},
};

pub fn parse_declaration(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    let mut node = AbstractSyntaxTree::new(SyntaxComponent::Declaration);

    let type_token = tokens.next().unwrap();

    match type_token {
        Token::NumType => {
            node.push_back(AbstractSyntaxTree::new(SyntaxComponent::Type(Type::Number)));
        }
        Token::BoolType => {
            node.push_back(AbstractSyntaxTree::new(SyntaxComponent::Type(
                Type::Boolean,
            )));
        }
        token => panic!(
            "First token of declaration must be a type token. Got {:?}",
            token
        ),
    }

    if let Some(Token::Identifier(id)) = tokens.next() {
        node.push_back(AbstractSyntaxTree::new(SyntaxComponent::Identifier(id)));
    } else {
        return Err("Expected identifier after type token".into());
    }

    if let Token::OperatorAssignment = tokens
        .peek()
        .ok_or("Unexpected end of token stream while parsing declaration")?
    {
        // skip assignment token
        tokens.next();
        node.push_back(parse_expression(tokens)?);
    };

    expect_token(tokens, Token::Semicolon)?;

    Ok(node)
}
