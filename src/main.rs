use std::time::{Instant};
use decimal::d128;

#[derive(Debug)]
pub enum Operator {
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

#[derive(Debug)]
pub enum Constant {
  Pi,
  EulersNumber,
}

#[derive(Debug)]
pub enum Token {
  Operator(Operator),
  Number(d128),
  Function(String),
  Constant(Constant),
}

pub type TokenVector = Vec<Token>;

mod lexer;

fn main() {
  let now = Instant::now();
  
  use std::env;
  let args: Vec<String> = env::args().collect();
  let s = if args.len() == 2 { &args[1] } else { "0.1" };

  match lexer::lex(s) {
    Ok(vector) => println!("{:?}", vector),
    Err(e) => println!("lexing error: {}", e),
  }
  
  let duration = Instant::now().duration_since(now).as_nanos() as f32;
  println!("\u{23f1}  {:.3}ms lexing", duration/1000.0/1000.0);
}
