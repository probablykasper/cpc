use std::time::{Instant};
use decimal::d128;

#[derive(Clone, Debug)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  Modulo,
  Percent,
  Caret,
  Factorial,
  LeftParen, // lexer only
  RightParen, // lexer only
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

#[derive(Clone, Copy, Debug)]
pub enum Unit {
  Nanosecond,
  Microsecond,
  Millisecond,
  Second,
  Minute,
  Hour,
  Day,
  Week,
  Month,
  Quarter,
  Year,
  Decade,
  Century,
  Milleniums,

  Millimeter,
  Centimeter,
  Decimeter,
  Meter,
  Kilometer,
  Inch,
  Foot,
  Yard,
  Mile,
  NauticalMile,

  SquareMeter,
  // etc

  CubicMeter,
  //etc

  
}

#[derive(Clone, Debug)]
pub enum Token {
  Operator(Operator),
  Number(d128),
  FunctionIdentifier(FunctionIdentifier),
  Constant(Constant),
  Paren, // parser only
  TextOperator(TextOperator),
  Negative, // parser only
  Unit(Unit),
}

pub type TokenVector = Vec<Token>;

mod lexer;
mod parser;

fn main() {
  let lex_start = Instant::now();
  
  use std::env;
  let args: Vec<String> = env::args().collect();
  let s = if args.len() >= 2 { &args[1] } else { "0.1" };

  match lexer::lex(s) {
    Ok(tokens) => {
      let lex_time = Instant::now().duration_since(lex_start).as_nanos() as f32;
      println!("Lexed TokenVector: {:?}", tokens);

      let parse_start = Instant::now();
      match parser::parse(&tokens) {
        Ok(ast) => {
          let parse_time = Instant::now().duration_since(parse_start).as_nanos() as f32;
          println!("Parsed AstNode: {:#?}", ast);
          println!("\u{23f1}  {:.3}ms lexing", lex_time/1000.0/1000.0);
          println!("\u{23f1}  {:.3}ms parsing", parse_time/1000.0/1000.0);
        },
        Err(e) => println!("parsing error: {}", e),
      }
    },
    Err(e) => println!("lexing error: {}", e),
  }
  
}
