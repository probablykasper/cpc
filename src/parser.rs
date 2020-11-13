use crate::{Token, TokenVector};
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{To, Of};
use crate::units::Unit::{Foot, Inch};

#[derive(Debug)]
/// A struct with a [`Token`](struct.AstNode.html#structfield.token) and `AstNode` [`children`](struct.AstNode.html#structfield.children)
pub struct AstNode {
  /// The children of the `AstNode`
  pub children: Vec<AstNode>,
  /// The token of the `AstNode`
  pub token: Token,
}

impl AstNode {
  pub fn new(token: Token) -> AstNode {
    AstNode {
      children: Vec::new(),
      token: token,
    }
  }
}

/// Parse [`TokenVector`](type.TokenVector.html) into an Abstract Syntax Tree ([`AstNode`](struct.AstNode.html))
pub fn parse(tokens: &TokenVector) -> Result<AstNode, String> {
  parse_level_1(tokens, 0).and_then(|(ast, next_pos)| if next_pos == tokens.len() {
      Ok(ast)
  } else {
      Err(format!("Expected end of input, found {:?} at {}", tokens[next_pos], next_pos))
  })
}

// level 1 precedence (lowest): to, of
/// Parse [`to`](../enum.TextOperator.html#variant.To) and [`of`](../enum.TextOperator.html#variant.Of)
pub fn parse_level_1(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  // do higher precedences first, then come back down
  let (mut node, mut pos) = parse_level_2(tokens, pos)?;
  // now we loop through the next tokens
  loop {
    let token = tokens.get(pos);
    match token {
      // if there's a match, we once again do higher precedences, then come
      // back down again and continue the loop
      Some(&Token::TextOperator(To)) | Some(&Token::TextOperator(Of)) => {
        let (right_node, next_pos) = parse_level_2(tokens, pos + 1)?;
        let mut new_node = AstNode::new(token.unwrap().clone());
        new_node.children.push(node);
        new_node.children.push(right_node);
        node = new_node;
        pos = next_pos;
      },
      // if there's no match, we go down to a lower precedence (or, in this
      // case, we're done)
      _ => {
        return Ok((node, pos));
      },
    }
  }
}

// level 2 precedence: +, -
/// Parse [`Plus`](../enum.Operator.html#variant.Plus) and [`Minus`](../enum.Operator.html#variant.Minus)
pub fn parse_level_2(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  let (mut node, mut pos) = parse_level_3(tokens, pos)?;
  loop {
    let token = tokens.get(pos);
    match token {
      Some(&Token::Operator(Plus)) | Some(&Token::Operator(Minus)) => {
        let (right_node, next_pos) = parse_level_3(tokens, pos + 1)?;
        let mut new_node = AstNode::new(token.unwrap().clone());
        new_node.children.push(node);
        new_node.children.push(right_node);
        node = new_node;
        pos = next_pos;
      },
      _ => {
        return Ok((node, pos));
      },
    }
  }
}

