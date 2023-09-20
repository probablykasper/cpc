use crate::units::Unit::{Foot, Inch};
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::TextOperator::{Of, To};
use crate::Token;
use crate::UnaryOperator::{Factorial, Percent};

#[derive(Debug)]
/// A struct with a [`Token`](AstNode::token) and [`AstNode`] [`children`](AstNode::children)
pub struct AstNode {
	/// The children of the [`AstNode`]
	pub children: Vec<AstNode>,
	/// The token of the [`AstNode`]
	pub token: Token,
}

impl AstNode {
	pub const fn new(token: Token) -> AstNode {
		AstNode {
			children: Vec::new(),
			token,
		}
	}
}

/// Parse [`Token`]s into an Abstract Syntax Tree ([`AstNode`])
pub fn parse(tokens: &[Token]) -> Result<AstNode, String> {
	parse_text_operators(tokens, 0).and_then(|(ast, next_pos)| {
		if next_pos == tokens.len() {
			Ok(ast)
		} else {
			Err(format!(
				"Expected end of input, found {:?} at {}",
				tokens[next_pos], next_pos
			))
		}
	})
}

// level 1 precedence (lowest): to, of
/// Parse [`To`](crate::TextOperator::To) and [`Of`](crate::TextOperator::Of)
pub fn parse_text_operators(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	// do higher precedences first, then come back down
	let (mut node, mut pos) = parse_plus(tokens, pos)?;
	// now we loop through the next tokens
	loop {
		let token = tokens.get(pos);
		match token {
			// if there's a match, we once again do higher precedences, then come
			// back down again and continue the loop
			Some(&Token::TextOperator(To)) | Some(&Token::TextOperator(Of)) => {
				let (right_node, next_pos) = parse_plus(tokens, pos + 1)?;
				let mut new_node = AstNode::new(token.unwrap().clone());
				new_node.children.push(node);
				new_node.children.push(right_node);
				node = new_node;
				pos = next_pos;
			}
			// if there's no match, we go down to a lower precedence
			_ => {
				return Ok((node, pos));
			}
		}
	}
}

/// Parse [`+`](crate::Operator::Plus), [`-`](crate::Operator::Minus)
pub fn parse_plus(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	let (mut node, mut pos) = parse_unary(tokens, pos)?;
	loop {
		let token = tokens.get(pos);
		match token {
			Some(&Token::Operator(Plus)) | Some(&Token::Operator(Minus)) => {
				let (right_node, next_pos) = parse_unary(tokens, pos + 1)?;
				let mut new_node = AstNode::new(token.unwrap().clone());
				new_node.children.push(node);
				new_node.children.push(right_node);
				node = new_node;
				pos = next_pos;
			}
			_ => {
				return Ok((node, pos));
			}
		}
	}
}

/// Parse [`unary -`](Token::Negative) (for example -5)
pub fn parse_unary(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	// Since a unary operator has no left side, we parse the the unary operator immediately
	let token = tokens.get(pos);
	match token {
		Some(&Token::Operator(Minus)) => {
			let (right_node, next_pos) = parse_mult_level(tokens, pos + 1)?;
			let mut new_node = AstNode::new(Token::Negative);
			new_node.children.push(right_node);
			Ok((new_node, next_pos))
		}
		_ => parse_mult_level(tokens, pos),
	}
}

