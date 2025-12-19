use calculator::parser::Parser;
use calculator::tokeniser::Tokeniser;
use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();
    let equation = args.next().unwrap_or_else(|| {
        eprintln!("Expected argument equation, found no arguments");
        process::exit(0);
    });
    let tokens = Tokeniser::new(equation).run();

    let parsed = Parser::new(tokens).parse();

    let answer = parsed.evaluate();

    println!("The answer is: {answer:.2}");
}
