use trees::Node;

use crate::{
    symbol_table::{DataType, Symbol, SymbolTable},
    syntax_analysis::{Constant, SyntaxComponent, Type, UnaryOperation},
};

#[derive(PartialEq, Debug)]
pub enum ReturnType {
    Boolean,
    Number,
    Void,
}

pub fn semantic_analysis(
    abstract_syntax_tree: &Node<SyntaxComponent>,
    symbol_table: &mut SymbolTable,
) -> Result<ReturnType, String> {
    let mut children = abstract_syntax_tree.iter();

    let result = match abstract_syntax_tree.data() {
        SyntaxComponent::If => {
            let condition = children
                .next()
                .ok_or(String::from("If statement must have a condition"))?;

            let condition_type = semantic_analysis(condition, symbol_table)?;

            if condition_type != ReturnType::Boolean {
                return Err(String::from(
                    "If statement condition must evaluate to a boolean",
                ));
            }

            let body = children
                .next()
                .ok_or(String::from("If statement must have a condition"))?;

            semantic_analysis(body, symbol_table)?;

            Ok(ReturnType::Void)
        }
        SyntaxComponent::For => {
            let pre_loop_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("If statement must have a pre-loop node"))?,
                symbol_table,
            )?;

            if pre_loop_type != ReturnType::Void {
                return Err(String::from("For statement pre-loop must evaluate to void"));
            }

            let condition_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("If statement must have a condition node"))?,
                symbol_table,
            )?;

            if condition_type != ReturnType::Boolean {
                return Err(String::from(
                    "For statement condition must evaluate to boolean",
                ));
            }

            let post_loop_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("If statement must have a post-loop node"))?,
                symbol_table,
            )?;

            if post_loop_type != ReturnType::Void {
                return Err(String::from(
                    "Post-loop return type must be a void return type",
                ));
            }

            let body_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("If statement must have a body node"))?,
                symbol_table,
            )?;

            if body_type != ReturnType::Void {
                return Err(String::from("For statement body must evaluate to boolean"));
            }

            Ok(ReturnType::Void)
        }
        SyntaxComponent::Null => {
            if abstract_syntax_tree.has_no_child() {
                Ok(ReturnType::Void)
            } else {
                Err("Null node must not have any children".into())
            }
        }
        SyntaxComponent::Sequence => {
            let mut inherited_symbol_table = symbol_table.clone();
            for child in children {
                semantic_analysis(child, &mut inherited_symbol_table)?;
            }
            // don't check for more children, avoid borrow checker error
            return Ok(ReturnType::Void);
        }
        SyntaxComponent::Assignment => {
            let left_side = children
                .next()
                .ok_or(String::from("Assignment must have a left side"))?;

            if !left_side.data().is_identifier() {
                return Err(format!(
                    "Expected assignment to identifier, got {:?}",
                    left_side.data(),
                ));
            }

            let left_side_type = semantic_analysis(left_side, symbol_table)?;
            let right_side_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("Assignment must have a right side"))?,
                symbol_table,
            )?;

            if left_side_type != right_side_type {
                Err(format!(
                    "{:?} cannot be assigned to {:?}",
                    left_side_type, right_side_type
                ))
            } else {
                Ok(ReturnType::Void)
            }
        }
        SyntaxComponent::Relation(_) => {
            let left_side_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("Relation operator must have a left side"))?,
                symbol_table,
            )?;

            let right_side_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("Relation operator must have a right side"))?,
                symbol_table,
            )?;

            if left_side_type != right_side_type {
                Err(format!(
                    "{:?} cannot be compared to {:?}",
                    left_side_type, right_side_type
                ))
            } else {
                Ok(ReturnType::Boolean)
            }
        }
        SyntaxComponent::BinaryOperation(_) => {
            let left_side_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("Binary operator must have a left side"))?,
                symbol_table,
            )?;

            let right_side_type = semantic_analysis(
                children
                    .next()
                    .ok_or(String::from("Binary operator must have a right side"))?,
                symbol_table,
            )?;

            if left_side_type != ReturnType::Number || right_side_type != ReturnType::Number {
                return Err("Binary operation must be applied to numbers".into());
            }

            Ok(ReturnType::Number)
        }
        SyntaxComponent::UnaryOperation(unary_operation) => {
            let operand = children
                .next()
                .ok_or(String::from("Unary operator must have exactly one operand"))?;

            let operand_type = semantic_analysis(operand, symbol_table)?;

            match unary_operation {
                UnaryOperation::Increment | UnaryOperation::Decrement => {
                    if operand_type != ReturnType::Number {
                        Err("Cannot increment or decrement a non-number".into())
                    } else {
                        Ok(ReturnType::Void)
                    }
                }
                UnaryOperation::Negation => {
                    if operand_type != ReturnType::Boolean {
                        Err("Cannot negate a non-boolean".into())
                    } else {
                        Ok(ReturnType::Boolean)
                    }
                }
            }
        }
        SyntaxComponent::Constant(constant) => match constant {
            Constant::Boolean(_) => Ok(ReturnType::Boolean),
            Constant::Float(_) => Ok(ReturnType::Number),
        },
        SyntaxComponent::Identifier(identifier) => {
            let symbol = symbol_table
                .get(identifier)
                .ok_or(format!("Undeclared identifier: {}", identifier))?;

            let ok_value = match symbol.data_type {
                DataType::Number => ReturnType::Number,
                DataType::Boolean => ReturnType::Boolean,
            };

            Ok(ok_value)
        }
        SyntaxComponent::Declaration => {
            // advance iterator
            let data_type = children
                .next()
                .ok_or(format!("Declaration must have a return type"))?;

            let expected_value_type = match data_type.data() {
                SyntaxComponent::Type(t) => match t {
                    Type::Boolean => ReturnType::Boolean,
                    Type::Number => ReturnType::Number,
                },
                _ => return Err(format!("Data type must be a type syntax component")),
            };

            let identifier_data_type = match expected_value_type {
                ReturnType::Number => DataType::Number,
                ReturnType::Boolean => DataType::Boolean,
                _ => panic!("Got expected void type"),
            };

            let identifier = children
                .next()
                .ok_or(format!("Declaration must have an identifier to assign"))?;

            if let SyntaxComponent::Identifier(id_name) = identifier.data() {
                symbol_table.insert(
                    id_name.to_string(),
                    Symbol {
                        data_type: identifier_data_type,
                    },
                );
            }

            let value = children.next();

            if value.is_some() {
                let value_type = semantic_analysis(value.unwrap(), symbol_table)?;
                if expected_value_type != value_type {
                    return Err(format!(
                        "Cannot define {:?} as {:?}",
                        value_type, expected_value_type,
                    ));
                }
            }

            Ok(ReturnType::Void)
        }
        SyntaxComponent::Type(_) => Ok(ReturnType::Void),
    };

    if children.next().is_some() {
        Err("Too many children".into())
    } else {
        result
    }
}
