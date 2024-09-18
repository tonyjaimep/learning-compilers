use std::iter::Peekable;

use crate::{
    symbol_table::{self, SymbolTable},
    token::*,
};

#[derive(Debug)]
enum TokenBuildingStateType {
    // empty stack
    Empty,
    // only single ! in stack
    Not,
    // only single = in stack
    Equal,
    // only composable operators in stack
    ComposableOperator,
    // only a-zA-Z in stack
    Alphabetic,
    // only 0-9 in stack
    Numeric,
    // 0-9+. in stack
    NumericPeriod,
    // 0-9+.0-9+ in stack
    NumericFloatingPoint,
    // / in stack
    MaybeComment,
    // // found. waiting for newline
    LineComment,
    // /* found. waiting for *
    BlockComment,
    // /* * found. waiting for /
    BlockCommentMaybeClosing,
}

struct TokenBuildingState {
    pub state_type: TokenBuildingStateType,
    pub accumulator: String,
    pub token_vector: Vec<Token>,
}

macro_rules! composable_operators {
    () => {
        '*' | '<' | '>' | '+' | '-'
    };
}

macro_rules! grouping_characters {
    () => {
        '{' | '}' | '(' | ')'
    };
}

fn tokenize(string: String, symbol_table: SymbolTable) -> Result<(Token, SymbolTable)> {
    let token = Token::try_from(string)?;
    symbol_table.insert(k, v)
}

fn reset_state_with_state_type(
    state: TokenBuildingState,
    state_type: TokenBuildingStateType,
) -> TokenBuildingState {
    TokenBuildingState {
        state_type,
        accumulator: String::new(),
        token_vector: state.token_vector,
    }
}

fn push_token(state: TokenBuildingState, token: Token) -> TokenBuildingState {
    TokenBuildingState {
        state_type: TokenBuildingStateType::Empty,
        accumulator: String::new(),
        token_vector: vec![&state.token_vector[..], &[token]].concat(),
    }
}

fn commit_accumulator(state: TokenBuildingState) -> Result<TokenBuildingState, String> {
    let accumulator = state.accumulator.clone();
    if accumulator.is_empty() {
        Ok(state)
    } else {
        let token = Token::try_from(accumulator)?;

        Ok(push_token(state, token))
    }
}

fn accumulate_character(
    character: char,
    state: TokenBuildingState,
    state_type: TokenBuildingStateType,
) -> TokenBuildingState {
    let mut accumulator = state.accumulator.clone();
    accumulator.push(character);

    TokenBuildingState {
        state_type,
        accumulator,
        token_vector: state.token_vector,
    }
}

fn unexpected_character_error(
    character: char,
    state: TokenBuildingState,
) -> Result<TokenBuildingState, String> {
    Err(format!(
        "Unexpected {} after {}, with state {:?}",
        if character.is_whitespace() {
            String::from("whitespace")
        } else {
            character.into()
        },
        if state.accumulator.is_empty() {
            String::from("empty string")
        } else {
            state.accumulator.into()
        },
        state.state_type
    ))
}

fn commit_accumulator_and_begin_with_character(
    character: char,
    state: TokenBuildingState,
) -> Result<TokenBuildingState, String> {
    let state_after_committing_accumulator = commit_accumulator(state)?;

    let state_type = match character {
        '!' => TokenBuildingStateType::Not,
        '=' => TokenBuildingStateType::Equal,
        '/' => TokenBuildingStateType::MaybeComment,
        composable_operators!() => TokenBuildingStateType::ComposableOperator,
        // single-character tokens result in an empty state
        ';' | grouping_characters!() => {
            return Ok(push_token(
                state_after_committing_accumulator,
                Token::try_from(character.to_string())?,
            ))
        }
        _ if character.is_alphabetic() => TokenBuildingStateType::Alphabetic,
        _ if character.is_numeric() => TokenBuildingStateType::Numeric,
        _ => return unexpected_character_error(character, state_after_committing_accumulator),
    };

    return Ok(accumulate_character(
        character,
        state_after_committing_accumulator,
        state_type,
    ));
}

