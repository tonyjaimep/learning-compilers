use std::iter::Peekable;

use crate::token::*;

#[derive(Debug)]
enum TokenBuildingStateType {
    // empty stack
    Empty,
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
}

struct TokenBuildingState {
    pub state_type: TokenBuildingStateType,
    pub accumulator: String,
    pub token_vector: Vec<Token>,
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
        "Unexpected {} after {}, with state {}",
        if character.is_whitespace() {
            String::from("whitespace")
        } else {
            character.to_string()
        },
        if state.accumulator.is_empty() {
            String::from("empty string")
        } else {
            state.accumulator.to_string()
        },
        format!("{:?}", state.state_type)
    ))
}

fn commit_accumulator_and_begin_with_character(
    character: char,
    state: TokenBuildingState,
    state_type: TokenBuildingStateType,
) -> Result<TokenBuildingState, String> {
    let state_after_committing_accumulator = commit_accumulator(state)?;
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
    match character {
        '{' | '}' | '(' | ')' | ';' => match state.state_type {
            TokenBuildingStateType::NumericPeriod => unexpected_character_error(character, state),
            _ => commit_accumulator_and_single_character_token(character, state),
        },
        '=' => match state.state_type {
            TokenBuildingStateType::Numeric
            | TokenBuildingStateType::Alphabetic
            | TokenBuildingStateType::NumericFloatingPoint => {
                commit_accumulator_and_begin_with_character(
                    character,
                    state,
                    TokenBuildingStateType::Equal,
                )
            }
            TokenBuildingStateType::ComposableOperator | TokenBuildingStateType::Equal => {
                accumulate_character_and_commit_accumulator(character, state)
            }
            TokenBuildingStateType::Empty => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::Equal,
            )),
            _ => unexpected_character_error(character, state),
        },
        '/' | '*' | '<' | '>' | '+' | '-' => match state.state_type {
            TokenBuildingStateType::NumericPeriod => unexpected_character_error(character, state),
            TokenBuildingStateType::ComposableOperator => {
                accumulate_character_and_commit_accumulator(character, state)
            }
            TokenBuildingStateType::Empty => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::ComposableOperator,
            )),
            TokenBuildingStateType::Alphabetic
            | TokenBuildingStateType::NumericFloatingPoint
            | TokenBuildingStateType::Numeric => commit_accumulator_and_begin_with_character(
                character,
                state,
                TokenBuildingStateType::ComposableOperator,
            ),
            _ => unexpected_character_error(character, state),
        },
        '.' => match state.state_type {
            TokenBuildingStateType::Numeric => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::NumericPeriod,
            )),
            _ => unexpected_character_error(character, state),
        },
        _ if character.is_alphabetic() => match state.state_type {
            TokenBuildingStateType::Empty | TokenBuildingStateType::Alphabetic => Ok(
                accumulate_character(character, state, TokenBuildingStateType::Alphabetic),
            ),
            TokenBuildingStateType::NumericFloatingPoint
            | TokenBuildingStateType::NumericPeriod
            | TokenBuildingStateType::Numeric => unexpected_character_error(character, state),
            _ => commit_accumulator_and_begin_with_character(
                character,
                state,
                TokenBuildingStateType::Alphabetic,
            ),
        },
        _ if character.is_whitespace() => {
            return match state.state_type {
                TokenBuildingStateType::NumericPeriod => {
                    unexpected_character_error(character, state)
                }
                _ => commit_accumulator(state),
            };
        }
        _ if character.is_numeric() => match state.state_type {
            TokenBuildingStateType::NumericFloatingPoint
            | TokenBuildingStateType::NumericPeriod => Ok(accumulate_character(
                character,
                state,
                TokenBuildingStateType::NumericFloatingPoint,
            )),
            TokenBuildingStateType::Numeric | TokenBuildingStateType::Empty => Ok(
                accumulate_character(character, state, TokenBuildingStateType::Numeric),
            ),
            TokenBuildingStateType::Alphabetic => unexpected_character_error(character, state),
            _ => commit_accumulator_and_begin_with_character(
                character,
                state,
                TokenBuildingStateType::Numeric,
            ),
        },
        _ => unexpected_character_error(character, state),
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
    );

    Ok(token_building_state?.token_vector.into_iter().peekable())
}
