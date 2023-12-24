mod lexer;
mod token;
use token::Token;
use lexer::lex;

fn main() {
    let input = String::from("Your test string here"); // Replace with dynamic input if needed
    let tokens = lex(&input);

    for token in tokens {
        println!("{:?}", token); // Display each token
    }
}
