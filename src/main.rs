use std::fmt::Display;
use std::io::stdin;

use trees::Node;

mod lexeme;
mod token;

mod lexical_analysis;
mod syntax_analysis;

fn tree_to_string<T: Display>(node: &Node<T>) -> String {
    if node.has_no_child() {
        node.data().to_string()
    } else {
        format!(
            "{}( {})",
            node.data(),
            node.iter()
                .fold(String::new(), |s, c| format!("{}{} ", s, tree_to_string(c)))
        )
    }
}

fn main() {
    for line in stdin().lines() {
        let input = match line {
            Ok(input) => input,
            Err(_) => break,
        };

        if input.is_empty() {
            break;
        }

        let token_stream = match lexical_analysis::lexical_analysis(input.chars()) {
            Ok(value) => value,
            Err(message) => {
                println!("Failed lexical analysis: {message}");
                continue;
            }
        };

        let syntax_tree = match syntax_analysis::syntax_analysis(token_stream) {
            Ok(value) => value,
            Err(message) => {
                println!("Failed syntax analysis: {message}");
                continue;
            }
        };

        println!("{}", tree_to_string(&syntax_tree));
    }
}
