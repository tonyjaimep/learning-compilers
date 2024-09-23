use trees::Node;

use crate::{
    symbol_table::{DataType, Symbol, SymbolTable},
    syntax_analysis::{self, BinaryOperation, Relation, SyntaxComponent, Type, UnaryOperation},
};

type Label = String;

#[derive(Debug)]
enum Instruction {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Negation,
    Copy,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Clone, Debug)]
pub enum Address {
    Constant(f32),
    Temp(u32),
}

#[derive(Debug)]
pub struct ThreeAddressCode {
    instruction: Instruction,
    operand_1: Address,
    operand_2: Option<Address>,
    result: Address,
}

#[derive(Debug)]
pub enum Code {
    ThreeAddress(ThreeAddressCode),
    Label(Label),
    JumpIfFalse(Address, Label),
    Jump(Label),
}

impl Code {
    fn try_get_result_address(&self) -> Result<Address, String> {
        if let Code::ThreeAddress(three_address_code) = self {
            Ok(three_address_code.result.clone())
        } else {
            Err("Cannot extract result from non-three-address code".into())
        }
    }
}

fn extract_value_address(
    value_ast: &Node<SyntaxComponent>,
    code: &mut Vec<Code>,
    symbol_table: &mut SymbolTable,
) -> Result<Address, String> {
    let address = match value_ast.data() {
        SyntaxComponent::Constant(const_value) => match const_value {
            syntax_analysis::Constant::Float(float) => {
                log::trace!("extracting address for a float");
                Address::Constant(float.clone())
            }
            syntax_analysis::Constant::Boolean(boolean) => {
                log::trace!("extracting address for a boolean");
                if *boolean {
                    Address::Constant(1.0)
                } else {
                    Address::Constant(0.0)
                }
            }
        },
        SyntaxComponent::Identifier(id_name) => {
            log::trace!("extracting address for an identifier");
            let symbol = symbol_table.get(id_name).unwrap();
            symbol.location.clone().unwrap()
        }
        SyntaxComponent::BinaryOperation(binary_operation) => match binary_operation {
            BinaryOperation::Add => {
                log::trace!("extracting address for an addition");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Addition,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            BinaryOperation::IncreaseBy => {
                log::trace!("extracting address for an increment operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot increment non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                let increase_by_value_tree = value_children.next().unwrap();

                let increase_by_value_address =
                    extract_value_address(increase_by_value_tree, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Addition,
                    operand_1: target_address.clone(),
                    operand_2: Some(increase_by_value_address),
                    result: target_address.clone(),
                }));

                target_address
            }
            BinaryOperation::Subtract => {
                log::trace!("extracting address for a subtraction operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Subtraction,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            BinaryOperation::DecreaseBy => {
                log::trace!("extracting address for a decrement operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot decrement non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                let increase_by_value_tree = value_children.next().unwrap();

                let increase_by_value_address =
                    extract_value_address(increase_by_value_tree, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Addition,
                    operand_1: target_address.clone(),
                    operand_2: Some(increase_by_value_address),
                    result: target_address.clone(),
                }));

                target_address
            }
            BinaryOperation::Multiply => {
                log::trace!("extracting address for a multiplication operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Multiplication,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            BinaryOperation::MultiplyBy => {
                log::trace!("extracting address for a multiplication operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot decrement non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                let increase_by_value_tree = value_children.next().unwrap();

                let increase_by_value_address =
                    extract_value_address(increase_by_value_tree, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Multiplication,
                    operand_1: target_address.clone(),
                    operand_2: Some(increase_by_value_address),
                    result: target_address.clone(),
                }));

                target_address
            }
            BinaryOperation::Divide => {
                log::trace!("extracting address for a division operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Multiplication,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            BinaryOperation::DivideBy => {
                log::trace!("extracting address for a division operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot decrement non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                let increase_by_value_tree = value_children.next().unwrap();

                let increase_by_value_address =
                    extract_value_address(increase_by_value_tree, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Division,
                    operand_1: target_address.clone(),
                    operand_2: Some(increase_by_value_address),
                    result: target_address.clone(),
                }));

                target_address
            }
        },
        SyntaxComponent::UnaryOperation(operation) => match operation {
            UnaryOperation::Increment => {
                log::trace!("extracting address for a unary increment operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot increment non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Addition,
                    operand_1: target_address.clone(),
                    operand_2: Some(Address::Constant(1.0)),
                    result: target_address.clone(),
                }));

                target_address
            }
            UnaryOperation::Decrement => {
                log::trace!("extracting address for a unary decrement operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot decrement non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Subtraction,
                    operand_1: target_address.clone(),
                    operand_2: Some(Address::Constant(1.0)),
                    result: target_address.clone(),
                }));

                target_address
            }
            UnaryOperation::Negation => {
                log::trace!("extracting address for a unary negation operation");
                let mut value_children = value_ast.iter();
                let target = value_children.next().unwrap();

                if !target.data().is_identifier() {
                    return Err("Cannot decrement non-identifier".into());
                }

                let target_address = extract_value_address(target, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Negation,
                    operand_1: target_address.clone(),
                    operand_2: None,
                    result: target_address.clone(),
                }));

                target_address
            }
        },
        SyntaxComponent::Relation(relation_operation) => match relation_operation {
            Relation::GreaterThan => {
                log::trace!("extracting address for a greater than operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::GreaterThan,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            Relation::GreaterThanOrEqual => {
                log::trace!("extracting address for a greater than or equal operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::GreaterThanOrEqual,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            Relation::LessThan => {
                log::trace!("extracting address for a less than operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::LessThan,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            Relation::LessThanOrEqual => {
                log::trace!("extracting address for a less than or equal operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::LessThanOrEqual,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            Relation::EqualTo => {
                log::trace!("extracting address for an equals operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Equal,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
            Relation::NotEqualTo => {
                log::trace!("extracting address for a not-equal-to operation");
                let mut value_children = value_ast.iter();
                let left_operand = value_children.next().unwrap();
                let right_operand = value_children.next().unwrap();
                let result_address = Address::Temp(symbol_table.new_temp());

                let left_operand_address = extract_value_address(left_operand, code, symbol_table)?;

                let right_operand_address =
                    extract_value_address(right_operand, code, symbol_table)?;

                code.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::NotEqual,
                    operand_1: left_operand_address,
                    operand_2: Some(right_operand_address),
                    result: result_address.clone(),
                }));

                result_address
            }
        },
        non_compatible => {
            log::error!("extracting address for a non-compatible operation");
            return Err(format!(
                "SyntaxComponent cannot be converted into address {:?}",
                non_compatible
            ));
        }
    };

    Ok(address)
}

pub fn intermediate_code_generation(
    ast: &Node<SyntaxComponent>,
    symbol_table: &mut SymbolTable,
) -> Result<Vec<Code>, String> {
    let mut result: Vec<Code> = vec![];

    match ast.data() {
        SyntaxComponent::Sequence => {
            log::trace!("Generating code for sequence");
            // clone for this "block"
            let mut symbol_table_clone = symbol_table.clone();
            let mut sequence_items = ast.iter();

            while let Some(sequence_item) = sequence_items.next() {
                let mut item_code =
                    intermediate_code_generation(sequence_item, &mut symbol_table_clone)?;
                result.append(&mut item_code);
            }

            Ok(result)
        }
        SyntaxComponent::If => {
            log::trace!("Generating code for if statement");
            let after_label_string = format!("if_before_{}", symbol_table.new_if());

            let mut children = ast.iter();
            let condition = children.next().unwrap();
            let mut condition_code = intermediate_code_generation(condition, symbol_table)?;
            let condition_code_address = condition_code.last().unwrap().try_get_result_address()?;

            let body = children.next().unwrap();
            let mut body_code = intermediate_code_generation(body, symbol_table)?;

            result.append(&mut condition_code);
            result.push(Code::JumpIfFalse(
                condition_code_address,
                after_label_string.clone(),
            ));
            result.append(&mut body_code);
            result.push(Code::Label(after_label_string));

            Ok(result)
        }
        SyntaxComponent::For => {
            log::trace!("Generating code for for statement");
            let for_id = symbol_table.new_for();
            let before_label_string = format!("for_before_{}", for_id);
            let after_label_string = format!("for_after_{}", for_id);

            let mut children = ast.iter();
            let pre_loop = children.next().unwrap();
            let condition = children.next().unwrap();
            let post_loop = children.next().unwrap();

            let mut pre_loop_code = intermediate_code_generation(pre_loop, symbol_table)?;
            let mut condition_code = intermediate_code_generation(condition, symbol_table)?;
            let mut post_loop_code = intermediate_code_generation(post_loop, symbol_table)?;

            let condition_code_address = condition_code.last().unwrap().try_get_result_address()?;

            let body = children.next().unwrap();
            let mut body_code = intermediate_code_generation(body, symbol_table)?;

            result.append(&mut pre_loop_code);
            result.push(Code::Label(before_label_string.clone()));
            result.append(&mut condition_code);
            result.push(Code::JumpIfFalse(
                condition_code_address.clone(),
                after_label_string.clone(),
            ));
            result.append(&mut body_code);
            result.append(&mut post_loop_code);
            result.append(&mut body_code);
            result.push(Code::Jump(before_label_string));
            result.push(Code::Label(after_label_string.clone()));

            Ok(result)
        }
        SyntaxComponent::Declaration => {
            log::trace!("Generating code for declaration");
            let mut children = ast.iter();
            let data_type = children.next().unwrap().data().try_get_type()?;
            let identifier_name = children.next().unwrap().data().try_get_identifier_name()?;
            let value = children.next();

            let id_address = Address::Temp(symbol_table.new_temp());

            symbol_table.insert(
                identifier_name,
                Symbol {
                    location: Some(id_address.clone()),
                    data_type: match data_type {
                        Type::Number => DataType::Number,
                        Type::Boolean => DataType::Boolean,
                    },
                },
            );

            if value.is_some() {
                let value_tree = value.unwrap();

                let value_result_address =
                    extract_value_address(value_tree, &mut result, symbol_table)?;

                result.push(Code::ThreeAddress(ThreeAddressCode {
                    instruction: Instruction::Copy,
                    operand_1: value_result_address,
                    operand_2: None,
                    result: id_address,
                }));
            }

            Ok(result)
        }
        SyntaxComponent::Assignment => {
            log::trace!("Generating code for assignment");
            let mut children = ast.iter();
            let target_tree = children.next().unwrap();
            let target_address = extract_value_address(target_tree, &mut result, symbol_table)?;
            let value_tree = children.next().unwrap();
            let value_address = extract_value_address(value_tree, &mut result, symbol_table)?;

            result.push(Code::ThreeAddress(ThreeAddressCode {
                instruction: Instruction::Copy,
                operand_1: value_address,
                operand_2: None,
                result: target_address,
            }));

            Ok(result)
        }
        SyntaxComponent::Relation(_)
        | SyntaxComponent::BinaryOperation(_)
        | SyntaxComponent::UnaryOperation(_)
        | SyntaxComponent::Identifier(_) => {
            log::trace!("Generating code for valuable");
            let _address = extract_value_address(ast, &mut result, symbol_table)?;
            Ok(result)
        }

        SyntaxComponent::Type(_) | SyntaxComponent::Null | SyntaxComponent::Constant(_) => Err(
            "Recursed too far bro you shouldn't be generating code for a constant or null or type"
                .into(),
        ),
    }
}
