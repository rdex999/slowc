use crate::{error::CompileError, print_errln};

use super::{super::lexer::*, tree::*};
use std::collections::HashMap;


pub struct Parser<'a>
{
	ir: Root,
	lexer: Lexer<'a>,
	current_token: Token
}

impl<'a> Parser<'a>
{
	pub fn new(lexer: Lexer<'a>) -> Self
	{
		return Self{
			ir: Root::new(HashMap::new()),
			lexer,
			current_token: Token::new( 					/* Dont really need this line, but yk if rust wants it so bad.. */
				TokenKind::None,
				TextSpan::new(0, 0)
			)
		};
	}

	pub fn generate_ir(&mut self) -> &Root
	{
		while let Some(token) = self.advance_token()
		{
			match token.kind
			{
				TokenKind::FuncDecl =>
				{
					let (identifier, func) = self.parse_function();
					self.ir.functions.insert(identifier, func);
				}
				
				_ => 
				{
					// print_errln!(CompileError::Syntax, token.span.start, self.lexer, "Unexpected entity at global scope.");
				},
			}
		}

		return &self.ir;
	}

	fn advance_token(&mut self) -> Option<Token>
	{
		if let Some(next_token) = self.lexer.next()
		{
			self.current_token = next_token;
			return Some(self.current_token.clone());
		}
		return None;
	}

	fn parse_function(&mut self) -> (String, Function)
	{
		let first_token_pos = self.current_token.span.start;

		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, first_token_pos, self.lexer, "Unexpected EOF while parsing function.");
		});

		let identifier: String;
		if let TokenKind::Ident(ident) = token_ident.kind
		{
			identifier = ident;
		} else
		{
			print_errln!(CompileError::Syntax, token_ident.span.start, self.lexer, "Expected function identifier after {KEYWORD_FUNC_DECL}");
		}

		let token_return_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, token_ident.span.end, self.lexer, "Unexpected EOF while parsing function.");
		});

		if !Self::is_type(&token_return_type.kind)
		{
			print_errln!(CompileError::Syntax, token_return_type.span.start, self.lexer, "Expected function return type after identifier.");
		}

		/* TODO: parse arguments, and curly braces */

		return (identifier, Function::new(Vec::new()));

	}

	// checks for i32, ...
	fn is_type(token_kind: &TokenKind) -> bool
	{
		return *token_kind == TokenKind::I32;
	}
}