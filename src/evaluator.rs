use decimal::d128;
use crate::Token;
use crate::units::Unit;
use crate::parser::AstNode;
#[allow(unused_imports)]
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};

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

fn factorial(n: d128) -> d128 {
    if n < d128!(2) {
        d128!(1)
    } else {
        n * factorial(n - d128!(1))
    }
}

fn evaluate_node(ast_node: &AstNode) -> Result<Answer, String> {
  let token = &ast_node.token;
  let children = &ast_node.children;
  match token {
    Token::Number(number) => {
      Ok(Answer::new(number.clone(), Unit::NoUnit))
    },
    Token::Unit(unit) => {
      let child_node = children.get(0).expect("Unit has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      Ok(Answer::new(child_answer.value, unit.clone()))
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
            Ok(Answer::new(factorial(child_answer.value), child_answer.unit))
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
            return Ok(Answer::new(left.value ^ right.value, left.unit))
          } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
            // 1 km ^ 3
            return Ok(Answer::new(left.value ^ right.value, left.unit))
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
      Err(format!("Unexpected ast node {:?}", token))
    },
  }
}