// level 3 precedence: *, /, modulo, implicative multiplication, foot-inch 6'4"
/// Parse [`Multiply`](../enum.Operator.html#variant.Multiply), [`Divide`](../enum.Operator.html#variant.Divide), [`Modulo`](../enum.Operator.html#variant.Modulo) and implicative multiplication (for example`2pi`)
pub fn parse_level_3(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {

  // parse foot-inch syntax 6'4"
  let token0 = tokens.get(pos);
  if let Some(Token::Number(_number)) = token0 {
    let token1 = tokens.get(pos + 1);
    if let Some(Token::Unit(Foot)) = token1 {
      let token2 = tokens.get(pos + 2);
      if let Some(Token::Number(_number)) = token2 {
        let token3 = tokens.get(pos + 3);
        if let Some(Token::Unit(Inch)) = token3 {
          let new_node = AstNode {
            children: vec![
              AstNode {
                children: vec![
                  AstNode::new(token0.unwrap().clone()),
                ],
                token: Token::Unit(Foot),
              },
              AstNode {
                children: vec![
                  AstNode::new(token2.unwrap().clone()),
                ],
                token: Token::Unit(Inch),
              },
            ],
            token: Token::Operator(Plus),
          };
          return Ok((new_node, pos + 4))
        }
      }
    }
  }
  
  let (mut node, mut pos) = parse_level_4(tokens, pos)?;

  loop {
    let token = tokens.get(pos);
    match token {
      Some(&Token::Operator(Multiply)) | Some(&Token::Operator(Divide)) | Some(&Token::Operator(Modulo)) => {
        let (right_node, next_pos) = parse_level_4(tokens, pos + 1)?;
        let mut new_node = AstNode::new(token.unwrap().clone());
        new_node.children.push(node);
        new_node.children.push(right_node);
        node = new_node;
        pos = next_pos;
      },

      // Below is implicative multiplication, for example '2pi'. Constants and
      // such will only end up here if they were unable to be parsed as part of
      // other operators.
      // Note that this match statement matches an AstNode token, but the
      // matches nested inside check the TokenVector. That's why we for example
      // match a FunctionIdentifier, and inside that, a RightParen.

      // pi2, )2
      Some(&Token::Number(_)) => {
        let last_token = tokens.get(pos - 1);
        match last_token {
          Some(&Token::Constant(_)) | Some(&Token::Operator(RightParen)) => {
            let (right_node, next_pos) = parse_level_4(tokens, pos)?;
            let mut new_node = AstNode::new(Token::Operator(Multiply));
            new_node.children.push(node);
            new_node.children.push(right_node);
            node = new_node;
            pos = next_pos;
          },
          _ => {
            return Ok((node, pos));
          },
        }
      },
      // 2pi, )pi
      Some(&Token::Constant(_)) => {
        let last_token = tokens.get(pos - 1);
        match last_token {
          Some(&Token::Number(_)) | Some(&Token::Operator(RightParen)) => {
            let (right_node, next_pos) = parse_level_4(tokens, pos)?;
            let mut new_node = AstNode::new(Token::Operator(Multiply));
            new_node.children.push(node);
            new_node.children.push(right_node);
            node = new_node;
            pos = next_pos;
          },
          _ => {
            return Ok((node, pos));
          },
        }
      },
      // 2log(1), )log(1)
      Some(&Token::FunctionIdentifier(_)) => {
        let last_token = tokens.get(pos - 1);
        match last_token {
          Some(&Token::Number(_)) | Some(&Token::Operator(RightParen)) => {
            let (right_node, next_pos) = parse_level_4(tokens, pos)?;
            let mut new_node = AstNode::new(Token::Operator(Multiply));
            new_node.children.push(node);
            new_node.children.push(right_node);
            node = new_node;
            pos = next_pos;
          },
          _ => {
            return Ok((node, pos));
          },
        }
      },
      // 2(3), pi(3), )(3)
      Some(&Token::Operator(LeftParen)) => {
        let last_token = tokens.get(pos - 1);
        match last_token {
          Some(&Token::Number(_)) | Some(&Token::Constant(_)) | Some(&Token::Operator(RightParen)) => {
            let (right_node, next_pos) = parse_level_4(tokens, pos)?;
            let mut new_node = AstNode::new(Token::Operator(Multiply));
            new_node.children.push(node);
            new_node.children.push(right_node);
            node = new_node;
            pos = next_pos;
          },
          _ => {
            return Ok((node, pos));
          },
        }
      },
      _ => {
        return Ok((node, pos));
      },
    }
  }
}

// level 4 precedence: ^
/// Parse [`Caret`](../enum.Operator.html#variant.Caret)
pub fn parse_level_4(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  let (mut node, mut pos) = parse_level_5(tokens, pos)?;
  loop {
    let token = tokens.get(pos);
    match token {
      Some(&Token::Operator(Caret)) => {
        let (right_node, next_pos) = parse_level_5(tokens, pos + 1)?;
        let mut new_node = AstNode::new(token.unwrap().clone());
        new_node.children.push(node);
        new_node.children.push(right_node);
        node = new_node;
        pos = next_pos;
      },
      _ => {
        return Ok((node, pos));
      },
    }
  }
}

// level 5 precedence: - (as in -5, but not 4-5)
/// Parse [`Negative`](../enum.Token.html#variant.Negative)
pub fn parse_level_5(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  // Here we parse the negative unary operator. If the current token
  // is a minus, we wrap the right_node inside a Negative AstNode.
  // 
  // Why doesn't this parse 4-5? First, we will first get a 4. In which case,
  // we just return the result of parse_level_6(), which will include the pos
  // of +. This will then go down to level 2 and be parsed as a normal minus
  // operator.
  // The difference is that in other levels, we parse higher priorities
  // immediately, while in this one we instead check if the current token
  // is a minus, and if not, we then return the higher priority as normal.
  let token = tokens.get(pos);
  match token {
    Some(&Token::Operator(Minus)) => {
      let (right_node, next_pos) = parse_level_6(tokens, pos + 1)?;
      let mut new_node = AstNode::new(Token::Negative);
      new_node.children.push(right_node);
      return Ok((new_node, next_pos));
    },
    _ => {
      return Ok(parse_level_6(tokens, pos)?);
    }
  }
}

// level 6 precedence: !, percent, units attached to values
/// Parse [`Factorial`](../enum.UnaryOperator.html#variant.Factorial) and [`Percent`](../enum.UnaryOperator.html#variant.Percent)
pub fn parse_level_6(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  let (mut node, mut pos) = parse_level_7(tokens, pos)?;
  loop {
    let token = tokens.get(pos);
    match token {
      Some(&Token::UnaryOperator(Factorial)) | Some(&Token::UnaryOperator(Percent)) => {
        // Here we are handling unary operators, aka stuff written as
        // "Number Operator" (3!) instead of "Number Operator Number" (3+3).
        // Therefore, if we find a match, we don't parse what comes after it.
        let mut new_node = AstNode::new(token.unwrap().clone());
        new_node.children.push(node);
        node = new_node;
        pos += 1;
      },
      Some(&Token::Unit(_unit)) => {
        // We won't allow units to repeat, like "1min min", so we end the loop if it's found.
        let mut new_node = AstNode::new(token.unwrap().clone());
        new_node.children.push(node);
        return Ok((new_node, pos + 1));
      },
      _ => {
        // let's say we parse 1+2. parse_level_7 then returns 1, and token
        // is set to plus. Plus has lower precedence than level 4, so we
        // don't do anything, and pass the number down to a lower precedence.
        return Ok((node, pos));
      },
    }
  }
}

// level 7 precedence: numbers, standalone units, constants, functions, parens
/// Parse [`Number`](../enum.Token.html#variant.Number),
/// [`Unit`](../units/enum.Unit.html),
/// [`Constant`](../enum.Constant.html),
/// [`FunctionIdentifier`](../enum.FunctionIdentifier.html),
/// [`Paren`](../enum.Token.html#variant.Paren)
pub fn parse_level_7(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  let token: &Token = tokens.get(pos).ok_or(format!("Unexpected end of input at {}", pos))?;
  match token {
    &Token::Number(_number) => {
      let node = AstNode::new(token.clone());
      Ok((node, pos + 1))
    },
    &Token::Unit(_unit) => {
      let node = AstNode::new(token.clone());
      Ok((node, pos + 1))
    },
    Token::Constant(_constant) => {
      let node = AstNode::new(token.clone());
      Ok((node, pos + 1))
    },
    Token::FunctionIdentifier(_function_identifier) => {
      let left_paren_pos = pos + 1;
      let left_paren_token = tokens.get(left_paren_pos);
      // check if '(' comes after function identifier, like 'log('
      match left_paren_token {
        Some(&Token::Operator(LeftParen)) => {
          // parse everything inside as you would with normal parentheses,
          // then put it inside an ast node.
          parse_level_1(tokens, left_paren_pos + 1).and_then(|(node, next_pos)| {
            if let Some(&Token::Operator(RightParen)) = tokens.get(next_pos) {
              let mut function_node = AstNode::new(token.clone());
              function_node.children.push(node);
              Ok((function_node, next_pos + 1))
            } else {
              Err(format!("Expected closing paren at {} but found {:?}", next_pos, tokens.get(next_pos)))
            }
          })
        },
        _ => {
          return Err(format!("Expected ( after {} at {:?} but found {:?}", left_paren_pos, token, left_paren_token));
        }
      }
    },
    Token::Operator(LeftParen) => {
      parse_level_1(tokens, pos + 1).and_then(|(node, next_pos)| {
        if let Some(&Token::Operator(RightParen)) = tokens.get(next_pos) {
          let mut paren_node = AstNode::new(Token::Paren);
          paren_node.children.push(node);
          Ok((paren_node, next_pos + 1))
        } else {
          Err(format!("Expected closing paren at {} but found {:?}", next_pos, tokens.get(next_pos)))
        }
      })
    },
    _ => {
      Err(format!("Unexpected token {:?}, expected paren or number", token))
    },
  }
}
