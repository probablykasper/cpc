use std::time::{Instant};
use decimal::d128;

#[derive(Clone, Debug)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  Modulo,
  Caret,
  // Percent,
  // Factorial,
  LeftParen, // lexer only
  RightParen, // lexer only
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
  Percent,
  Factorial,
}

#[derive(Clone, Debug)]
pub enum TextOperator {
  To,
  Of,
}

#[derive(Clone, Debug)]
pub enum Constant {
  Pi,
  E,
}

#[derive(Clone, Debug)]
pub enum FunctionIdentifier {
  Sqrt,
  Cbrt,

  Log,
  Ln,
  Exp,
  
  Ceil,
  Floor,
  Round,
  Fabs,

  Sin,
  Cos,
  Tan,
  Asin,
  Acos,
  Atan,
  Sinh,
  Cosh,
  Tanh,
  Asinh,
  Acosh,
  Atanh,
}

mod units;

#[derive(Clone, Debug)]
pub enum Token {
  Operator(Operator),
  UnaryOperator(UnaryOperator),
  Number(d128),
  FunctionIdentifier(FunctionIdentifier),
  Constant(Constant),
  Paren, // parser only
  TextOperator(TextOperator),
  Negative, // parser only
  Unit(units::Unit),
}

pub type TokenVector = Vec<Token>;

mod lexer;
mod parser;
mod evaluator;

fn main() {
  let lex_start = Instant::now();
  
  use std::env;
  let args: Vec<String> = env::args().collect();
  let s = if args.len() >= 2 { &args[1] } else { "0.1" };

  match lexer::lex(s) {
    Ok(tokens) => {
      let lex_time = Instant::now().duration_since(lex_start).as_nanos() as f32;
      // println!("Lexed TokenVector: {:?}", tokens);

      let parse_start = Instant::now();
      match parser::parse(&tokens) {
        Ok(ast) => {
          let parse_time = Instant::now().duration_since(parse_start).as_nanos() as f32;
          // println!("Parsed AstNode: {:#?}", ast);

          let eval_start = Instant::now();
          match evaluator::evaluate(&ast) {
            Ok(answer) => {
              let eval_time = Instant::now().duration_since(eval_start).as_nanos() as f32;
              println!("Evaluated answer: {:#?}", answer);

              println!("\u{23f1}  {:.3}ms lexing", lex_time/1000.0/1000.0);
              println!("\u{23f1}  {:.3}ms parsing", parse_time/1000.0/1000.0);
              println!("\u{23f1}  {:.3}ms evaluation", eval_time/1000.0/1000.0);
            },
            Err(e) => println!("Eval error: {}", e),
          }
          
        },
        Err(e) => println!("Parsing error: {}", e),
      }
    },
    Err(e) => println!("Lexing error: {}", e),
  }
  
}
