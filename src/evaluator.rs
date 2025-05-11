use crate::lookup::lookup_named_number;
use crate::parser::AstNode;
use crate::units::{add, convert, divide, inaccurate_modulo, modulo, multiply, pow, subtract, Unit, UnitType};
use crate::Constant::{Pi, E};
use crate::{r, FunctionIdentifier::*};
use crate::Operator::{Caret, Divide, Minus, Modulo, Multiply, Plus};
use crate::TextOperator::{Of, To};
use crate::UnaryOperator::{Factorial, Percent};
use crate::{Number, Token};
use malachite::rational::Rational;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::Factorial as MalachiteFactorial;
use malachite::base::num::arithmetic::traits::Pow;

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
pub fn factorial(input: u64) -> Natural {
	Natural::factorial(input)
}

/// Returns the square root of a [`struct@d128`]
pub fn sqrt(input: Rational) -> Rational {
	let mut n = r("1");
	let half = r("0.5");
	for _ in 0..10 {
		n = (&n + &input / &n) * &half;
	}
	n
}

/// Returns the cube root of a [`struct@d128`]
pub fn cbrt(input: Rational) -> Rational {
	let mut n: Rational = input.clone();
	// hope that 20 iterations makes it accurate enough
	let three = r("3");
	for _ in 0..20 {
		let z2 = &n * &n;
		n = &n - ((&n * &z2 - &input) / (&three * z2));
	}
	n
}

/// Returns the sine of a [`struct@d128`]
pub fn sin(mut input: Rational) -> Rational {
	let pi = r("3.141592653589793238462643383279503");
	let pi2 = r("6.283185307179586476925286766559006");

	// input %= pi2;
	input = inaccurate_modulo(input, pi2);

	let negative_correction = if input < 0 {
		input -= pi;
		Rational::from(-1)
	} else {
		Rational::from(1)
	};

	let one = 1u64;
	let two = 2;
	let neg_one = Rational::from(-1);

	let precision = 37;
	let mut result = Rational::from(0);
	for i_int in 0..precision {
		let i = i_int;
		let calc_result = two * i + one;
		result += neg_one.clone().pow(i) * (input.clone().pow(calc_result) / Rational::from(factorial(calc_result)));
	}

	negative_correction * result
}

/// Returns the cosine of a [`struct@d128`]
pub fn cos(input: Rational) -> Rational {
	let half_pi = r("1.570796326794896619231321691639751");
	sin(half_pi - input)
}

/// Returns the tangent of a [`struct@d128`]
pub fn tan(input: Rational) -> Rational {
	sin(input.clone()) / cos(input)
}

/// Evaluate an [`AstNode`] into a [`Number`]
fn evaluate_node(ast_node: &AstNode) -> Result<Number, String> {
	let token = &ast_node.token;
	let children = &ast_node.children;
	match token {
		Token::Number(number) => Ok(Number::new(number.clone(), Unit::NoUnit)),
		Token::Constant(constant) => match constant {
			Pi => Ok(Number::new(
				r("3.141592653589793238462643383279503"),
				Unit::NoUnit,
			)),
			E => Ok(Number::new(
				r("2.718281828459045235360287471352662"),
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
						let result = child_answer.value.ln();
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
					let mut result = child_answer.value.quantize(1);
					let rounding_change = result - child_answer.value;
					// If the result was rounded down by 0.5, correct by +1
					if rounding_change == -0.5 {
						result += 1;
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Ceil => {
					let mut result = child_answer.value.quantize(1);
					let rounding_change = result - child_answer.value;
					if rounding_change.is_negative() {
						result += 1;
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Floor => {
					let mut result = child_answer.value.quantize(1);
					let rounding_change = result - child_answer.value;
					if !rounding_change.is_negative() {
						result -= 1;
					}
					Ok(Number::new(result, child_answer.unit))
				}
				Abs => {
					let mut result = child_answer.value.abs();
					let rounding_change = result - child_answer.value;
					if rounding_change == -0.5 {
						result += 1;
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
					child_answer.value / Rational::from(100),
					child_answer.unit,
				)),
				Factorial => {
					match u64::try_from(&child_answer.value) {
						Ok(value) => {
							let result = factorial(value);
							Ok(Number::new(Rational::from(result), child_answer.unit))
						}
						Err(_) => Err("Can only perform factorial on integers from 0 to 1000".to_string()),
					}
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
