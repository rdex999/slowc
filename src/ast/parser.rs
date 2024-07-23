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
	source: &'a str,
	has_passed_eof: bool
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
			source,
			has_passed_eof: false
		};
	}

	pub fn generate_ir(mut self) -> Root
	{
		while !self.has_passed_eof	
		{
			let token = self.current_token();
			match token.kind
			{
				TokenKind::FuncDecl => self.parse_function_decl(),
				
				_ => 
				{
					print_errln!(CompileError::Syntax, self.source, token.span.start, "Unexpected entity at global scope.");
				},

			}
		}

		return self.ir;
	}


}