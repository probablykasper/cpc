//! calculation + conversion
//! 
//! cpc parses and evaluates strings of math, with support for units and conversion. 128-bit decimal floating points are used for high accuracy.
//! 
//! cpc lets you mix units, so for example 1 km - 1m results in Number { value: 999, unit: Meter }.
//! 
//! Check out the [list of supported units](units/enum.Unit.html)
//! 
//! # Example usage
//! ```rust
//! use cpc::{eval};
//! use cpc::units::Unit;
//! 
//! match eval("3m + 1cm", true, Unit::Celsius, false) {
//!     Ok(answer) => {
//!         // answer: Number { value: 301, unit: Unit::Centimeter }
//!         println!("Evaluated value: {} {:?}", answer.value, answer.unit)
//!     },
//!     Err(e) => {
//!         println!("{}", e)
//!     }
//! }
//! ```

use std::time::{Instant};
use decimal::d128;
use crate::units::Unit;

/// Units, and functions you can use with them
pub mod units;
/// Turns a string into [`Token`]s
pub mod lexer;
/// Turns [`Token`]s into an [`AstNode`](parser::AstNode)
pub mod parser;
/// Turns an [`AstNode`](parser::AstNode) into a [`Number`]
pub mod evaluator;
mod lookup;

#[derive(Clone, Debug)]
/// A number with a `Unit`.
/// 
/// Example:
/// ```rust
/// use cpc::{eval,Number};
/// use cpc::units::Unit;
/// use decimal::d128;
/// 
/// let x = Number {
///   value: d128!(100),
///   unit: Unit::Meter,
/// };
/// ```
pub struct Number {
  /// The number part of a [`Number`] struct
  pub value: d128,
  /// The unit of a [`Number`] struct. This can be [`NoType`](units::UnitType::NoType)
  pub unit: Unit,
}

impl Number {
  pub fn new(value: d128, unit: Unit) -> Number {
    Number {
      value: value,
      unit: unit,
    }
  }
}

#[derive(Clone, Debug)]
/// Math operators like [`Multiply`](Operator::Multiply), parentheses, etc.
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  Modulo,
  Caret,
  LeftParen, // lexer only
  RightParen, // lexer only
}

#[derive(Clone, Debug)]
/// Unary operators like [`Percent`](UnaryOperator::Percent) and [`Factorial`](UnaryOperator::Factorial).
pub enum UnaryOperator {
  Percent,
  Factorial,
}

#[derive(Clone, Debug)]
/// A Text operator like [`To`](TextOperator::To) or [`Of`](TextOperator::Of).
pub enum TextOperator {
  To,
  Of,
}

#[derive(Clone, Debug)]
/// A named number like [`Million`](NamedNumber::Million).
pub enum NamedNumber {
  Hundred,
  Thousand,
  Million,
  Billion,
  Trillion,
  Quadrillion,
  Quintillion,
  Sextillion,
  Septillion,
  Octillion,
  Nonillion,
  Decillion,
  Undecillion,
  Duodecillion,
  Tredecillion,
  Quattuordecillion,
  Quindecillion,
  Sexdecillion,
  Septendecillion,
  Octodecillion,
  Novemdecillion,
  Vigintillion,
  Centillion,
  Googol,
}

#[derive(Clone, Debug)]
/// A constant like [`Pi`](Constant::Pi) or [`E`](Constant::E).
pub enum Constant {
  Pi,
  E,
}

#[derive(Clone, Debug)]
/// Functions identifiers like [`Sqrt`](FunctionIdentifier::Sqrt), [`Sin`](FunctionIdentifier::Sin), [`Round`](FunctionIdentifier::Round), etc.
pub enum FunctionIdentifier {
  Sqrt,
  Cbrt,

  Log,
  Ln,
  Exp,
  
  Round,
  Ceil,
  Floor,
  Abs,

  Sin,
  Cos,
  Tan,
}

#[derive(Clone, Debug)]
/// A temporary enum used by the [`lexer`] to later determine what [`Token`] it is.
/// 
/// For example, when a symbol like `%` is found, the lexer turns it into a
/// the [`PercentChar`](LexerKeyword::PercentChar) variant
/// and then later it checks the surrounding [`Token`]s and,
/// dependingon them, turns it into a [`Percent`](UnaryOperator::Percent) or
/// [`Modulo`](Operator::Modulo) [`Token`].
pub enum LexerKeyword {
  Per,
  PercentChar,
  In,
  DoubleQuotes,
  Mercury,
  Hg,
  PoundForce,
  Force,
}

#[derive(Clone, Debug)]
/// A token like a [`Number`](Token::Number), [`Operator`](Token::Operator), [`Unit`](Token::Unit) etc.
/// 
/// Strings can be divided up into these tokens by the [`lexer`], and then put into the [`parser`].
pub enum Token {
  Operator(Operator),
  UnaryOperator(UnaryOperator),
  Number(d128),
  FunctionIdentifier(FunctionIdentifier),
  Constant(Constant),
  /// Used by the parser only
  Paren,
  /// Used by the lexer only
  Per,
  /// Used by the parser only
  LexerKeyword(LexerKeyword),
  TextOperator(TextOperator),
  NamedNumber(NamedNumber),
  /// The `-` symbol, specifically when used as `-5` and not `5-5`. Used by the parser only
  Negative,
  Unit(units::Unit),
}

/// Evaluates a string into a resulting [`Number`].
/// 
/// Example:
/// ```rust
/// use cpc::{eval};
/// use cpc::units::Unit;
/// 
/// match eval("3m + 1cm", true, Unit::Celsius, false) {
///     Ok(answer) => {
///         // answer: Number { value: 301, unit: Unit::Centimeter }
///         println!("Evaluated value: {} {:?}", answer.value, answer.unit)
///     },
///     Err(e) => {
///         println!("{}", e)
///     }
/// }
/// ```
pub fn eval(input: &str, allow_trailing_operators: bool, default_degree: Unit, verbose: bool) -> Result<Number, String> {

  let lex_start = Instant::now();

  match lexer::lex(input, allow_trailing_operators, default_degree) {
    Ok(tokens) => {
      let lex_time = Instant::now().duration_since(lex_start).as_nanos() as f32;
      if verbose == true { println!("Lexed TokenVector: {:?}", tokens); }

      let parse_start = Instant::now();
      match parser::parse(&tokens) {
        Ok(ast) => {
          let parse_time = Instant::now().duration_since(parse_start).as_nanos() as f32;
          if verbose == true { println!("Parsed AstNode: {:#?}", ast); }

          let eval_start = Instant::now();
          match evaluator::evaluate(&ast) {
            Ok(answer) => {
              let eval_time = Instant::now().duration_since(eval_start).as_nanos() as f32;

              if verbose == true {
                println!("Evaluated value: {} {:?}", answer.value, answer.unit);
                println!("\u{23f1}  {:.3}ms lexing", lex_time/1000.0/1000.0);
                println!("\u{23f1}  {:.3}ms parsing", parse_time/1000.0/1000.0);
                println!("\u{23f1}  {:.3}ms evaluation", eval_time/1000.0/1000.0);
              }

              return Ok(answer)
            },
            Err(e) => Err(format!("Eval error: {}", e)),
          }
          
        },
        Err(e) => Err(format!("Parsing error: {}", e)),
      }

    },
    Err(e) => Err(format!("Lexing error: {}", e)),
  }
  
}
