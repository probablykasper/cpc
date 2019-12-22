use decimal::d128;
use crate::Token;
use crate::units::Unit;
use crate::parser::AstNode;
#[allow(unused_imports)]
use crate::Operator::{Percent, Caret, Divide, Factorial, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};

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

fn evaluate_node(ast_node: &AstNode) -> Result<Answer, String> {
  let token = &ast_node.token;
  let children = &ast_node.children;
  match token {
    Token::Number(number) => {
      Ok(Answer::new(number.clone(), Unit::NoUnit))
    },
    Token::Unit(unit) => {
      let child_node = ast_node.children.get(0).expect("Unit has no child[0]");
      let child_answer = evaluate_node(child_node)?;
      Ok(Answer::new(child_answer.value, unit.clone()))
    },
    Token::Paren => {
      let child_node = ast_node.children.get(0).expect("Paren has no child[0]");
      return evaluate_node(child_node)
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
            return Err(format!("Cannot subtract {:?} and {:?}", left.unit, right.unit))
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
