use crate::lookup::{lookup_factorial, lookup_named_number};
use crate::parser::AstNode;
use crate::units::{add, convert, divide, modulo, multiply, pow, subtract, Unit, UnitType};
use crate::Constant::{Pi, E};
use crate::FunctionIdentifier::*;
use crate::Operator::{Caret, Divide, Minus, Modulo, Multiply, Plus};
use crate::TextOperator::{Of, To};
use crate::UnaryOperator::{Factorial, Percent};
use crate::{Number, Token};
use fastnum::{dec128 as d, D128};

/// Evaluate an [`AstNode`] into a [`Number`]
pub fn evaluate(ast: &AstNode) -> Result<Number, String> {
	let answer = evaluate_node(ast)?;
	Ok(answer)
}

/// Returns the factorial of a [`struct@d128`] up to `1000!` without doing any math
///
/// Factorials do not work with decimal numbers.
///
/// All return values of this function are hard-coded.
pub fn factorial(input: D128) -> D128 {
	lookup_factorial(input.try_into().unwrap())
}

/// Returns the square root of a [`struct@d128`]
pub fn sqrt(input: D128) -> D128 {
	let mut n = d!(1);
	let half = d!(0.5);
	for _ in 0..10 {
		n = (n + input / n) * half;
	}
	n
}

/// Returns the cube root of a [`struct@d128`]
pub fn cbrt(input: D128) -> D128 {
	let mut n: D128 = input;
	// hope that 20 iterations makes it accurate enough
	let three = d!(3);
	for _ in 0..20 {
		let z2 = n * n;
		n = n - ((n * z2 - input) / (three * z2));
	}
	n
}

/// Returns the sine of a [`struct@d128`]
pub fn sin(input: D128) -> D128 {
	let result =input.sin();
	match result.is_zero() {
		true => D128::ZERO,
		false => result,
	}
}

/// Returns the cosine of a [`struct@d128`]
pub fn cos(input: D128) -> D128 {
	input.cos()
}

/// Returns the tangent of a [`struct@d128`]
pub fn tan(input: D128) -> D128 {
	input.tan()
}