fn accumulate_character_and_commit_accumulator(
    character: char,
    state: TokenBuildingState,
) -> Result<TokenBuildingState, String> {
    // setting state to empty because it will not matter after accumulator is committed
    commit_accumulator(accumulate_character(
        character,
        state,
        TokenBuildingStateType::Empty,
    ))
}

fn commit_accumulator_and_single_character_token(
    character: char,
    state: TokenBuildingState,
) -> Result<TokenBuildingState, String> {
    let state_after_committing_accumulator = commit_accumulator(state)?;
    Ok(push_token(
        state_after_committing_accumulator,
        Token::try_from(character.to_string())?,
    ))
}

fn handle_character(
    character: char,
    state: TokenBuildingState,
) -> Result<TokenBuildingState, String> {
    match state.state_type {
        // character will be accumulated either way. figure out next state
        TokenBuildingStateType::Empty => {
            if character.is_whitespace() {
                return Ok(state);
            }

            let new_token_building_state_type = match character {
                '/' => TokenBuildingStateType::MaybeComment,
                '=' => TokenBuildingStateType::Equal,
                '!' => TokenBuildingStateType::Not,
                composable_operators!() => TokenBuildingStateType::ComposableOperator,
                ';' | grouping_characters!() => {
                    return accumulate_character_and_commit_accumulator(character, state)
                }
                _ if character.is_alphabetic() => TokenBuildingStateType::Alphabetic,
                _ if character.is_numeric() => TokenBuildingStateType::Numeric,
                _ => return unexpected_character_error(character, state),
            };

            Ok(accumulate_character(
                character,
                state,
                new_token_building_state_type,
            ))
        }
        TokenBuildingStateType::Equal => match character {
            '=' => accumulate_character_and_commit_accumulator(character, state),
            '/' | ';' | grouping_characters!() | _ if character.is_alphanumeric() => {
                commit_accumulator_and_single_character_token(character, state)
            }
            _ if character.is_whitespace() => commit_accumulator(state),
            _ => unexpected_character_error(character, state),
        },
        TokenBuildingStateType::Not => match character {
            '=' => accumulate_character_and_commit_accumulator(character, state),
            _ => unexpected_character_error(character, state),
        },
        TokenBuildingStateType::ComposableOperator => match character {
            '=' | composable_operators!() => {
                accumulate_character_and_commit_accumulator(character, state)
            }
            '/' | ';' | grouping_characters!() | _ if character.is_alphanumeric() => {
                commit_accumulator_and_single_character_token(character, state)
            }
            _ if character.is_whitespace() => commit_accumulator(state),
            _ => unexpected_character_error(character, state),
        },
        TokenBuildingStateType::Alphabetic => {
            match character {
                ';' | '=' | '/' | '!' | grouping_characters!() | composable_operators!() => {
                    commit_accumulator_and_begin_with_character(character, state)
                }
                // identifiers can be composed of letters and numbers but not viceversa
                _ if character.is_alphanumeric() => {
                    return Ok(accumulate_character(
                        character,
                        state,
                        TokenBuildingStateType::Alphabetic,
                    ))
                }
                _ if character.is_whitespace() => commit_accumulator(state),
                _ => return unexpected_character_error(character, state),
            }
        }
        TokenBuildingStateType::Numeric => match character {
            '=' | '/' | '!' | composable_operators!() | grouping_characters!() => {
                commit_accumulator_and_begin_with_character(character, state)
            }
            '.' => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::NumericPeriod,
            )),
            ';' => commit_accumulator_and_single_character_token(character, state),
            _ if character.is_numeric() => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::Numeric,
            )),
            _ if character.is_whitespace() => commit_accumulator(state),
            _ => unexpected_character_error(character, state),
        },
        TokenBuildingStateType::NumericPeriod => match character {
            _ if character.is_numeric() => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::NumericFloatingPoint,
            )),
            _ => unexpected_character_error(character, state),
        },
        TokenBuildingStateType::NumericFloatingPoint => match character {
            '=' | '/' | '!' | composable_operators!() | grouping_characters!() => {
                commit_accumulator_and_begin_with_character(character, state)
            }
            _ if character.is_numeric() => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::NumericFloatingPoint,
            )),
            _ if character.is_whitespace() => commit_accumulator(state),
            _ => unexpected_character_error(character, state),
        },
        TokenBuildingStateType::MaybeComment => match character {
            '/' => Ok(reset_state_with_state_type(
                state,
                TokenBuildingStateType::LineComment,
            )),
            '*' => Ok(reset_state_with_state_type(
                state,
                TokenBuildingStateType::BlockComment,
            )),
            '=' => accumulate_character_and_commit_accumulator(character, state),
            _ if character.is_whitespace() => commit_accumulator(state),
            _ => commit_accumulator_and_begin_with_character(character, state),
        },
        TokenBuildingStateType::LineComment => match character {
            // ignore everything but newlines
            '\n' => Ok(reset_state_with_state_type(
                state,
                TokenBuildingStateType::Empty,
            )),
            _ => Ok(state),
        },
        TokenBuildingStateType::BlockComment => match character {
            '*' => Ok(reset_state_with_state_type(
                state,
                TokenBuildingStateType::BlockCommentMaybeClosing,
            )),
            _ => Ok(state),
        },
        TokenBuildingStateType::BlockCommentMaybeClosing => match character {
            '/' => Ok(reset_state_with_state_type(
                state,
                TokenBuildingStateType::Empty,
            )),
            _ => Ok(state),
        },
    }
}

