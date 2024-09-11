use crate::token::*;

fn build_token(input: char) -> Result<Token, String> {
    let token_type = match input {
        '0'..='9' => TokenType::Constant,
        '+' | '-' | '*' | '/' => TokenType::Operator,
        _ => return Err(format!("Unexpected {input}")),
    };

    let result_token = Token {
        value: input,
        token_type,
    };

    Ok(result_token)
}

pub fn lexical_analysis(
    input: impl Iterator<Item = char>,
) -> Result<impl Iterator<Item = Token>, String> {
    let analysis_result: Result<Vec<Token>, String> = input.map(build_token).collect();

    match analysis_result {
        Ok(collection) => Ok(collection.into_iter()),
        Err(message) => Err(message),
    }
}
