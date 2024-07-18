mod function;

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
					let (identifier, func) = self.parse_function_decl();
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

	// checks for i32, ...
	fn is_type(token_kind: &TokenKind) -> bool
	{
		return *token_kind == TokenKind::I32;
	}

	fn kind_2_type(token_kind: &TokenKind) -> Type
	{
		match token_kind {
			TokenKind::I32 => return Type::I32,
			_ => panic!("Dev error!! parser, kind_2_type() called with token of kind {:?}", token_kind)
		};
	}

}