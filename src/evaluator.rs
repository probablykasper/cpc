use decimal::d128;
use crate::Token;
use crate::units::{Unit, UnitType, Number, convert, add, subtract};
use crate::parser::AstNode;
use crate::Operator::{Caret, Divide, Minus, Modulo, Multiply, Plus};
use crate::Constant::{Pi, E};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{To, Of};
use crate::FunctionIdentifier::*;
use crate::lookup::lookup_factorial;

pub fn evaluate(ast: &AstNode) -> Result<Number, String> {
  let answer = evaluate_node(ast)?;
  Ok(answer)
}

fn factorial(input: d128) -> d128 {
  return lookup_factorial(input.into());
}

fn sqrt(input: d128) -> d128 {
  let mut n = d128!(1);
  let half = d128!(0.5);
  for _ in 0..10 {
    n = (n + input/n) * half;
  }
  return n
}

fn cbrt(input: d128) -> d128 {
  let mut n: d128 = input;
  // hope that 20 iterations makes it accurate enough
  let three = d128!(3);
  for _ in 0..20 {
    let z2 = n*n;
    n = n - ((n*z2 - input) / (three*z2));
  }
  return n
}

fn sin(mut input: d128) -> d128 {
  let pi = d128!(3.141592653589793238462643383279503);
  let pi2 = d128!(6.283185307179586476925286766559006);

  input %= pi2;

  let negative_correction = if input.is_negative() {
    input -= pi;
    d128!(-1)
  } else {
    d128!(1)
  };

  let one = d128!(1);
  let two = d128!(2);
  let neg_one = -one;

  let precision = 37;
  let mut result = d128!(0);
  for i_int in 0..precision {
    let i = d128::from(i_int);
    let calc_result = two*i+one;
    result += neg_one.pow(i) * (input.pow(calc_result) / factorial(calc_result));
  }

  return negative_correction * result;
  
}

fn cos(input: d128) -> d128 {
  let half_pi = d128!(1.570796326794896619231321691639751);
  return sin(half_pi - input);
}

fn tan(input: d128) -> d128 {
  return sin(input) / cos(input);
}

