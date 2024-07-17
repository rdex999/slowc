use crate::{error::CompileError, print_errln};

use super::{super::lexer::*, tree::*};
use std::collections::HashMap;


pub struct Parser<'a>
{
	ir: Root,
	tokens_itr: Lexer<'a>,
	current_token: Token
}

impl<'a> Parser<'a>
{
	pub fn new(mut lexer: Lexer<'a>) -> Self
	{
		let first = lexer.next();
		return Self{
			ir: Root::new(HashMap::new()),
			tokens_itr: lexer,
			current_token: first.unwrap()
		};
	}

	pub fn generate_ir(&mut self) -> &Root
	{
		match self.current_token.kind
		{
			TokenKind::FuncDecl =>
			{
				let (identifier, func) = self.parse_function();
				self.ir.functions.insert(identifier, func);
			}

			_ => 
			{
				println!("Dev error line {}", line!()); 
				std::process::exit(-1);
			},
		}

		return &self.ir;
	}

	fn advance_token(&mut self) -> Option<&Token>
	{
		if let Some(next_token) = self.tokens_itr.next()
		{
			self.current_token = next_token;
			return Some(&self.current_token);
		}
		return None;
	}

	fn parse_function(&mut self) -> (String, Function)
	{
		let first_token_pos = self.current_token.span.start;

		if let Some(token_func_ident) = self.advance_token()
		{
			if let TokenKind::Ident(identifier) = &token_func_ident.kind
			{
				// if let Some(token_func_type) = self.advance_token()
				// {
				// 	if Self::is_type(token_func_type.kind)
				// 	{
				// 	}
				// }
				
				let stmts: Vec<Statement> = Vec::new();

				return (identifier.to_string(), Function::new(stmts));
			} else
			{
				let start = token_func_ident.span.start;
				print_errln!(CompileError::Syntax, start, self.tokens_itr, "Expected function identifier after {KEYWORD_FUNC_DECL}.");
			}
		} else 
		{
			// print_err!(CompileError::UnexpectedEof, "Started function declaration at the end of the file.");
			print_errln!(CompileError::Syntax, first_token_pos, self.tokens_itr, "Unexpected EOF after function declaration.");
		}
	}

	// checks for i32, ...
	fn is_type(token_kind: TokenKind) -> bool
	{
		return token_kind == TokenKind::I32;
	}
}