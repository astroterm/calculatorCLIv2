use std::process;
use crate::tokeniser::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_add()
    }

    fn parse_add(&mut self) -> Expr {
        let mut node = self.parse_mult();
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    node = match self.next() {
                        Some(Token::Plus) => Expr::Add {
                            left: Box::new(node),
                            right: Box::new(self.parse_mult()),
                        },
                        Some(Token::Minus) => Expr::Subtract {
                            left: Box::new(node),
                            right: Box::new(self.parse_mult()),
                        },
                        _ => unreachable!(),
                    };
                },
                _ => break
            }
        }
        node
    }

    fn parse_mult(&mut self) -> Expr {
        let mut node = self.parse_expo();
        while let Some(token) = self.peek() {
            match token {
                Token::Times | Token::Divide => {
                    node = match self.next() {
                        Some(Token::Times) => Expr::Multiply {
                            left: Box::new(node),
                            right: Box::new(self.parse_expo()),
                        },
                        Some(Token::Divide) => Expr::Divide {
                            left: Box::new(node),
                            right: Box::new(self.parse_expo()),
                        },
                        _ => unreachable!(),
                    };
                },
                _ => break
            }
        }
        node
    }

    fn parse_expo(&mut self) -> Expr {
        let mut node = self.parse_func();
        while let Some(token) = self.peek() {
            match token {
                Token::Caret => {
                    node = match self.next() {
                        Some(Token::Caret) => Expr::Exponent {
                            left: Box::new(node),
                            right: Box::new(self.parse_func()),
                        },
                        _ => unreachable!(),
                    };
                }
                _ => break
            }
        }
        node
    }

    fn parse_func(&mut self) -> Expr {
        match self.peek() {
            Some(Token::Sqrt) | Some(Token::Ln) | Some(Token::Sin) => {
                match self.next() {
                    Some(Token::Sqrt) => Expr::Sqrt {
                        expr: Box::new(self.parse_paren())
                    },
                    Some(Token::Ln) => Expr::Ln {
                        expr: Box::new(self.parse_paren())
                    },
                    Some(Token::Sin) => Expr::Sin {
                        angle: Box::new(self.parse_paren())
                    },
                    Some(Token::Cos) => Expr::Cos {
                        angle: Box::new(self.parse_paren())
                    },
                    Some(Token::Tan) => Expr::Tan {
                        angle: Box::new(self.parse_paren())
                    },
                    _ => unreachable!()
                }
            },
            _ => self.parse_paren()
        }
    }

    fn parse_paren(&mut self) -> Expr {
        match self.peek() {
            Some(Token::LeftParen) => {
                self.next();
                let expression = self.parse_add();
                match self.next() {
                    Some(Token::RightParen) => {
                        match self.peek() {
                            Some(Token::Factorial) => {
                                self.next();
                                Expr::Factorial {
                                    expr: Box::new(expression)
                                }
                            },
                            _ => expression
                        }
                    },
                    _ => {
                        eprintln!("Expected right parenthesis");
                        process::exit(0);
                    }
                }
            },
            Some(Token::Number(_)) => self.parse_num(),
            Some(other) => {
                eprintln!("Unexpected token while parsing parenthesis: {other:?}");
                process::exit(0);
            },
            None => unreachable!()
        }
    }

    fn parse_num(&mut self) -> Expr {
        match self.next() {
            Some(Token::Number(num)) =>{
                let num = num.clone();
                match self.peek() {
                    Some(Token::Factorial) => {
                        self.next();
                        Expr::Factorial {
                            expr: Box::new(Expr::Number(num))
                        }
                    },
                    _ => Expr::Number(num)
                }
            },
            Some(tok) => {
                eprintln!("Expected number, found: {tok:?}");
                process::exit(0);
            },
            None => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Add {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Subtract {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Multiply {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Divide {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Exponent {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Factorial {
        expr: Box<Expr>
    },
    Sqrt {
        expr: Box<Expr>
    },
    Ln {
        expr: Box<Expr>
    },
    Sin {
        angle: Box<Expr>
    },
    Cos {
        angle: Box<Expr>
    },
    Tan {
        angle: Box<Expr>
    },
    Number(f64),
}

impl Expr {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expr::Number(num) => *num as f64,
            Expr::Add        { left, right } => left.evaluate() + right.evaluate(),
            Expr::Subtract   { left, right } => left.evaluate() - right.evaluate(),
            Expr::Multiply   { left, right } => left.evaluate() * right.evaluate(),
            Expr::Divide     { left, right } => left.evaluate() / right.evaluate(),
            Expr::Exponent   { left, right } => left.evaluate().powf(right.evaluate()),

            Expr::Sin { angle } => angle.evaluate().to_radians().sin(),
            Expr::Cos { angle } => angle.evaluate().to_radians().cos(),
            Expr::Tan { angle } => angle.evaluate().to_radians().tan(),

            Expr::Factorial { expr } => (1..=expr.evaluate() as i64).product::<i64>() as f64,
            Expr::Sqrt      { expr } => expr.evaluate().sqrt(),
            Expr::Ln        { expr } => expr.evaluate().ln(),
        }
    }
}