/// Parse [`*`](crate::Operator::Multiply), [`/`](crate::Operator::Divide), [`Modulo`](crate::Operator::Modulo), implicative multiplication (for example`2pi`), foot-inch syntax (for example `6'4"`)
pub fn parse_mult_level(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
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
								children: vec![AstNode::new(token0.unwrap().clone())],
								token: Token::Unit(Foot),
							},
							AstNode {
								children: vec![AstNode::new(token2.unwrap().clone())],
								token: Token::Unit(Inch),
							},
						],
						token: Token::Operator(Plus),
					};
					return Ok((new_node, pos + 4));
				}
			}
		}
	}

	let (mut node, mut pos) = parse_caret(tokens, pos)?;

	loop {
		let token = tokens.get(pos);
		match token {
			Some(&Token::Operator(Multiply))
			| Some(&Token::Operator(Divide))
			| Some(&Token::Operator(Modulo)) => {
				let (right_node, next_pos) = parse_caret(tokens, pos + 1)?;
				let mut new_node = AstNode::new(token.unwrap().clone());
				new_node.children.push(node);
				new_node.children.push(right_node);
				node = new_node;
				pos = next_pos;
			}

			// Below is implicative multiplication, for example '2pi'. Constants and
			// such will only end up here if they were unable to be parsed as part of
			// other operators.
			// Note that this match statement matches an AstNode token, but the
			// matches nested inside check the [`Token`]s. That's why we for example
			// match a FunctionIdentifier, and inside that, a RightParen.

			// pi2, )2
			Some(&Token::Number(_)) => {
				let last_token = tokens.get(pos - 1);
				match last_token {
					Some(&Token::Constant(_)) | Some(&Token::Operator(RightParen)) => {
						let (right_node, next_pos) = parse_caret(tokens, pos)?;
						let mut new_node = AstNode::new(Token::Operator(Multiply));
						new_node.children.push(node);
						new_node.children.push(right_node);
						node = new_node;
						pos = next_pos;
					}
					_ => {
						return Ok((node, pos));
					}
				}
			}
			// 2pi, )pi
			Some(&Token::Constant(_)) => {
				let last_token = tokens.get(pos - 1);
				match last_token {
					Some(&Token::Number(_)) | Some(&Token::Operator(RightParen)) => {
						let (right_node, next_pos) = parse_caret(tokens, pos)?;
						let mut new_node = AstNode::new(Token::Operator(Multiply));
						new_node.children.push(node);
						new_node.children.push(right_node);
						node = new_node;
						pos = next_pos;
					}
					_ => {
						return Ok((node, pos));
					}
				}
			}
			// 2log(1), )log(1)
			Some(&Token::FunctionIdentifier(_)) => {
				let last_token = tokens.get(pos - 1);
				match last_token {
					Some(&Token::Number(_)) | Some(&Token::Operator(RightParen)) => {
						let (right_node, next_pos) = parse_caret(tokens, pos)?;
						let mut new_node = AstNode::new(Token::Operator(Multiply));
						new_node.children.push(node);
						new_node.children.push(right_node);
						node = new_node;
						pos = next_pos;
					}
					_ => {
						return Ok((node, pos));
					}
				}
			}
			// 2(3), pi(3), )(3)
			Some(&Token::Operator(LeftParen)) => {
				let last_token = tokens.get(pos - 1);
				match last_token {
					Some(&Token::Number(_))
					| Some(&Token::Constant(_))
					| Some(&Token::Operator(RightParen)) => {
						let (right_node, next_pos) = parse_caret(tokens, pos)?;
						let mut new_node = AstNode::new(Token::Operator(Multiply));
						new_node.children.push(node);
						new_node.children.push(right_node);
						node = new_node;
						pos = next_pos;
					}
					_ => {
						return Ok((node, pos));
					}
				}
			}
			_ => {
				return Ok((node, pos));
			}
		}
	}
}

/// Parse [`^`](crate::Operator::Caret)
pub fn parse_caret(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	let (mut node, mut pos) = parse_unary_high(tokens, pos)?;
	loop {
		let token = tokens.get(pos);
		match token {
			Some(&Token::Operator(Caret)) => {
				let (right_node, next_pos) = parse_unary_high(tokens, pos + 1)?;
				let mut new_node = AstNode::new(token.unwrap().clone());
				new_node.children.push(node);
				new_node.children.push(right_node);
				node = new_node;
				pos = next_pos;
			}
			_ => {
				return Ok((node, pos));
			}
		}
	}
}

/// Parse [`unary -`](Token::Negative) at high precedence (for example in 3^-2)
pub fn parse_unary_high(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	let token = tokens.get(pos);
	match token {
		Some(&Token::Operator(Minus)) => {
			let (right_node, next_pos) = parse_suffix(tokens, pos + 1)?;
			let mut new_node = AstNode::new(Token::Negative);
			new_node.children.push(right_node);
			Ok((new_node, next_pos))
		}
		_ => parse_suffix(tokens, pos),
	}
}

