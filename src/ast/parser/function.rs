use std::collections::HashMap;

use crate::{ast::tree::*, error::CompileError, lexer::*, print_errln, print_msg};
use super::Parser;

impl<'a> Parser<'a>
{
	pub fn parse_function_decl(&mut self) -> (String, Function)
	{
		let first_token_pos = self.current_token.span.start;

		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, first_token_pos, self.lexer, "While parsing function.");
		});

		// TODO: Parse attributes (global)

		let identifier: String;
		if let TokenKind::Ident(ident) = token_ident.kind
		{
			identifier = ident;
		} else
		{
			print_errln!(CompileError::Syntax, token_ident.span.start, self.lexer, "Expected function identifier after {KEYWORD_FUNC_DECL}");
		}

		let token_left_paren = self.advance_token().unwrap_or_else(|| { 
			print_errln!(CompileError::UnexpectedEof, token_ident.span.end, self.lexer, "While parsing function."); 
		});
		if token_left_paren.kind != TokenKind::LeftParen
		{
			print_errln!(CompileError::Syntax, token_left_paren.span.start, self.lexer, "Expected argument list after function identifier.");
		}

		let args = self.parse_function_decl_parameters();

		print_msg!("Arguments: {:?}\nCurrent: {:?}", args, self.current_token);
		return (identifier, Function::new(Vec::new(), Vec::new()));

	}

	pub fn parse_function_decl_parameters(&mut self) -> HashMap<String, Variable>
	{
		let mut args: HashMap<String, Variable> = HashMap::new();

		while let Some(token) = self.advance_token()
		{
			match token.kind {
				TokenKind::RightParen => break,
				TokenKind::Ident(identifier) =>
				{
					let token_arg_type = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, token.span.end, self.lexer, "While parsing function parameters.");
					});

					if !Self::is_type(&token_arg_type.kind)
					{
						print_errln!(CompileError::Syntax, token_arg_type.span.start, self.lexer, "Expected parameter type after identifier.");
					}

					args.insert(identifier, Variable::new(
						Self::kind_2_type(&token_arg_type.kind)
					));

					let token_comma = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, token_arg_type.span.end, self.lexer, "While parsing function parameters.");
					});

					match token_comma.kind {
						TokenKind::Comma => continue,
						TokenKind::RightParen => break,
						_ => { print_errln!(CompileError::Syntax, token.span.start, self.lexer, "Unexpected token while parsing function parameters."); }
					}
				
				},
				_ => { print_errln!(CompileError::Syntax, token.span.start, self.lexer, "Unexpected token while parsing function parameters."); }
			}
		}
		return args;	

	}


}