use std::time::{Instant};
// use num::rational::BigRational;

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

// #[derive(Debug)]
// enum Number {
//   FloatNumber(i32),
//   BigRationalNumber(BigRational),
// }
// use Number::*;

#[derive(Debug)]
enum Token {
  Operator(Operator),
  // Number(Number),
}

type TokenVector = Vec<Token>;

fn lex(input: &str) -> Option<TokenVector> {

  let chars:Vec<char> = input.chars().collect();
  let mut tokens: TokenVector = vec![];
  
  let mut index = 0;
  while index < chars.len() {
    let value = chars[index];
    match value {
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

        // word
        
      },
      '.' | '0'..='9' => {

        // numbers
        if value == '.' {
          if !chars[index+1].is_digit(10) { return None };
        }

      },
      _ => {
        println!("{}", value);
        return None;
      },
    }
    index += 1;
  };
  return Some(tokens)
}

mod bignumbers;
fn main() {
  bignumbers::main();
  let now = Instant::now();
  
  // use std::env;
  // let s = &args[1];
  // let s = "3 +pi^ (1+1* 2)-pi^3 "; // = 3
  let s = "*+(/)";
  match lex(s) {
    Some(vector) => println!("{:?}", vector),
    None => println!("Error"),
  }
  
  let duration = Instant::now().duration_since(now).as_nanos() as f32;
  println!("\u{23f1}  {:.3}ms lexing", duration/1000.0/1000.0);
}
