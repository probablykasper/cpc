use std::time::{Instant};
// use num_rational::BigRational;
use decimal::d128;

#[derive(Debug)]
enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  PercentOrModulo,
  Caret,
  Factorial,
  LeftParen,
  RightParen,
}
use Operator::*;

#[derive(Debug)]
enum Token {
  Operator(Operator),
  Number(d128),
}

type TokenVector = Vec<Token>;

fn lex(input: &str) -> Option<TokenVector> {

  let mut chars = input.chars().enumerate().peekable();
  let mut tokens: TokenVector = vec![];
  
  while let Some((index, current_char)) = chars.next() {
    match current_char {
      '+' => tokens.push(Token::Operator(Plus)),
      '-' => tokens.push(Token::Operator(Minus)),
      '*' => tokens.push(Token::Operator(Multiply)),
      '/' => tokens.push(Token::Operator(Divide)),
      '%' => tokens.push(Token::Operator(PercentOrModulo)),
      '^' => tokens.push(Token::Operator(Caret)),
      '!' => tokens.push(Token::Operator(Factorial)),
      '(' => tokens.push(Token::Operator(LeftParen)),
      ')' => tokens.push(Token::Operator(RightParen)),
      ',' => continue,
      value if value.is_whitespace() => continue,
      value if value.is_alphabetic() => {

      },
      '.' | '0'..='9' => {

        let start_index = index;
        let mut end_index = index+1;
        while let Some((_idk, current_char)) = chars.peek() {
          if current_char == &'.' || current_char.is_digit(10) {
            chars.next();
            end_index += 1;
          } else {
            break;
          }
        }
        match &input[start_index..end_index].parse::<d128>() {
          Ok(number) => {
            tokens.push(Token::Number(*number));
            println!("parsed as d128: {}", number);
          },
          Err(e) => {
            println!("{:?}", e);
            return None;
          }
        };

      },
      _ => {
        println!("{}", current_char);
        return None;
      },
    }
  };
  return Some(tokens)
}

fn main() {
  let now = Instant::now();
  
  use std::env;
  let args: Vec<String> = env::args().collect();
  let s = if args.len() == 2 { &args[1] } else { "0.1" };

  match lex(s) {
    Some(vector) => println!("{:?}", vector),
    None => println!("Error"),
  }
  
  let duration = Instant::now().duration_since(now).as_nanos() as f32;
  println!("\u{23f1}  {:.3}ms lexing", duration/1000.0/1000.0);
}
