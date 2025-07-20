use std::{iter, process};

pub struct Tokeniser {
    equation: String,
    tokens: Vec<Token>,
}


impl Tokeniser {
    pub fn new(equation: String) -> Self {
        Self {
            equation,
            tokens: vec![],
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut char_iter = self.equation.chars().peekable();
        
        while let Some(chr) = char_iter.next() {
            match chr {
                chr if chr.is_whitespace() => continue,
                '+' => self.tokens.push(Token::Plus),
                '-' => self.tokens.push(Token::Minus),
                '*' => self.tokens.push(Token::Times),
                '/' => self.tokens.push(Token::Divide),
                '(' => self.tokens.push(Token::LeftParen),
                ')' => self.tokens.push(Token::RightParen),
                '^' => self.tokens.push(Token::Caret),
                '!' => self.tokens.push(Token::Factorial),
                chr if chr.is_ascii_digit() => {
                    let num: f64 = iter::once(chr).chain(iter::from_fn(|| {
                        char_iter.by_ref().next_if(|c| c.is_ascii_digit() || *c == '.')
                    })) 
                        .collect::<String>()
                        .parse()
                        .expect("Failed to parse number");
                    self.tokens.push(Token::Number(num));
                },
                chr if chr.is_ascii_alphanumeric() => {
                    let func_name: String = iter::once(chr).chain(iter::from_fn(|| {
                        char_iter.by_ref().next_if(|c| c.is_ascii_alphanumeric())
                    })).collect::<String>();
                    match func_name.as_ref() {
                        "sqrt" => self.tokens.push(Token::Sqrt),
                        "ln" => self.tokens.push(Token::Ln),
                        "sin" => self.tokens.push(Token::Sin),
                        "cos" => self.tokens.push(Token::Cos),
                        "tan" => self.tokens.push(Token::Tan),
                        unimpl => {
                            eprintln!("Function not implemented yet: {}", unimpl);
                            process::exit(0);
                        }
                    };
                },
                chr => {
                    eprintln!("Unexpected character: {}", chr);
                    process::exit(0);
                }
            }
        }
        self.tokens.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Minus,
    Times,
    Divide,
    LeftParen,
    RightParen,
    Caret,
    Factorial,
    Sqrt,
    Ln,
    Sin,
    Cos,
    Tan,
    Number(f64)
}