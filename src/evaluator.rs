use decimal::d128;
use crate::Token;
use crate::units::{Unit, UnitType};
use crate::parser::AstNode;
use crate::Operator::{Caret, Divide, Minus, Modulo, Multiply, Plus};
use crate::Constant::{Pi, E};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{To, Of};
use crate::FunctionIdentifier::*;

#[derive(Clone, Debug)]
pub struct Answer {
  value: d128,
  unit: Unit,
}

impl Answer {
  pub fn new(value: d128, unit: Unit) -> Answer {
    Answer {
      value: value,
      unit: unit,
    }
  }
}

pub fn evaluate(ast: &AstNode) -> Result<Answer, String> {
  let answer = evaluate_node(ast)?;
  Ok(answer)
}

fn factorial(n: d128, one: d128, two: d128) -> d128 {
    if n < two {
        one
    } else {
        n * factorial(n - one, one, two)
    }
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

fn evaluate_node(ast_node: &AstNode) -> Result<Answer, String> {
  let token = &ast_node.token;
  let children = &ast_node.children;
  match token {
    Token::Number(number) => {
      Ok(Answer::new(number.clone(), Unit::NoUnit))
    },
    Token::Constant(constant) => {
      match constant {
        Pi => {
          Ok(Answer::new(d128!(3.141592653589793238462643383279503), Unit::NoUnit))
        },
        E => {
          Ok(Answer::new(d128!(2.718281828459045235360287471352662), Unit::NoUnit))
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
            return Ok(Answer::new(result, child_answer.unit))
          } else {
            return Err(format!("log() only accepts UnitType::NoUnit").to_string())
          }
        },
        Sqrt => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = sqrt(child_answer.value);
            return Ok(Answer::new(result, child_answer.unit))
          } else {
            return Err(format!("log() only accepts UnitType::NoUnit").to_string())
          }
        },
        Log => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = child_answer.value.log10();
            return Ok(Answer::new(result, child_answer.unit))
          } else {
            return Err(format!("log() only accepts UnitType::NoUnit").to_string())
          }
        },
        Ln => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = child_answer.value.ln();
            return Ok(Answer::new(result, child_answer.unit))
          } else {
            return Err(format!("ln() only accepts UnitType::NoUnit").to_string())
          }
        },
        Exp => {
          if child_answer.unit.category() == UnitType::NoUnit {
            let result = child_answer.value.exp(child_answer.value);
            return Ok(Answer::new(result, child_answer.unit))
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
          return Ok(Answer::new(result, child_answer.unit))
        },
        Ceil => {
          let mut result = child_answer.value.quantize(d128!(1));
          let rounding_change = result - child_answer.value;
          if rounding_change.is_negative() { result += d128!(1); }
          return Ok(Answer::new(result, child_answer.unit))
        },
        Floor => {
          let mut result = child_answer.value.quantize(d128!(1));
          let rounding_change = result - child_answer.value;
          if !rounding_change.is_negative() { result -= d128!(1); }
          return Ok(Answer::new(result, child_answer.unit))
        },
        Abs => {
          let mut result = child_answer.value.abs();
          let rounding_change = result - child_answer.value;
          if rounding_change == d128!(-0.5) { result += d128!(1); }
          return Ok(Answer::new(result, child_answer.unit))
        },

        // Sin,
        // Cos,
        // Tan,
        // Asin,
        // Acos,
        // Atan,
        // Sinh,
        // Cosh,
        // Tanh,
        // Asinh,
        // Acosh,
        // Atanh,
        // Sin
        // Sinh
        // Tan
        // Tanh
        _ => {
          return Err(format!("EVAL UNSUPPORTED FUNC").to_string())
        },
      }
    }
    Token::Unit(unit) => {
      let child_node = children.get(0).expect("Unit has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      Ok(Answer::new(child_answer.value, unit.clone()))
    },
    Token::Negative => {
      let child_node = children.get(0).expect("Negative has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      Ok(Answer::new(-child_answer.value, child_answer.unit))
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
          Ok(Answer::new(child_answer.value / d128!(100), child_answer.unit))
        },
        Factorial => {
          if child_answer.value.is_negative() || child_answer.value.is_signed() {
            Err("Cannot perform factorial of floats or negative".to_string())
          } else if child_answer.value > d128!(1000) {
            Err("Cannot perform factorial of numbers above 1000".to_string())
          } else {
            Ok(Answer::new(factorial(child_answer.value, d128!(1), d128!(2)), child_answer.unit))
          }
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
            if left.unit.category() == right_unit.category() {
              let left_weight = left.unit.weight();
              let right_weight = right_unit.weight();
              let result = left.value * left_weight / right_weight;
              return Ok(Answer::new(result, right_unit))
            } else {
              return Err(format!("Cannot convert from {:?} to {:?}", left.unit, right_unit))
            }
          } else {
            return Err("Right side of To operator needs to be a unit".to_string())
          }
        },
        Of => {
          let left = evaluate_node(left_child)?;
          let right = evaluate_node(right_child)?;
          if left.unit == Unit::NoUnit {
            return Ok(Answer::new(left.value * right.value, right.unit))
          } else {
            return Err("child[0] of the Of operator must be NoUnit".to_string())
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
          if left.unit == right.unit {
            Ok(Answer::new(left.value + right.value, left.unit))
          } else if left.unit.category() == right.unit.category() {
            let result = left.value * left.unit.weight() + right.value * right.unit.weight();
            Ok(Answer::new(result, Unit::Millimeter))
          } else {
            return Err(format!("Cannot add {:?} and {:?}", left.unit, right.unit))
          }
        },
        Minus => {
          if left.unit == right.unit {
            Ok(Answer::new(left.value - right.value, left.unit))
          } else if left.unit.category() == right.unit.category() {
            let result = left.value * left.unit.weight() - right.value * right.unit.weight();
            Ok(Answer::new(result, Unit::Millimeter))
          } else {
            return Err(format!("Cannot subtract {:?} by {:?}", left.unit, right.unit))
          }
        },
        Multiply => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 * 2
            return Ok(Answer::new(left.value * right.value, left.unit))
          } else if left.unit == Unit::NoUnit && right.unit != Unit::NoUnit {
            // 3 * 1 km
            return Ok(Answer::new(left.value * right.value, right.unit))
          } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
            // 1 km * 3
            return Ok(Answer::new(left.value * right.value, left.unit))
          } else {
            return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
          }
        },
        Divide => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 / 2
            return Ok(Answer::new(left.value / right.value, left.unit))
          } else if left.unit != Unit::NoUnit && right.unit == Unit::NoUnit {
            // 1 km / 2
            return Ok(Answer::new(left.value / right.value, right.unit))
          } else {
            return Err(format!("Cannot divide {:?} by {:?}", left.unit, right.unit))
          }
        },
        Modulo => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 / 2
            return Ok(Answer::new(left.value % right.value, left.unit))
          } else if left.unit != Unit::NoUnit && right.unit == Unit::NoUnit {
            // 1 km / 2
            return Ok(Answer::new(left.value % right.value, right.unit))
          } else {
            return Err(format!("Cannot modulo {:?} by {:?}", left.unit, right.unit))
          }
        },
        Caret => {
          if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
            // 3 ^ 2
            return Ok(Answer::new(left.value.pow(right.value), left.unit))
          } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
            // 1 km ^ 3
            return Ok(Answer::new(left.value.pow(right.value), left.unit))
          } else {
            return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
          }
        },
        _ => {
          Err(format!("Unexpected operator {:?}", operator))
        }
      }
    },
  }
}
