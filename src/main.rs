use std::io::{stdin, Read};
mod expression;
mod statement;
mod token;

mod lexical_analysis;
mod syntax_analysis;

fn main() {
    env_logger::init();

    log::trace!("Starting input from standard input");
    let char_iter = stdin().bytes().filter_map(|b| b.ok()).map(|b| b as char);

    log::trace!("Staring lexical analysis");

    let mut token_stream = match lexical_analysis::lexical_analysis(char_iter) {
        Ok(value) => value,
        Err(message) => {
            log::error!("Failed lexical analysis: {message}");
            return;
        }
    };

    log::trace!("Lexical analysis completed");

    let syntax_statement = match syntax_analysis::syntax_analysis(&mut token_stream) {
        Ok(value) => value,
        Err(message) => {
            log::error!("Failed syntax analysis: {message}");
            return;
        }
    };

    println!("{}", syntax_statement);
}
