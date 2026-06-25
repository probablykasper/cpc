use crate::Constant::{E, Pi};
use crate::FunctionIdentifier::*;
use crate::Operator::{Caret, Divide, Minus, Modulo, Multiply, Plus};
use crate::TextOperator::{Of, To};
use crate::UnaryOperator::{Factorial, Percent};
use crate::lookup::{lookup_factorial, lookup_named_number};
use crate::parser::AstNode;
use crate::units::{add, convert, divide, modulo, multiply, pow, subtract};
use crate::{Number, Token};
use fastnum::decimal::Context;
use fastnum::{D128, dec128 as d, decimal::RoundingMode};

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

/// Returns the sine of a [`struct@d128`]
pub fn sin(input: D128) -> D128 {
	let result = input.sin();
	// D128::PI is a finite-precision approximation of pi, so sin(pi) lands a few
	// ulp away from zero rather than exactly zero. Snap sub-precision residue to 0.
	if result.abs() < d!(1E-30) {
		D128::ZERO
	} else {
		result
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

/// Turn `old` into `new` without updating the signal
fn replace_without_updating_signals(old: D128, new: D128) -> D128 {
	let new_without_signal = D128::parse_str(&new.to_string(), Context::default());
	old - old + new_without_signal
}

/// Evaluate an [`AstNode`] into a [`Number`]
fn evaluate_node(ast_node: &AstNode) -> Result<Number, String> {
	let token = &ast_node.token;
	let children = &ast_node.children;
	match token {
		Token::Number(number) => Ok(Number::new_unitless(*number)),
		Token::Constant(constant) => match constant {
			Pi => Ok(Number::new_unitless(D128::PI)),
			E => Ok(Number::new_unitless(D128::E)),
		},
		Token::FunctionIdentifier(function) => {
			let child_node = children.get(0).ok_or("Paren has no child[0]")?;
			let child_answer = evaluate_node(child_node)?;
			match function {
				Sqrt => {
					if child_answer.is_unitless() {
						let mut result = child_answer.value.sqrt();
						let result_with_old_signals =
							replace_without_updating_signals(child_answer.value, result);
						let result_squared = result_with_old_signals * result_with_old_signals;
						// if result^2 is exact and equals the original, we avoid the OP_INEXACT signal
						if !result_squared.is_op_inexact() && result_squared == child_answer.value {
							result = replace_without_updating_signals(child_answer.value, result);
						}
						Ok(Number::with_unit(result, child_answer.unit))
					} else {
						Err("sqrt() only accepts unitless numbers".to_string())
					}
				}
				Cbrt => {
					if child_answer.is_unitless() {
						let mut result = child_answer.value.cbrt();
						let result_with_old_signals =
							replace_without_updating_signals(child_answer.value, result);
						let result_squared = result_with_old_signals
							* result_with_old_signals
							* result_with_old_signals;
						// if result^2 is exact and equals the original, we avoid the OP_INEXACT signal
						if !result_squared.is_op_inexact() && result_squared == child_answer.value {
							result = replace_without_updating_signals(child_answer.value, result);
						}
						Ok(Number::with_unit(result, child_answer.unit))
					} else {
						Err("cbrt() only accepts unitless numbers".to_string())
					}
				}
				Log => {
					if child_answer.is_unitless() {
						let result = child_answer.value.log10();
						Ok(Number::with_unit(result, child_answer.unit))
					} else {
						Err("log() only accepts unitless numbers".to_string())
					}
				}
				Ln => {
					if child_answer.is_unitless() {
						let result = child_answer.value.ln();
						Ok(Number::with_unit(result, child_answer.unit))
					} else {
						Err("ln() only accepts unitless numbers".to_string())
					}
				}
				Exp => {
					if child_answer.is_unitless() {
						let result = child_answer.value.exp();
						Ok(Number::with_unit(result, child_answer.unit))
					} else {
						Err("exp() only accepts unitless numbers".to_string())
					}
				}
				Round => {
					// Round half away from zero (HalfUp) so round(2.5) == 3, not 2.
					let result = child_answer
						.value
						.with_rounding_mode(RoundingMode::HalfUp)
						.round(0);
					let result = replace_without_updating_signals(child_answer.value, result);
					Ok(Number::with_unit(result, child_answer.unit))
				}
				Ceil => {
					let result = child_answer.value.ceil();
					let result = replace_without_updating_signals(child_answer.value, result);
					Ok(Number::with_unit(result, child_answer.unit))
				}
				Floor => {
					let result = child_answer.value.floor();
					let result = replace_without_updating_signals(child_answer.value, result);
					Ok(Number::with_unit(result, child_answer.unit))
				}
				Abs => {
					let result = child_answer.value.abs();
					Ok(Number::with_unit(result, child_answer.unit))
				}
				Sin => {
					let result = sin(child_answer.value);
					Ok(Number::with_unit(result, child_answer.unit))
				}
				Cos => {
					let result = cos(child_answer.value);
					Ok(Number::with_unit(result, child_answer.unit))
				}
				Tan => {
					let result = tan(child_answer.value);
					Ok(Number::with_unit(result, child_answer.unit))
				}
			}
		}
		Token::Unit(unit) => {
			let child_node = children.get(0).ok_or("Unit has no child[0]")?;
			let child_answer = evaluate_node(child_node)?;
			Ok(Number::with_basic_unit(child_answer.value, *unit))
		}
		Token::Negative => {
			let child_node = children.get(0).ok_or("Negative has no child[0]")?;
			let child_answer = evaluate_node(child_node)?;
			Ok(Number::with_unit(-child_answer.value, child_answer.unit))
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
				Percent => Ok(Number::with_unit(
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
					Ok(Number::with_unit(result, child_answer.unit))
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
			Ok(Number::with_unit(result, child_answer.unit))
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
						let result = convert(left, vec![(right_unit, 1)])?;
						Ok(result)
					} else {
						Err("Right side of To operator needs to be a unit".to_string())
					}
				}
				Of => {
					let left = evaluate_node(left_child)?;
					let right = evaluate_node(right_child)?;
					if left.is_unitless() {
						Ok(Number::with_unit(left.value * right.value, right.unit))
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
	use super::*;
	use crate::{eval, units::Unit::*};

	fn eval_default<'a>(input: &'a str) -> Number {
		let result = eval(input, true, false).unwrap();
		result
	}
	fn eval_num<'a>(input: &'a str) -> String {
		let result = eval(input, true, false).unwrap();
		assert!(result.is_unitless());

		result.to_string()
	}

	#[test]
	fn test_evaluations() {
		assert_eq!(eval_num("-2(-3)"), "6");
		assert_eq!(eval_num("-2(3)"), "-6");
		assert_eq!(eval_num("(3)-2"), "1");
		assert_eq!(
			eval_default("-1km to m"),
			Number::with_basic_unit(d!(-1000), Meter)
		);
		assert_eq!(eval_num("2*-3*0.5"), "-3");
		assert_eq!(eval_num("-3^2"), "-9");
		assert_eq!(eval_num("e^2"), "≈ 7.3890560989306502272304274605750078132");
		assert_eq!(
			eval_num("e^2.5"),
			"≈ 12.1824939607034734380701759511679661832"
		);
		assert_eq!(eval_num("-1+2"), "1");
	}

	#[test]
	fn test_functions() {
		assert_eq!(eval_num("cbrt(125)"), "5");
		assert_eq!(
			eval_num("cbrt(2)"),
			"≈ 1.25992104989487316476721060727822835057"
		);

		assert_eq!(eval_num("sqrt(25)"), "5");
		assert_eq!(
			eval_num("sqrt(2)"),
			"≈ 1.41421356237309504880168872420969807857"
		);

		assert_eq!(eval_num("log(100)"), "2");
		assert_eq!(
			eval_num("log(2)"),
			"≈ 0.301029995663981195213738894724493026768"
		);

		assert_eq!(eval_num("ln(1)"), "0");
		assert_eq!(
			eval_num("ln(2)"),
			"≈ 0.69314718055994530941723212145817656808"
		);
		assert_eq!(eval_num("ln(e)"), "≈ 1");
		assert_eq!(eval_num("ln(e^2)"), "≈ 2");

		assert_eq!(
			eval_num("exp(1)"),
			"≈ 2.71828182845904523536028747135266249776"
		);

		assert_eq!(eval_num("round(1.4)"), "1");
		assert_eq!(eval_num("round(1.6)"), "2");
		assert_eq!(eval_num("round(1.5)"), "2");
		assert_eq!(eval_num("round(2.5)"), "3");

		assert_eq!(eval_num("ceil(1.5)"), "2");
		assert_eq!(eval_num("ceil(-1.5)"), "-1");

		assert_eq!(eval_num("floor(1.5)"), "1");
		assert_eq!(eval_num("floor(-1.5)"), "-2");

		assert_eq!(eval_num("abs(-3)"), "3");

		assert_eq!(
			eval_num("sin(2)"),
			"≈ 0.9092974268256816953960198659117448427"
		);
		assert_eq!(
			eval_num("sin(-2)"),
			"≈ -0.9092974268256816953960198659117448427"
		);

		assert_eq!(
			eval_num("cos(2)"),
			"≈ -0.41614683654714238699756822950076218977"
		);
		assert_eq!(
			eval_num("cos(-2)"),
			"≈ -0.41614683654714238699756822950076218977"
		);
		assert_eq!(
			eval_num("tan(2)"),
			"≈ -2.18503986326151899164330610231368254343"
		);
	}
}
