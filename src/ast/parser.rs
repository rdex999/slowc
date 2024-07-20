mod common;
mod function;
mod statement;
mod expression;
mod variable;

use crate::{error::CompileError, print_errln};

use super::{super::lexer::*, *};
use std::collections::HashMap;

pub struct Parser<'a>
{
	ir: Root,
	tokens: Vec<Token>,
	position: usize,
	source: &'a str
}

impl<'a> Parser<'a>
{
	pub fn new(lexer: Lexer<'a>) -> Self
	{
		let source = lexer.source;
		return Self{
			ir: Root::new(HashMap::new()),
			tokens: lexer.collect(),
			position: 0,
			source
		};
	}

	pub fn generate_ir(mut self) -> Root
	{
		loop
		{
			let token = self.current_token();
			match token.kind
			{
				TokenKind::Eof => break,

				TokenKind::FuncDecl =>
				{
					let (identifier, func) = self.parse_function_decl();
					self.ir.functions.insert(identifier, func);
				}
				
				_ => 
				{
					print_errln!(CompileError::Syntax, self.source, token.span.start, "Unexpected entity at global scope.");
				},

			}
		}

		return self.ir;
	}

	fn advance_token(&mut self) -> Option<Token>
	{
		if self.position >= self.tokens.len() - 1
		{
			return None;
		}
		self.position += 1;
		return Some(self.tokens[self.position]);
	}

	fn current_token(&self) -> Token
	{
		return self.tokens[self.position];
	}
}