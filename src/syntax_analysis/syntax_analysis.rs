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

    expect_token(tokens, TokenType::EOF)?;

    Ok(root)
}

#[cfg(test)]
mod tests {
    use trees::tr;

    use super::*;
    use crate::{
        syntax_analysis::{BinaryOperation, Relation, UnaryOperation},
        token::TokenValue,
    };

    fn assert_tokens_parse_to(
        tokens: Vec<(TokenType, Option<TokenValue>)>,
        expected: AbstractSyntaxTree,
    ) {
        let result = syntax_analysis(
            &mut tokens
                .iter()
                .map(|(token_type, token_value)| Token {
                    token_type: token_type.clone(),
                    value: token_value.clone(),
                })
                .peekable(),
        );
        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_parses_if_statements() {
        // if (i == 0) {}
        let tokens = vec![
            (TokenType::If, None),
            (TokenType::ParenthesisOpening, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("i".into()))),
            (TokenType::OperatorEqual, None),
            (TokenType::Constant, Some(TokenValue::Float(0.0))),
            (TokenType::ParenthesisClosing, None),
            (TokenType::CurlyOpening, None),
            (TokenType::CurlyClosing, None),
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::If)
                / (tr(SyntaxComponent::Relation(Relation::EqualTo))
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(0.0))))
                / (tr(SyntaxComponent::Sequence)));

        assert_tokens_parse_to(tokens, expected);
    }

    #[test]
    fn it_parses_for_statements() {
        // for (i = 0; i < 100; i++) {}
        let tokens = vec![
            (TokenType::For, None),
            (TokenType::ParenthesisOpening, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("i".into()))),
            (TokenType::OperatorEqual, None),
            (TokenType::Constant, Some(TokenValue::Float(0.0))),
            (TokenType::Semicolon, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("i".into()))),
            (TokenType::OperatorLessThan, None),
            (TokenType::Constant, Some(TokenValue::Float(100.0))),
            (TokenType::Semicolon, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("i".into()))),
            (TokenType::OperatorIncrement, None),
            (TokenType::ParenthesisClosing, None),
            (TokenType::CurlyOpening, None),
            (TokenType::CurlyClosing, None),
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::For)
                / (tr(SyntaxComponent::Relation(Relation::EqualTo))
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(0.0))))
                / (tr(SyntaxComponent::Relation(Relation::LessThan))
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(100.0))))
                / (tr(SyntaxComponent::UnaryOperation(UnaryOperation::Increment))
                    / (tr(SyntaxComponent::Identifier("i".into()))))
                / (tr(SyntaxComponent::Sequence)));

        assert_tokens_parse_to(tokens, expected);
    }
    #[test]
    fn it_parses_block_statements() {
        // { i = 0; j = 1; j++; }
        let tokens = vec![
            (TokenType::CurlyOpening, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("i".into()))),
            (TokenType::OperatorAssignment, None),
            (TokenType::Constant, Some(TokenValue::Float(0.0))),
            (TokenType::Semicolon, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("j".into()))),
            (TokenType::OperatorAssignment, None),
            (TokenType::Constant, Some(TokenValue::Float(1.0))),
            (TokenType::Semicolon, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("j".into()))),
            (TokenType::OperatorIncrement, None),
            (TokenType::Semicolon, None),
            (TokenType::CurlyClosing, None),
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::Sequence)
                / (tr(SyntaxComponent::Assignment)
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(0.0))))
                / (tr(SyntaxComponent::Assignment)
                    / (tr(SyntaxComponent::Identifier("j".into())))
                    / (tr(SyntaxComponent::Constant(1.0))))
                / (tr(SyntaxComponent::UnaryOperation(UnaryOperation::Increment))
                    / (tr(SyntaxComponent::Identifier("j".into())))));

        assert_tokens_parse_to(tokens, expected);
    }

    #[test]
    fn it_keeps_operator_precedence() {
        // i = 1 + 2 * 3 / 4 + 5 * 6 - j--;
        let tokens = vec![
            (TokenType::Identifier, Some(TokenValue::Lexeme("i".into()))),
            (TokenType::OperatorAssignment, None),
            (TokenType::Constant, Some(TokenValue::Float(1.0))),
            (TokenType::OperatorAddition, Some(TokenValue::Float(1.0))),
            (TokenType::Constant, Some(TokenValue::Float(2.0))),
            (TokenType::OperatorMultiplication, None),
            (TokenType::Constant, Some(TokenValue::Float(3.0))),
            (TokenType::OperatorDivision, None),
            (TokenType::Constant, Some(TokenValue::Float(4.0))),
            (TokenType::OperatorAddition, None),
            (TokenType::Constant, Some(TokenValue::Float(5.0))),
            (TokenType::OperatorMultiplication, None),
            (TokenType::Constant, Some(TokenValue::Float(6.0))),
            (TokenType::OperatorSubtraction, None),
            (TokenType::Identifier, Some(TokenValue::Lexeme("j".into()))),
            (TokenType::OperatorDecrement, None),
            (TokenType::Semicolon, None),
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::Assignment)
                / tr(SyntaxComponent::Identifier("i".into()))
                / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Add))
                    / tr(SyntaxComponent::Constant(1.0))
                    / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Add))
                        / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Multiply))
                            / tr(SyntaxComponent::Constant(2.0))
                            / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Divide))
                                / tr(SyntaxComponent::Constant(3.0))
                                / tr(SyntaxComponent::Constant(4.0))))
                        / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Subtract))
                            / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Multiply))
                                / tr(SyntaxComponent::Constant(5.0))
                                / tr(SyntaxComponent::Constant(6.0)))
                            / (tr(SyntaxComponent::UnaryOperation(UnaryOperation::Decrement))
                                / tr(SyntaxComponent::Identifier("j".into())))))));

        assert_tokens_parse_to(tokens, expected);
    }
}
