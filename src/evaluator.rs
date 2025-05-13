use crate::lookup::lookup_named_number;
use crate::parser::AstNode;
use crate::units::{add, convert, divide, modulo, multiply, pow, subtract, Unit, UnitType};
use crate::Constant::{Pi, E};
use crate::{f256_to_rational, r, rational_to_f256, FunctionIdentifier::*};
use crate::Operator::{Caret, Divide, Minus, Modulo, Multiply, Plus};
use crate::TextOperator::{Of, To};
use crate::UnaryOperator::{Factorial, Percent};
use crate::{Number, Token};
use malachite::base::num::basic::traits::{One, Zero};
use malachite::rational::Rational;
use malachite::Natural;
use malachite::base::num::arithmetic::traits::{Abs, Ceiling, Factorial as MalachiteFactorial, Floor};
use malachite::base::num::arithmetic::traits::Pow;

/// Evaluate an [`AstNode`] into a [`Number`]
pub fn evaluate(ast: &AstNode) -> Result<Number, String> {
	let answer = evaluate_node(ast)?;
	Ok(answer)
}

pub fn calc_modulo(left: Rational, right: Rational) -> Rational {
	let left_abs = left.clone().abs();
	let right_abs = right.clone().abs();
	let div_result = &left_abs / &right_abs;
	let result = left_abs - Rational::from(div_result.floor()) * right_abs;
	if right < 0 {
		return -result;
	} else {
		return result;
	}
}

/// Returns the square root
pub fn sqrt(input: Rational) -> Rational {
	let mut n = r("1");
	let half = r("0.5");
	for _ in 0..10 {
		n = (&n + &input / &n) * &half;
	}
	n
}

/// Returns the cube root
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

/// Returns the sine
pub fn sin(mut input: Rational) -> Rational {
	let pi2 = r("6.283185307179586476925286766559006");
	let negative = input < 0;

	// input %= pi2;
	input = calc_modulo(input, pi2);


	let mut result = Rational::from(0);
	let neg_one = Rational::from(-1);
	let one = 1u64;
	let two = 2;

	let precision = 37;
	for i in 0..precision {
		let exponent = two * i + one;
		let term = (&neg_one).pow(i) *
			((&input).pow(exponent) /
			Rational::from(Natural::factorial(exponent)));
		result += term;
	}

	if negative {
		-result
	} else {
		result
	}
}

/// Returns the cosine of a number
pub fn cos(input: Rational) -> Rational {
	let half_pi = r("1.570796326794896619231321691639751");
	sin(half_pi - input)
}

/// Returns the tangent
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
						Err("cbrt() only accepts UnitType::NoType".to_string())
					}
				}
				Sqrt => {
					if child_answer.value < Rational::ZERO {
						return Err("Not a number".to_string())
					}
					if child_answer.unit.category() == UnitType::NoType {
						let result = sqrt(child_answer.value);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("sqrt() only accepts UnitType::NoType".to_string())
					}
				}
				Log => {
					if child_answer.value < Rational::ZERO {
						return Err("Not a number".to_string())
					}
					if child_answer.unit.category() == UnitType::NoType {
						let value_f256 = rational_to_f256(child_answer.value, 64);
						let result_f256 = value_f256.log10();
						let result = f256_to_rational(result_f256);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("log() only accepts UnitType::NoType".to_string())
					}
				}
				Ln => {
					if child_answer.value < Rational::ZERO {
						return Err("Not a number".to_string())
					}
					if child_answer.unit.category() == UnitType::NoType {
						let value_f256 = rational_to_f256(child_answer.value, 64);
						let result_f256 = value_f256.ln();
						let result = f256_to_rational(result_f256);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("ln() only accepts UnitType::NoType".to_string())
					}
				}
				Exp => {
					if child_answer.unit.category() == UnitType::NoType {
						let value_f256 = rational_to_f256(child_answer.value, 64);
						let result_f256 = value_f256.exp();
						let result = f256_to_rational(result_f256);
						Ok(Number::new(result, child_answer.unit))
					} else {
						Err("exp() only accepts UnitType::NoType".to_string())
					}
				}
				Round => {
					let floor = Rational::from((&child_answer.value).floor());
					let floor_offset = &child_answer.value - &floor;
					let result = if floor_offset == 0.5 && child_answer.value < 0 {
						floor
					} else if floor_offset >= 0.5 {
						Rational::from(child_answer.value.ceiling())
					} else {
						floor
					};
					Ok(Number::new(result, child_answer.unit))
				}
				Ceil => {
					let result = Rational::from(child_answer.value.ceiling());
					Ok(Number::new(result, child_answer.unit))
				}
				Floor => {
					let result = Rational::from(child_answer.value.floor());
					Ok(Number::new(result, child_answer.unit))
				}
				Abs => {
					let mut result = (&child_answer.value).abs();
					let rounding_change = &result - child_answer.value;
					if rounding_change == -0.5 {
						result += Rational::ONE;
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
							let result = Natural::factorial(value);
							Ok(Number::new(Rational::from(result), child_answer.unit))
						}
						Err(_) => Err("Can only perform factorial on integers".to_string()),
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

#[cfg(test)]
mod tests {
	use malachite::base::num::conversion::string::options::ToSciOptions;
	use malachite::base::num::conversion::traits::ToSci;
	use crate::eval;
	use super::*;

	fn eval_default<'a>(input: &'a str) -> Number {
		let result = eval(input, true, false).unwrap();
		result
	}
	fn eval_num<'a>(input: &'a str) -> String {
		let result = eval(input, true, false).unwrap();
		assert_eq!(result.unit, Unit::NoUnit);

		let mut sci_options = ToSciOptions::default();
		sci_options.set_precision(32);
		let value_str = result.value.to_sci_with_options(sci_options).to_string();
		value_str
	}

	#[test]
	fn test_evaluations() {
		assert_eq!(eval_num("-2(-3)"), "6");
		assert_eq!(eval_num("-2(3)"), "-6");
		assert_eq!(eval_num("(3)-2"), "1");
		assert_eq!(eval_default("-1km to m"), Number::new(r("-1000"), Unit::Meter));
		assert_eq!(eval_num("2*-3*0.5"), "-3");
		assert_eq!(eval_num("-3^2"), "-9");
		assert_eq!(eval_num("-1+2"), "1");
	}

	#[test]
	fn test_functions() {
		assert_eq!(eval_num("abs(-3)"), "3");

		assert_eq!(eval_num("round(1.4)"), "1");
		assert_eq!(eval_num("round(1.6)"), "2");
		assert_eq!(eval_num("round(1.5)"), "2");
		assert_eq!(eval_num("round(2.5)"), "3");

		assert_eq!(eval_num("ceil(1.5)"), "2");
		assert_eq!(eval_num("ceil(-1.5)"), "-1");

		assert_eq!(eval_num("floor(1.5)"), "1");
		assert_eq!(eval_num("floor(-1.5)"), "-2");

		assert_eq!(eval_num("log(100)"), "2");
		assert_eq!(eval_num("log(2)"), "0.30102999566398119521373889472449");

		assert_eq!(eval_num("ln(1)"), "0");
		assert_eq!(eval_num("ln(2)"), "0.69314718055994530941723212145818");
		assert_eq!(eval_num("ln(e)"), "1");
		assert_eq!(eval_num("ln(e^2)"), "2");

		assert_eq!(eval_num("exp(2)"), "7.389056098930650227230427460575");
	}
}
