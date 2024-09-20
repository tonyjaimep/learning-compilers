use std::iter::Peekable;

use crate::{
    statement::parse_statement,
    token::{expect_token, Token},
};

use super::{AbstractSyntaxTree, SyntaxComponent};

pub fn syntax_analysis(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    let mut root = AbstractSyntaxTree::new(SyntaxComponent::Sequence);

    while let Some(token) = tokens.peek() {
        if *token == Token::EOF {
            break;
        }
        root.push_back(parse_statement(tokens)?);
    }

    expect_token(tokens, Token::EOF)?;

    Ok(root)
}

#[cfg(test)]
mod tests {
    use trees::tr;

    use super::*;
    use crate::{
        syntax_analysis::{BinaryOperation, Constant, Relation, UnaryOperation},
        token::Token,
    };

    fn assert_tokens_parse_to(tokens: Vec<Token>, expected: AbstractSyntaxTree) {
        let result = syntax_analysis(&mut tokens.into_iter().peekable());
        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_parses_if_statements() {
        // if (i == 0) {}
        let tokens = vec![
            Token::If,
            Token::ParenthesisOpening,
            Token::Identifier("i".into()),
            Token::OperatorEqual,
            Token::Constant(0.0),
            Token::ParenthesisClosing,
            Token::CurlyOpening,
            Token::CurlyClosing,
            Token::EOF,
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::If)
                / (tr(SyntaxComponent::Relation(Relation::EqualTo))
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(Constant::Float(0.0)))))
                / (tr(SyntaxComponent::Sequence)));

        assert_tokens_parse_to(tokens, expected);
    }

    #[test]
    fn it_parses_for_statements() {
        // for (i = 0; i < 100; i++) {}
        let tokens = vec![
            Token::For,
            Token::ParenthesisOpening,
            Token::Identifier("i".into()),
            Token::OperatorEqual,
            Token::Constant(0.0),
            Token::Semicolon,
            Token::Identifier("i".into()),
            Token::OperatorLessThan,
            Token::Constant(100.0),
            Token::Semicolon,
            Token::Identifier("i".into()),
            Token::OperatorIncrement,
            Token::ParenthesisClosing,
            Token::CurlyOpening,
            Token::CurlyClosing,
            Token::EOF,
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::For)
                / (tr(SyntaxComponent::Relation(Relation::EqualTo))
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(Constant::Float(0.0)))))
                / (tr(SyntaxComponent::Relation(Relation::LessThan))
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(Constant::Float(100.0)))))
                / (tr(SyntaxComponent::UnaryOperation(UnaryOperation::Increment))
                    / (tr(SyntaxComponent::Identifier("i".into()))))
                / (tr(SyntaxComponent::Sequence)));

        assert_tokens_parse_to(tokens, expected);
    }
    #[test]
    fn it_parses_block_statements() {
        // { i = 0; j = 1; j++; }
        let tokens = vec![
            Token::CurlyOpening,
            Token::Identifier("i".into()),
            Token::OperatorAssignment,
            Token::Constant(0.0),
            Token::Semicolon,
            Token::Identifier("j".into()),
            Token::OperatorAssignment,
            Token::Constant(1.0),
            Token::Semicolon,
            Token::Identifier("j".into()),
            Token::OperatorIncrement,
            Token::Semicolon,
            Token::CurlyClosing,
            Token::EOF,
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::Sequence)
                / (tr(SyntaxComponent::Assignment)
                    / (tr(SyntaxComponent::Identifier("i".into())))
                    / (tr(SyntaxComponent::Constant(Constant::Float(0.0)))))
                / (tr(SyntaxComponent::Assignment)
                    / (tr(SyntaxComponent::Identifier("j".into())))
                    / (tr(SyntaxComponent::Constant(Constant::Float(1.0)))))
                / (tr(SyntaxComponent::UnaryOperation(UnaryOperation::Increment))
                    / (tr(SyntaxComponent::Identifier("j".into())))));

        assert_tokens_parse_to(tokens, expected);
    }

    #[test]
    fn it_keeps_operator_precedence() {
        // i = 1 + 2 * 3 / 4 + 5 * 6 - j--;
        let tokens = vec![
            Token::Identifier("i".into()),
            Token::OperatorAssignment,
            Token::Constant(1.0),
            Token::OperatorAddition,
            Token::Constant(2.0),
            Token::OperatorMultiplication,
            Token::Constant(3.0),
            Token::OperatorDivision,
            Token::Constant(4.0),
            Token::OperatorAddition,
            Token::Constant(5.0),
            Token::OperatorMultiplication,
            Token::Constant(6.0),
            Token::OperatorSubtraction,
            Token::Identifier("j".into()),
            Token::OperatorDecrement,
            Token::Semicolon,
            Token::EOF,
        ];

        let expected = tr(SyntaxComponent::Sequence)
            / (tr(SyntaxComponent::Assignment)
                / tr(SyntaxComponent::Identifier("i".into()))
                / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Add))
                    / tr(SyntaxComponent::Constant(Constant::Float(1.0)))
                    / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Add))
                        / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Multiply))
                            / tr(SyntaxComponent::Constant(Constant::Float(2.0)))
                            / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Divide))
                                / tr(SyntaxComponent::Constant(Constant::Float(3.0)))
                                / tr(SyntaxComponent::Constant(Constant::Float(4.0)))))
                        / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Subtract))
                            / (tr(SyntaxComponent::BinaryOperation(BinaryOperation::Multiply))
                                / tr(SyntaxComponent::Constant(Constant::Float(5.0)))
                                / tr(SyntaxComponent::Constant(Constant::Float(6.0))))
                            / (tr(SyntaxComponent::UnaryOperation(UnaryOperation::Decrement))
                                / tr(SyntaxComponent::Identifier("j".into())))))));

        assert_tokens_parse_to(tokens, expected);
    }
}
