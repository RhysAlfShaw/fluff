use std::env;
use std::fs::File;
use std::io::prelude::*;

// Define the tokens for our language
#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    Number(i32),
    Plus,
    Minus,
}

// Define the AST nodes
#[derive(Debug, PartialEq)]
enum Expression {
    Number(i32),
    BinaryOp(Box<Expression>, Op, Box<Expression>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Plus,
    Minus,
}

// Lexer
fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut number = c.to_digit(10).unwrap();
                while let Some(next) = chars.peek() {
                    if next.is_digit(10) {
                        number = number * 10 + chars.next().unwrap().to_digit(10).unwrap();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number as i32));
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            _ => {}
        }
    }

    tokens
}

// Parser
fn parse(tokens: &[Token]) -> Expression {
    parse_expression(&mut tokens.iter().peekable())
}

fn parse_expression<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Expression
where
    I: Iterator<Item = &'a Token>,
{
    let mut lhs = match tokens.next() {
        Some(Token::Number(n)) => Expression::Number(*n),
        token => panic!("Expected number, found: {:?}", token),
    };

    while let Some(op) = tokens.peek().cloned() {
        lhs = match op {
            Token::Plus => {
                tokens.next();
                let rhs = parse_term(tokens);
                Expression::BinaryOp(Box::new(lhs), Op::Plus, Box::new(rhs))
            }
            Token::Minus => {
                tokens.next();
                let rhs = parse_term(tokens);
                Expression::BinaryOp(Box::new(lhs), Op::Minus, Box::new(rhs))
            }
            _ => break,
        }
    }

    lhs
}

fn parse_term<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Expression
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = match tokens.next() {
        Some(Token::Number(n)) => Expression::Number(*n),
        token => panic!("Expected number, found: {:?}", token),
    };

    loop {
        expr = match tokens.peek() {
            Some(Token::Plus) => {
                tokens.next();
                let rhs = parse_term(tokens);
                Expression::BinaryOp(Box::new(expr), Op::Plus, Box::new(rhs))
            }
            Some(Token::Minus) => {
                tokens.next();
                let rhs = parse_term(tokens);
                Expression::BinaryOp(Box::new(expr), Op::Minus, Box::new(rhs))
            }
            _ => break,
        }
    }

    expr
}

// Evaluator
fn eval(expr: &Expression) -> i32 {
    match expr {
        Expression::Number(n) => *n,
        Expression::BinaryOp(lhs, op, rhs) => {
            let lhs_val = eval(lhs);
            let rhs_val = eval(rhs);
            match op {
                Op::Plus => lhs_val + rhs_val,
                Op::Minus => lhs_val - rhs_val,
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let file_path = &args[1];
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read file");

    let tokens = lex(&input);
    let ast = parse(&tokens);
    let result = eval(&ast);
    println!("{}",result);
}