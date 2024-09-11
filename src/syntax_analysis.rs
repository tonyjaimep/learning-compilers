use trees::{tr, Tree};

use crate::lexeme::Lexeme;
use crate::token::{Token, TokenType};

fn lexeme_tree_from_token_stack(token_stack: &mut Vec<Token>) -> Result<Tree<Lexeme>, String> {
    while token_stack.last().is_some() {
        let token = token_stack.pop().unwrap();
        let lexeme = match Lexeme::try_from(&token) {
            Ok(l) => l,
            Err(_) => return Err(format!("Unable to parse token {}", token.value)),
        };

        match token.token_type {
            TokenType::Operator => {
                let mut root = tr(lexeme);
                let left_tree = lexeme_tree_from_token_stack(token_stack)?;
                let right_tree = lexeme_tree_from_token_stack(token_stack)?;
                root.push_back(left_tree);
                root.push_back(right_tree);
                return Ok(root);
            }
            TokenType::Constant => return Ok(Tree::new(lexeme)),
        }
    }

    Err(String::from("Unable to parse token stack"))
}

pub fn syntax_analysis(tokens: impl Iterator<Item = Token>) -> Result<Tree<Lexeme>, String> {
    let mut token_stack: Vec<Token> = tokens.collect::<Vec<Token>>();

    lexeme_tree_from_token_stack(&mut token_stack)
}
