use trees::Tree;

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
    Negation,
}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Float(f32),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Number,
    Boolean,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxComponent {
    Null,
    // AKA blocks
    Sequence,
    If,
    For,
    Assignment,
    Declaration,
    Type(Type),
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
            Token::Not => Self::UnaryOperation(UnaryOperation::Negation),

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

impl SyntaxComponent {
    pub fn is_identifier(&self) -> bool {
        if let Self::Identifier(_) = self {
            true
        } else {
            false
        }
    }

    pub fn try_get_identifier_name(&self) -> Result<String, String> {
        if let Self::Identifier(name) = self {
            Ok(name.clone())
        } else {
            Err("Cannot get identifier name from non-identifier. Check your assumptions".into())
        }
    }

    pub fn try_get_type(&self) -> Result<Type, String> {
        if let Self::Type(data_type) = self {
            Ok(data_type.clone())
        } else {
            Err("Cannot get identifier name from non-identifier. Check your assumptions".into())
        }
    }
}