/// Parse [`!!`](crate::UnaryOperator::Factorial), [`Percent`](crate::UnaryOperator::Percent), units attached to values
pub fn parse_suffix(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	let (mut node, mut pos) = parse_highest(tokens, pos)?;
	loop {
		let token = tokens.get(pos);
		match token {
			Some(&Token::UnaryOperator(Factorial))
			| Some(&Token::UnaryOperator(Percent))
			| Some(&Token::NamedNumber(_)) => {
				// Here we are handling unary operators, aka stuff written as
				// "Number Operator" (3!) instead of "Number Operator Number" (3+3).
				// Therefore, if we find a match, we don't parse what comes after it.
				let mut new_node = AstNode::new(token.unwrap().clone());
				new_node.children.push(node);
				node = new_node;
				pos += 1;
			}
			Some(&Token::Unit(_unit)) => {
				// We won't allow units to repeat, like "1min min", so we end the loop if it's found.
				let mut new_node = AstNode::new(token.unwrap().clone());
				new_node.children.push(node);
				return Ok((new_node, pos + 1));
			}
			_ => {
				// let's say we parse 1+2. parse_level_7 then returns 1, and token
				// is set to plus. Plus has lower precedence than level 4, so we
				// don't do anything, and pass the number down to a lower precedence.
				return Ok((node, pos));
			}
		}
	}
}

/// Parse [`Number`](Token::Number), standalone [`Unit`](Token::Unit), [`Constant`](Token::Constant), [`FunctionIdentifier`](Token::FunctionIdentifier), [`Paren`](Token::Paren)
pub fn parse_highest(tokens: &[Token], pos: usize) -> Result<(AstNode, usize), String> {
	let token: &Token = tokens
		.get(pos)
		.ok_or(format!("Unexpected end of input at {}", pos))?;
	match token {
		&Token::Number(_number) => {
			let node = AstNode::new(token.clone());
			Ok((node, pos + 1))
		}
		&Token::Unit(_unit) => {
			let node = AstNode::new(token.clone());
			Ok((node, pos + 1))
		}
		Token::Constant(_constant) => {
			let node = AstNode::new(token.clone());
			Ok((node, pos + 1))
		}
		Token::FunctionIdentifier(_function_identifier) => {
			let left_paren_pos = pos + 1;
			let left_paren_token = tokens.get(left_paren_pos);
			// check if '(' comes after function identifier, like 'log('
			match left_paren_token {
				Some(&Token::Operator(LeftParen)) => {
					// parse everything inside as you would with normal parentheses,
					// then put it inside an ast node.
					parse_text_operators(tokens, left_paren_pos + 1).and_then(|(node, next_pos)| {
						if let Some(&Token::Operator(RightParen)) = tokens.get(next_pos) {
							let mut function_node = AstNode::new(token.clone());
							function_node.children.push(node);
							Ok((function_node, next_pos + 1))
						} else {
							Err(format!(
								"Expected closing paren at {} but found {:?}",
								next_pos,
								tokens.get(next_pos)
							))
						}
					})
				}
				_ => Err(format!(
					"Expected ( after {} at {:?} but found {:?}",
					left_paren_pos, token, left_paren_token
				)),
			}
		}
		Token::Operator(LeftParen) => {
			parse_text_operators(tokens, pos + 1).and_then(|(node, next_pos)| {
				if let Some(&Token::Operator(RightParen)) = tokens.get(next_pos) {
					let mut paren_node = AstNode::new(Token::Paren);
					paren_node.children.push(node);
					Ok((paren_node, next_pos + 1))
				} else {
					Err(format!(
						"Expected closing paren at {} but found {:?}",
						next_pos,
						tokens.get(next_pos)
					))
				}
			})
		}
		_ => Err(format!(
			"Unexpected token {:?}, expected paren or number",
			token
		)),
	}
}
