use crate::tokens::Token;

pub fn print_tokens(tokens: &[Token]) {
    println!("Tokens:");
    for (index, token) in tokens.iter().enumerate() {
        print!("{:?}", token.token_type);
        if index < tokens.len() - 1 {
            print!(", ")
        }
    }
    println!();
}