fn evaluate_node(ast_node: &AstNode) -> Result<Number, String> {
  let token = &ast_node.token;
  let children = &ast_node.children;
  match token {
    Token::Number(number) => {
      Ok(Number::new(number.clone(), Unit::NoUnit))
    },
    Token::Constant(constant) => {
      match constant {
        Pi => {
          Ok(Number::new(d128!(3.141592653589793238462643383279503), Unit::NoUnit))
        },
        E => {
          Ok(Number::new(d128!(2.718281828459045235360287471352662), Unit::NoUnit))
        },
      }
    },
    Token::FunctionIdentifier(function) => {
      let child_node = children.get(0).expect("Paren has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      match function {
        Cbrt => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = cbrt(child_answer.value);
            return Ok(Number::new(result, child_answer.unit))
          } else {
            return Err(format!("log() only accepts UnitType::NoUnit").to_string())
          }
        },
        Sqrt => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = sqrt(child_answer.value);
            return Ok(Number::new(result, child_answer.unit))
          } else {
            return Err(format!("log() only accepts UnitType::NoUnit").to_string())
          }
        },
        Log => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = child_answer.value.log10();
            return Ok(Number::new(result, child_answer.unit))
          } else {
            return Err(format!("log() only accepts UnitType::NoUnit").to_string())
          }
        },
        Ln => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = child_answer.value.ln();
            return Ok(Number::new(result, child_answer.unit))
          } else {
            return Err(format!("ln() only accepts UnitType::NoUnit").to_string())
          }
        },
        Exp => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = child_answer.value.exp(child_answer.value);
            return Ok(Number::new(result, child_answer.unit))
          } else {
            return Err(format!("exp() only accepts UnitType::NoUnit").to_string())
          }
        },
        Round => {
          // .quantize() rounds .5 to nearest even integer, so we correct that
          let mut result = child_answer.value.quantize(d128!(1));
          let rounding_change = result - child_answer.value;
          // If the result was rounded down by 0.5, correct by +1
          if rounding_change == d128!(-0.5) { result += d128!(1); }
          return Ok(Number::new(result, child_answer.unit))
        },
        Ceil => {
          let mut result = child_answer.value.quantize(d128!(1));
          let rounding_change = result - child_answer.value;
          if rounding_change.is_negative() { result += d128!(1); }
          return Ok(Number::new(result, child_answer.unit))
        },
        Floor => {
          let mut result = child_answer.value.quantize(d128!(1));
          let rounding_change = result - child_answer.value;
          if !rounding_change.is_negative() { result -= d128!(1); }
          return Ok(Number::new(result, child_answer.unit))
        },
        Abs => {
          let mut result = child_answer.value.abs();
          let rounding_change = result - child_answer.value;
          if rounding_change == d128!(-0.5) { result += d128!(1); }
          return Ok(Number::new(result, child_answer.unit))
        },
        Sin => {
          let result = sin(child_answer.value);
          return Ok(Number::new(result, child_answer.unit))
        },
        Cos => {
          let result = cos(child_answer.value);
          return Ok(Number::new(result, child_answer.unit))
        },
        Tan => {
          let result = tan(child_answer.value);
          return Ok(Number::new(result, child_answer.unit))
        },
      }
    }
    Token::Unit(unit) => {
      let child_node = children.get(0).expect("Unit has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      Ok(Number::new(child_answer.value, unit.clone()))
    },
    Token::Negative => {
      let child_node = children.get(0).expect("Negative has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      Ok(Number::new(-child_answer.value, child_answer.unit))
    },
    Token::Paren => {
      let child_node = children.get(0).expect("Paren has no child[0]");
      return evaluate_node(child_node)
    },
    Token::UnaryOperator(operator) => {
      let child_node = children.get(0).expect(format!("Token {:?} has no child[0]", token).as_str());
      let child_answer = evaluate_node(child_node)?;
      match operator {
        Percent => {
          Ok(Number::new(child_answer.value / d128!(100), child_answer.unit))
        },
        Factorial => {
          let result = factorial(child_answer.value);
          if result.is_nan() {
            return Err("Can only perform factorial on integers from 0 to 1000".to_string());
          }
          Ok(Number::new(result, child_answer.unit))
        },
      }
    },
    Token::TextOperator(operator) => {
      let left_child = children.get(0).expect(format!("Token {:?} has no child[0]", token).as_str());
      let right_child = children.get(1).expect(format!("Token {:?} has no child[1]", token).as_str());
      
      match operator {
        To => {
          if let Token::Unit(right_unit) = right_child.token {
            let left = evaluate_node(left_child)?;
            let result = convert(left, right_unit)?;
            return Ok(result)
          } else {
            return Err("Right side of To operator needs to be a unit".to_string())
          }
        },
        Of => {
          let left = evaluate_node(left_child)?;
          let right = evaluate_node(right_child)?;
          if left.unit == Unit::NoUnit {
            return Ok(Number::new(left.value * right.value, right.unit))
          } else {
            return Err("Left side of the Of operator must be NoUnit".to_string())
          }
        },
      }
    },
    Token::Operator(operator) => {
      let left_child = children.get(0).expect(format!("Token {:?} has no child[0]", token).as_str());
      let right_child = children.get(1).expect(format!("Token {:?} has no child[1]", token).as_str());
      let left = evaluate_node(left_child)?;
      let right = evaluate_node(right_child)?;
      match operator {
        Plus => {
          Ok(add(left, right)?)
        },
        Minus => {
          Ok(subtract(left, right)?)
        },
        Multiply => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 * 2
            return Ok(Number::new(left.value * right.value, left.unit))
          } else if left.unit == Unit::NoUnit && right.unit != Unit::NoUnit {
            // 3 * 1 km
            return Ok(Number::new(left.value * right.value, right.unit))
          } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
            // 1 km * 3
            return Ok(Number::new(left.value * right.value, left.unit))
          } else {
            return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
          }
        },
        Divide => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 / 2
            return Ok(Number::new(left.value / right.value, left.unit))
          } else if left.unit != Unit::NoUnit && right.unit == Unit::NoUnit {
            // 1 km / 2
            return Ok(Number::new(left.value / right.value, right.unit))
          } else {
            return Err(format!("Cannot divide {:?} by {:?}", left.unit, right.unit))
          }
        },
        Modulo => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 / 2
            return Ok(Number::new(left.value % right.value, left.unit))
          } else if left.unit != Unit::NoUnit && right.unit == Unit::NoUnit {
            // 1 km / 2
            return Ok(Number::new(left.value % right.value, right.unit))
          } else {
            return Err(format!("Cannot modulo {:?} by {:?}", left.unit, right.unit))
          }
        },
        Caret => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 ^ 2
            return Ok(Number::new(left.value.pow(right.value), left.unit))
          } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
            // 1 km ^ 3
            return Ok(Number::new(left.value.pow(right.value), left.unit))
          } else {
            return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
          }
        },
        _ => {
          Err(format!("Unexpected operator {:?}", operator))
        }
      }
    },
    _ => {
      Err(format!("Unexpected token {:?}", token))
    }
  }
}
