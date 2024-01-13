#![cfg_attr(
	feature = "cargo-clippy",
	allow(
		clippy::comparison_chain,
		clippy::if_same_then_else,
		clippy::match_like_matches_macro,
	)
)]
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
//! use cpc::eval;
//! use cpc::units::Unit;
//!
//! match eval("3m + 1cm", true, false) {
//!     Ok(answer) => {
//!         // answer: Number { value: 301, unit: Unit::Centimeter }
//!         println!("Evaluated value: {} {:?}", answer.value, answer.unit)
//!     },
//!     Err(e) => {
//!         println!("{e}")
//!     }
//! }
//! ```

use crate::units::Unit;
use decimal::d128;
use std::fmt::{self, Display};
use std::time::Instant;

/// Turns an [`AstNode`](parser::AstNode) into a [`Number`]
pub mod evaluator;
/// Turns a string into [`Token`]s
#[rustfmt::skip]
pub mod lexer;
#[rustfmt::skip]
mod lookup;
/// Turns [`Token`]s into an [`AstNode`](parser::AstNode)
pub mod parser;
/// Units, and functions you can use with them
#[rustfmt::skip]
pub mod units;

#[derive(Clone, Debug, PartialEq)]
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
	pub const fn new(value: d128, unit: Unit) -> Number {
		Number { value, unit }
	}
}
impl Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// 0.2/0.01 results in 2E+1, but if we add zero it becomes 20
		let value = self.value + d128!(0);
		let word = match self.value == d128!(1) {
			true => self.unit.singular(),
			false => self.unit.plural(),
		};
		let output = match word {
			"" => format!("{value}"),
			_ => format!("{value} {word}"),
		};
		write!(f, "{output}")
	}
}

#[derive(Clone, Debug, PartialEq)]
/// Math operators like [`Multiply`](Operator::Multiply), parentheses, etc.
pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide,
	Modulo,
	Caret,
	LeftParen,  // lexer only
	RightParen, // lexer only
}

#[derive(Clone, Debug, PartialEq)]
/// Unary operators like [`Percent`](UnaryOperator::Percent) and [`Factorial`](UnaryOperator::Factorial).
pub enum UnaryOperator {
	Percent,
	Factorial,
}

#[derive(Clone, Debug, PartialEq)]
/// A Text operator like [`To`](TextOperator::To) or [`Of`](TextOperator::Of).
pub enum TextOperator {
	To,
	Of,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
/// A constant like [`Pi`](Constant::Pi) or [`E`](Constant::E).
pub enum Constant {
	Pi,
	E,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
	Revolution,
}

#[derive(Clone, Debug, PartialEq)]
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

#[macro_export]
macro_rules! numtok {
	( $num:literal ) => {
		Token::Number(d128!($num))
	};
}

/// Evaluates a string into a resulting [`Number`].
///
/// Example:
/// ```rust
/// use cpc::eval;
/// use cpc::units::Unit;
///
/// match eval("3m + 1cm", true, false) {
///     Ok(answer) => {
///         // answer: Number { value: 301, unit: Unit::Centimeter }
///         println!("Evaluated value: {} {:?}", answer.value, answer.unit)
///     },
///     Err(e) => {
///         println!("{e}")
///     }
/// }
/// ```
pub fn eval(
	input: &str,
	allow_trailing_operators: bool,
	verbose: bool,
) -> Result<Number, String> {
	let lex_start = Instant::now();

	match lexer::lex(input, allow_trailing_operators) {
		Ok(tokens) => {
			let lex_time = Instant::now().duration_since(lex_start).as_nanos() as f32;
			if verbose {
				println!("Lexed TokenVector: {:?}", tokens);
			}

			let parse_start = Instant::now();
			match parser::parse(&tokens) {
				Ok(ast) => {
					let parse_time = Instant::now().duration_since(parse_start).as_nanos() as f32;
					if verbose {
						println!("Parsed AstNode: {:#?}", ast);
					}

					let eval_start = Instant::now();
					match evaluator::evaluate(&ast) {
						Ok(answer) => {
							let eval_time =
								Instant::now().duration_since(eval_start).as_nanos() as f32;

							if verbose {
								println!("Evaluated value: {} {:?}", answer.value, answer.unit);
								println!("\u{23f1}  {:.3}ms lexing", lex_time / 1000.0 / 1000.0);
								println!("\u{23f1}  {:.3}ms parsing", parse_time / 1000.0 / 1000.0);
								println!(
									"\u{23f1}  {:.3}ms evaluation",
									eval_time / 1000.0 / 1000.0
								);
							}

							Ok(answer)
						}
						Err(e) => Err(format!("Eval error: {}", e)),
					}
				}
				Err(e) => Err(format!("Parsing error: {}", e)),
			}
		}
		Err(e) => Err(format!("Lexing error: {}", e)),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn default_eval(input: &str) -> Number {
		eval(input, true, false).unwrap()
	}

	#[test]
	fn test_evaluations() {
		assert_eq!(default_eval("-2(-3)"), Number::new(d128!(6), Unit::NoUnit));
		assert_eq!(default_eval("-2(3)"), Number::new(d128!(-6), Unit::NoUnit));
		assert_eq!(default_eval("(3)-2"), Number::new(d128!(1), Unit::NoUnit));
		assert_eq!(default_eval("-1km to m"), Number::new(d128!(-1000), Unit::Meter));
		assert_eq!(default_eval("2*-3*0.5"), Number::new(d128!(-3), Unit::NoUnit));
		assert_eq!(default_eval("-3^2"), Number::new(d128!(-9), Unit::NoUnit));
		assert_eq!(default_eval("-1+2"), Number::new(d128!(1), Unit::NoUnit));
	}
}
