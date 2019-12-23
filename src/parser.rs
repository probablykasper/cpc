use crate::{Token, TokenVector};
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{To, Of};

#[derive(Debug)]
pub struct AstNode {
  pub children: Vec<AstNode>,
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

pub fn parse(tokens: &TokenVector) -> Result<AstNode, String> {
  parse_level_1(tokens, 0).and_then(|(ast, next_pos)| if next_pos == tokens.len() {
      Ok(ast)
  } else {
      Err(format!("Expected end of input, found {:?} at {}", tokens[next_pos], next_pos))
  })
}

// level 1 precedence (lowest): to, of
fn parse_level_1(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
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
fn parse_level_2(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
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

// level 3 precedence: *, /, modulo
fn parse_level_3(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
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
      _ => {
        return Ok((node, pos));
      },
    }
  }
}

// level 4 precedence: ^
fn parse_level_4(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
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

// level 5 precedence: !, percent
fn parse_level_5(tokens: &TokenVector, pos: usize) -> Result<(AstNode, usize), String> {
  let (mut node, mut pos) = parse_level_6(tokens, pos, None)?;
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

// level 6 precedence: numbers, parens
fn parse_level_6(tokens: &TokenVector, pos: usize, last_token: Option<Token>) -> Result<(AstNode, usize), String> {
  let token: &Token = tokens.get(pos).expect(&format!("Unexpected end of input at {}", pos));
  match token {
    Token::Operator(Minus) => {
      if let None = last_token {
        let (right_node, next_pos) = parse_level_6(tokens, pos + 1, Some(Token::Operator(Minus)))?;
        let mut new_node = AstNode::new(Token::Negative);
        new_node.children.push(right_node);
        Ok((new_node, next_pos))
      } else {
        // 3-1 might end up here?
        return Err(format!("Unexpected unary operator {0:?} at {1}", token, pos));
      }
    },
    &Token::Number(_number) => {
      let node = AstNode::new(token.clone());
      Ok((node, pos + 1))
    },
    &Token::Unit(_unit) => {
      let node = AstNode::new(token.clone());
      Ok((node, pos + 1))
    }
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
          let mut paren = AstNode::new(Token::Paren);
          paren.children.push(node);
          Ok((paren, next_pos + 1))
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