fn build_tokens(
    mut input: impl Iterator<Item = char>,
    state: TokenBuildingState,
) -> Result<TokenBuildingState, String> {
    match input.next() {
        Some(character) => build_tokens(input, handle_character(character, state)?),
        None => Ok(push_token(
            commit_accumulator(state)?,
            Token {
                value: None,
                token_type: TokenType::EOF,
            },
        )),
    }
}

pub fn lexical_analysis(
    input: impl Iterator<Item = char>,
) -> Result<Peekable<impl Iterator<Item = Token>>, String> {
    let token_building_state = build_tokens(
        input,
        TokenBuildingState {
            state_type: TokenBuildingStateType::Empty,
            accumulator: String::new(),
            token_vector: vec![],
        },
    )?;

    Ok(token_building_state.token_vector.into_iter().peekable())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_input_tokenizes_as(
        input: String,
        expected_tokens: Vec<(TokenType, Option<TokenValue>)>,
    ) {
        let result = lexical_analysis(input.chars());
        assert!(result.is_ok());

        let mut actual = result.unwrap();

        for (token_type, token_value) in expected_tokens {
            let next_option = actual.next();
            assert!(next_option.is_some());

            let actual_token = next_option.unwrap();
            assert_eq!(actual_token.token_type, token_type);
            assert_eq!(actual_token.value, token_value);
        }

        assert!(actual.next().is_none());
    }

    #[test]
    fn it_tokenizes_floating_point_values() {
        let input = String::from("123.4");
        let expected_tokens = vec![
            (TokenType::Constant, Some(TokenValue::Float(123.4))),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens);
    }

    #[test]
    fn it_tokenizes_integer_constants_as_floats() {
        let input = String::from("42");
        let expected_tokens = vec![
            (TokenType::Constant, Some(TokenValue::Float(42.0))),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens);
    }

    #[test]
    fn it_interrupts_numbers_when_grouping_characters_are_reached() {
        let input = String::from("24{9821}2)");
        let expected_tokens = vec![
            (TokenType::Constant, Some(TokenValue::Float(24.0))),
            (TokenType::CurlyOpening, None),
            (TokenType::Constant, Some(TokenValue::Float(9821.0))),
            (TokenType::CurlyClosing, None),
            (TokenType::Constant, Some(TokenValue::Float(2.0))),
            (TokenType::ParenthesisClosing, None),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens)
    }

    #[test]
    fn it_breaks_numbers_apart_using_whitespace() {
        let input = String::from("42 60 42.8 231");
        let expected_tokens = vec![
            (TokenType::Constant, Some(TokenValue::Float(42.0))),
            (TokenType::Constant, Some(TokenValue::Float(60.0))),
            (TokenType::Constant, Some(TokenValue::Float(42.8))),
            (TokenType::Constant, Some(TokenValue::Float(231.0))),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens);
    }

    #[test]
    fn it_tokenizes_keywords() {
        let input = String::from("for if");
        let expected_tokens = vec![
            (TokenType::For, None),
            (TokenType::If, None),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens);
    }

    #[test]
    fn it_tokenizes_identifiers() {
        let input = String::from("for if foo bar");
        let expected_tokens = vec![
            (TokenType::For, None),
            (TokenType::If, None),
            (
                TokenType::Identifier,
                Some(TokenValue::Lexeme("foo".into())),
            ),
            (
                TokenType::Identifier,
                Some(TokenValue::Lexeme("bar".into())),
            ),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens)
    }

    #[test]
    fn it_tokenizes_booleans() {
        let input = String::from("true false");
        let expected_tokens = vec![
            (TokenType::True, None),
            (TokenType::False, None),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens)
    }

    #[test]
    fn it_tokenizes_operators() {
        let input = String::from(" 1 + 2++ 3 +=4 -5 -- 6-= 7* 8*= 9/ 10/= 11== 12!=13");
        let expected_tokens = vec![
            (TokenType::Constant, Some(TokenValue::Float(1.0))),
            (TokenType::OperatorAddition, None),
            (TokenType::Constant, Some(TokenValue::Float(2.0))),
            (TokenType::OperatorIncrement, None),
            (TokenType::Constant, Some(TokenValue::Float(3.0))),
            (TokenType::OperatorIncreaseBy, None),
            (TokenType::Constant, Some(TokenValue::Float(4.0))),
            (TokenType::OperatorSubtraction, None),
            (TokenType::Constant, Some(TokenValue::Float(5.0))),
            (TokenType::OperatorDecrement, None),
            (TokenType::Constant, Some(TokenValue::Float(6.0))),
            (TokenType::OperatorDecreaseBy, None),
            (TokenType::Constant, Some(TokenValue::Float(7.0))),
            (TokenType::OperatorMultiplication, None),
            (TokenType::Constant, Some(TokenValue::Float(8.0))),
            (TokenType::OperatorMultiplyBy, None),
            (TokenType::Constant, Some(TokenValue::Float(9.0))),
            (TokenType::OperatorDivision, None),
            (TokenType::Constant, Some(TokenValue::Float(10.0))),
            (TokenType::OperatorDivideBy, None),
            (TokenType::Constant, Some(TokenValue::Float(11.0))),
            (TokenType::OperatorEqual, None),
            (TokenType::Constant, Some(TokenValue::Float(12.0))),
            (TokenType::OperatorNotEqual, None),
            (TokenType::Constant, Some(TokenValue::Float(13.0))),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens)
    }

    #[test]
    fn it_ignores_line_comments() {
        let input = String::from(
            "foo // this is the first line comment\n//this is the second line comment\nbar",
        );
        let expected_tokens = vec![
            (
                TokenType::Identifier,
                Some(TokenValue::Lexeme(String::from("foo"))),
            ),
            (
                TokenType::Identifier,
                Some(TokenValue::Lexeme(String::from("bar"))),
            ),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens)
    }

    #[test]
    fn it_ignores_block_comments() {
        let input = String::from("foo /*this\n\n is a block\n comment true false if / * ? */bar");
        let expected_tokens = vec![
            (
                TokenType::Identifier,
                Some(TokenValue::Lexeme(String::from("foo"))),
            ),
            (
                TokenType::Identifier,
                Some(TokenValue::Lexeme(String::from("bar"))),
            ),
            (TokenType::EOF, None),
        ];

        assert_input_tokenizes_as(input, expected_tokens)
    }
}