/// Evaluate an [`AstNode`] into a [`Number`]
fn evaluate_node(ast_node: &AstNode) -> Result<Number, String> {
	let token = &ast_node.token;
	let children = &ast_node.children;
	match token {
		Token::Number(number) => Ok(Number::new(*number, Unit::NoUnit)),
		Token::Constant(constant) => match constant {
			Pi => Ok(Number::new(
				D128::PI,
				Unit::NoUnit,
			)),
			E => Ok(Number::new(
				D128::E,
				Unit::NoUnit,
			)),
		},
		Token::FunctionIdentifier(function) => {
			let child_node = children.get(0).ok_or("Paren has no child[0]")?;
			let child_answer = evaluate_node(child_node)?;
			match function {
				Cbrt => {
					if child_answer.unit.category() == UnitType::NoType {
						let result = cbrt(child_answer.value);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("log() only accepts UnitType::NoType".to_string())
					}
				}
				Sqrt => {
					if child_answer.unit.category() == UnitType::NoType {
						let result = sqrt(child_answer.value);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("log() only accepts UnitType::NoType".to_string())
					}
				}
				Log => {
					if child_answer.unit.category() == UnitType::NoType {
						let result = child_answer.value.log10();
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("log() only accepts UnitType::NoType".to_string())
					}
				}
				Ln => {
					if child_answer.unit.category() == UnitType::NoType {
						let unrounded_result = child_answer.value.ln();
						let result = unrounded_result.quantize(unrounded_result * d!(10));
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("ln() only accepts UnitType::NoType".to_string())
					}
				}
				Exp => {
					if child_answer.unit.category() == UnitType::NoType {
						let result = child_answer.value.exp(child_answer.value);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("exp() only accepts UnitType::NoType".to_string())
					}
				}
				Round => {
					// .quantize() rounds .5 to nearest even integer, so we correct that
					let mut result = child_answer.value.quantize(d!(1));
					let rounding_change = result - child_answer.value;
					// If the result was rounded down by 0.5, correct by +1
					if rounding_change == d!(-0.5) {
						result += d!(1);
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Ceil => {
					let mut result = child_answer.value.quantize(d!(1));
					let rounding_change = result - child_answer.value;
					if rounding_change.is_negative() {
						result += d!(1);
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Floor => {
					let mut result = child_answer.value.quantize(d!(1));
					let rounding_change = result - child_answer.value;
					if !rounding_change.is_negative() {
						result -= d!(1);
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Abs => {
					let mut result = child_answer.value.abs();
					let rounding_change = result - child_answer.value;
					if rounding_change == d!(-0.5) {
						result += d!(1);
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Sin => {
					let result = sin(child_answer.value);
					Ok(Number::new(result, child_answer.unit))
				}
				Cos => {
					let result = cos(child_answer.value);
					Ok(Number::new(result, child_answer.unit))
				}
				Tan => {
					let result = tan(child_answer.value);
					Ok(Number::new(result, child_answer.unit))
				}
			}
		}
		Token::Unit(unit) => {
			let child_node = children.get(0).ok_or("Unit has no child[0]")?;
			let child_answer = evaluate_node(child_node)?;
			Ok(Number::new(child_answer.value, *unit))
		}
		Token::Negative => {
			let child_node = children.get(0).ok_or("Negative has no child[0]")?;
			let child_answer = evaluate_node(child_node)?;
			Ok(Number::new(-child_answer.value, child_answer.unit))
		}
		Token::Paren => {
			let child_node = children.get(0).ok_or("Paren has no child[0]")?;
			evaluate_node(child_node)
		}
		Token::UnaryOperator(operator) => {
			let child_node = children
				.get(0)
				.ok_or(format!("Token {:?} has no child[0]", token))?;
			let child_answer = evaluate_node(child_node)?;
			match operator {
				Percent => Ok(Number::new(
					child_answer.value / d!(100),
					child_answer.unit,
				)),
				Factorial => {
					let result = factorial(child_answer.value);
					if result.is_nan() {
						return Err(
							"Can only perform factorial on integers from 0 to 1000".to_string()
						);
					}
					Ok(Number::new(result, child_answer.unit))
				}
			}
		}
		Token::NamedNumber(named_number) => {
			let child_node = children
				.get(0)
				.ok_or(format!("Token {:?} has no child[0]", token))?;
			let named_number_value = lookup_named_number(named_number);
			if let Token::NamedNumber(child_nn) = &child_node.token {
				let child_nn_value = lookup_named_number(child_nn);
				if child_nn_value > named_number_value {
					return Err(format!("Unexpected smaller token {:?}", token));
				}
			}
			let child_answer = evaluate_node(child_node)?;
			let result = child_answer.value * named_number_value;
			Ok(Number::new(result, child_answer.unit))
		}
		Token::TextOperator(operator) => {
			let left_child = children
				.get(0)
				.ok_or(format!("Token {:?} has no child[0]", token))?;
			let right_child = children
				.get(1)
				.ok_or(format!("Token {:?} has no child[1]", token))?;

			match operator {
				To => {
					if let Token::Unit(right_unit) = right_child.token {
						let left = evaluate_node(left_child)?;
						let result = convert(left, right_unit)?;
						Ok(result)
					} else {
						Err("Right side of To operator needs to be a unit".to_string())
					}
				}
				Of => {
					let left = evaluate_node(left_child)?;
					let right = evaluate_node(right_child)?;
					if left.unit == Unit::NoUnit {
						Ok(Number::new(left.value * right.value, right.unit))
					} else {
						Err("Left side of the Of operator must be NoUnit".to_string())
					}
				}
			}
		}
		Token::Operator(operator) => {
			let left_child = children
				.get(0)
				.ok_or(format!("Token {:?} has no child[0]", token))?;
			let right_child = children
				.get(1)
				.ok_or(format!("Token {:?} has no child[1]", token))?;
			let left = evaluate_node(left_child)?;
			let right = evaluate_node(right_child)?;
			match operator {
				Plus => Ok(add(left, right)?),
				Minus => Ok(subtract(left, right)?),
				Multiply => Ok(multiply(left, right)?),
				Divide => Ok(divide(left, right)?),
				Modulo => Ok(modulo(left, right)?),
				Caret => Ok(pow(left, right)?),
				_ => Err(format!("Unexpected operator {:?}", operator)),
			}
		}
		_ => Err(format!("Unexpected token {:?}", token)),
	}
}

#[cfg(test)]
mod tests {
	use crate::eval;
	use super::*;

	fn eval_default<'a>(input: &'a str) -> Number {
		let result = eval(input, true, false).unwrap();
		result
	}
	fn eval_num<'a>(input: &'a str) -> String {
		let result = eval(input, true, false).unwrap();
		assert_eq!(result.unit, Unit::NoUnit);

		result.to_string()
	}

	#[test]
	fn test_evaluations() {
		assert_eq!(eval_num("-2(-3)"), "6");
		assert_eq!(eval_num("-2(3)"), "-6");
		assert_eq!(eval_num("(3)-2"), "1");
		assert_eq!(eval_default("-1km to m"), Number::new(d!(-1000), Unit::Meter));
		assert_eq!(eval_num("2*-3*0.5"), "-3");
		assert_eq!(eval_num("-3^2"), "-9");
		assert_eq!(eval_num("-1+2"), "1");
	}

	#[test]
	fn test_functions() {
		assert_eq!(eval_num("cbrt(125)"), "5");

		assert_eq!(eval_num("sqrt(25)"), "5");

		assert_eq!(eval_num("log(100)"), "2");
		assert_eq!(eval_num("log(2)"), "0.301029995663981195213738894724493026768");

		assert_eq!(eval_num("ln(1)"), "0");
		assert_eq!(eval_num("ln(2)"), "0.6931471805599453094172321214581765681");
		assert_eq!(eval_num("ln(e)"), "1");
		assert_eq!(eval_num("ln(e^2)"), "2");

		assert_eq!(eval_num("round(1.4)"), "1");
		assert_eq!(eval_num("round(1.6)"), "2");
		assert_eq!(eval_num("round(1.5)"), "2");
		assert_eq!(eval_num("round(2.5)"), "3");

		assert_eq!(eval_num("ceil(1.5)"), "2");
		assert_eq!(eval_num("ceil(-1.5)"), "-1");

		assert_eq!(eval_num("floor(1.5)"), "1");
		assert_eq!(eval_num("floor(-1.5)"), "-2");

		assert_eq!(eval_num("abs(-3)"), "3");

		assert_eq!(eval_num("sin(2)"), "0.9092974268256816953960198659117448427");
		assert_eq!(eval_num("sin(-2)"), "-0.9092974268256816953960198659117448427");
	}
}
