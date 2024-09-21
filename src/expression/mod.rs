use std::iter::Peekable;

use crate::{syntax_analysis::AbstractSyntaxTree, token::*};

// examples of expressions
// 5 + 1
// foo > bar
// foo > (5 + 1)
// 5 + 1 >= foo + 2
// foo++ < (bar - (4 + 2))
// foo++

fn token_concludes_expression(token: &Token) -> bool {
    match token {
        Token::EOF | Token::Semicolon | Token::ParenthesisClosing => true,
        _ => false,
    }
}

pub fn parse_expression(
    input: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<AbstractSyntaxTree, String> {
    log::debug!("Parsing expression");

    let mut expression_tokens: Vec<Token> = vec![];

    while let Some(token) = input.peek() {
        if token_concludes_expression(token) {
            break;
        } else {
            expression_tokens.push(input.next().unwrap());
        }
    }

    log::debug!(
        "Parsing expression with  {} tokens",
        expression_tokens.len()
    );

    if expression_tokens.len() == 1 {
        let only_token = expression_tokens[0].clone();
        return match only_token {
            Token::Constant(_) | Token::Identifier(_) | Token::True | Token::False => {
                Ok(AbstractSyntaxTree::new(only_token.try_into()?))
            }
            _ => Err(format!(
                "Expected constant or identifier as operands. Got {:?}",
                only_token
            )),
        };
    }

    let operator_precedence = vec![
        vec![Token::OperatorAssignment],
        vec![Token::OperatorAddition, Token::OperatorSubtraction],
        vec![Token::OperatorMultiplication, Token::OperatorDivision],
        vec![
            Token::OperatorDecrement,
            Token::OperatorIncrement,
            Token::OperatorGreaterThan,
            Token::OperatorGreaterThanOrEqual,
            Token::OperatorLessThan,
            Token::OperatorLessThanOrEqual,
            Token::OperatorEqual,
        ],
        vec![
            Token::OperatorIncreaseBy,
            Token::OperatorDecreaseBy,
            Token::Not,
        ],
    ];

    for precedence in operator_precedence {
        for operator_type in precedence {
            let operator_position_option = expression_tokens
                .clone()
                .into_iter()
                .position(|token| token == operator_type);

            if operator_position_option.is_some() {
                let position = operator_position_option.unwrap();
                let operator = &expression_tokens[position];
                let mut node = AbstractSyntaxTree::new(operator.clone().try_into()?);

                if operator.is_binary_operator() {
                    let first_operand = parse_expression(
                        &mut expression_tokens[0..position]
                            .to_vec()
                            .into_iter()
                            .peekable(),
                    )?;
                    let second_operand = parse_expression(
                        &mut expression_tokens[position + 1..]
                            .to_vec()
                            .into_iter()
                            .peekable(),
                    )?;
                    node.push_back(first_operand);
                    node.push_back(second_operand);
                    return Ok(node);
                } else {
                    // operator is unary
                    let operand_tokens: &[Token] = if position == 0 {
                        &expression_tokens[1..]
                    } else {
                        &expression_tokens[0..position]
                    };
                    let mut operand_tokens_iter = operand_tokens.to_vec().into_iter().peekable();
                    let operand = parse_expression(&mut operand_tokens_iter)?;
                    node.push_back(operand);

                    return Ok(node);
                }
            }
        }
    }

    println!("{:?}", expression_tokens);
    Err(String::from("Unexpected end of expression"))
}
