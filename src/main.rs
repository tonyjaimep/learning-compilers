use std::io::{stdin, Read};

use symbol_table::SymbolTable;
mod expression;
mod statement;
mod symbol_table;
mod token;

mod code_generation;
mod lexical_analysis;
mod semantic_analysis;
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

    let abstract_syntax_tree = match syntax_analysis::syntax_analysis(&mut token_stream) {
        Ok(value) => value,
        Err(message) => {
            log::error!("Failed syntax analysis: {message}");
            return;
        }
    };

    log::debug!("{:?}", abstract_syntax_tree);
    log::trace!("Syntax analysis completed");

    let mut symbol_table = SymbolTable::new();

    if let Err(message) =
        semantic_analysis::semantic_analysis(&abstract_syntax_tree, &mut symbol_table)
    {
        log::error!("Failed semantic analysis: {message}");
        return;
    }

    log::trace!("Semantic analysis completed");

    let mut icg_symbol_table = SymbolTable::new();

    let code_sequence = match code_generation::intermediate_code_generation(
        &abstract_syntax_tree,
        &mut icg_symbol_table,
    ) {
        Ok(value) => value,
        Err(message) => {
            log::error!("Failed semantic analysis: {message}");
            return;
        }
    };

    println!(
        "{}",
        code_sequence
            .into_iter()
            .map(|code| format!("{:?}", code))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
