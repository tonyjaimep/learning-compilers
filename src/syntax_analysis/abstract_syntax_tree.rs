use trees::{Node, Tree};

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Relation {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    EqualTo,
    NotEqualTo,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperation {
    Add,
    IncreaseBy,
    Subtract,
    DecreaseBy,
    Multiply,
    MultiplyBy,
    Divide,
    DivideBy,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperation {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Float(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum SyntaxComponent {
    Null,
    Sequence,
    If,
    For,
    Assignment,
    Relation(Relation),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Constant(Constant),
    Identifier(String),
}

pub type AbstractSyntaxTree = Tree<SyntaxComponent>;

impl TryFrom<Token> for SyntaxComponent {
    type Error = String;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let ok_value = match token {
            Token::If => Self::If,
            Token::For => Self::For,

            Token::OperatorMultiplication => Self::BinaryOperation(BinaryOperation::Multiply),
            Token::OperatorMultiplyBy => Self::BinaryOperation(BinaryOperation::MultiplyBy),
            Token::OperatorDivision => Self::BinaryOperation(BinaryOperation::Divide),
            Token::OperatorDivideBy => Self::BinaryOperation(BinaryOperation::DivideBy),
            Token::OperatorAddition => Self::BinaryOperation(BinaryOperation::Add),
            Token::OperatorSubtraction => Self::BinaryOperation(BinaryOperation::Subtract),
            Token::OperatorIncreaseBy => Self::BinaryOperation(BinaryOperation::IncreaseBy),
            Token::OperatorDecreaseBy => Self::BinaryOperation(BinaryOperation::DecreaseBy),

            Token::OperatorIncrement => Self::UnaryOperation(UnaryOperation::Increment),
            Token::OperatorDecrement => Self::UnaryOperation(UnaryOperation::Decrement),

            Token::OperatorAssignment => Self::Assignment,

            Token::OperatorLessThan => Self::Relation(Relation::LessThan),
            Token::OperatorLessThanOrEqual => Self::Relation(Relation::LessThanOrEqual),
            Token::OperatorGreaterThan => Self::Relation(Relation::GreaterThan),
            Token::OperatorGreaterThanOrEqual => Self::Relation(Relation::GreaterThanOrEqual),
            Token::OperatorEqual => Self::Relation(Relation::EqualTo),
            Token::OperatorNotEqual => Self::Relation(Relation::NotEqualTo),
            Token::Identifier(name) => Self::Identifier(name),
            Token::Constant(value) => Self::Constant(Constant::Float(value)),
            Token::True => Self::Constant(Constant::Boolean(true)),
            Token::False => Self::Constant(Constant::Boolean(false)),
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

pub fn evaluates_to_boolean(node: &Node<SyntaxComponent>) -> bool {
    match node.data() {
        SyntaxComponent::Relation(_) => true,
        SyntaxComponent::Constant(constant) => match constant {
            Constant::Boolean(_) => true,
            _ => false,
        },
        _ => false,
    }
}
