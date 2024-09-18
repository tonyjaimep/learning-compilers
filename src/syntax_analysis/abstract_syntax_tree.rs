use trees::Tree;

use crate::token::{Token, TokenType, TokenValue};

enum Relation {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    EqualTo,
    NotEqualTo,
}

enum BinaryOperation {
    Add,
    IncreaseBy,
    Subtract,
    DecreaseBy,
    Multiply,
    MultiplyBy,
    Divide,
    DivideBy,
}

enum UnaryOperation {
    Increment,
    Decrement,
}

pub enum SyntaxComponent {
    Null,
    Sequence,
    Statement,
    If,
    For,
    Expression,
    Assignment,
    Relation(Relation),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Constant(f32),
    Identifier(String),
}

pub type AbstractSyntaxTree = Tree<SyntaxComponent>;

impl TryFrom<Token> for SyntaxComponent {
    type Error = String;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let ok_value = match token.token_type {
            TokenType::If => Self::If,
            TokenType::For => Self::For,

            TokenType::OperatorMultiplication => Self::BinaryOperation(BinaryOperation::Multiply),
            TokenType::OperatorMultiplyBy => Self::BinaryOperation(BinaryOperation::MultiplyBy),
            TokenType::OperatorDivision => Self::BinaryOperation(BinaryOperation::Divide),
            TokenType::OperatorDivideBy => Self::BinaryOperation(BinaryOperation::DivideBy),
            TokenType::OperatorAddition => Self::BinaryOperation(BinaryOperation::Add),
            TokenType::OperatorSubtraction => Self::BinaryOperation(BinaryOperation::Subtract),
            TokenType::OperatorIncreaseBy => Self::BinaryOperation(BinaryOperation::IncreaseBy),
            TokenType::OperatorDecreaseBy => Self::BinaryOperation(BinaryOperation::DecreaseBy),

            TokenType::OperatorIncrement => Self::UnaryOperation(UnaryOperation::Increment),
            TokenType::OperatorDecrement => Self::UnaryOperation(UnaryOperation::Decrement),

            TokenType::OperatorAssignment => Self::Assignment,

            TokenType::OperatorLessThan => Self::Relation(Relation::LessThan),
            TokenType::OperatorLessThanOrEqual => Self::Relation(Relation::LessThanOrEqual),
            TokenType::OperatorGreaterThan => Self::Relation(Relation::GreaterThan),
            TokenType::OperatorGreaterThanOrEqual => Self::Relation(Relation::GreaterThanOrEqual),
            TokenType::OperatorEqual => Self::Relation(Relation::EqualTo),
            TokenType::OperatorNotEqual => Self::Relation(Relation::NotEqualTo),
            TokenType::Identifier => {
                let token_value = token.value.expect(
                    "Error while parsing syntax component: identifier token does not have a value",
                );

                if let TokenValue::Lexeme(lexeme) = token_value {
                    Self::Identifier(lexeme)
                } else {
                    return Err("Identifier value must be a lexeme".into());
                }
            }
            TokenType::Constant => {
                let token_value = token.value.expect(
                    "Error while parsing syntax component: constant token does not have a value",
                );

                if let TokenValue::Float(float) = token_value {
                    SyntaxComponent::Constant(float)
                } else {
                    return Err("Constant value must be a float".into());
                }
            }
            TokenType::True => Self::Constant(1.0),
            TokenType::False => Self::Constant(0.0),
            _ => {
                return Err(format!(
                    "Token {} does not represent a syntax component",
                    token
                ))
            }
        };
        Ok(ok_value)
    }
}
