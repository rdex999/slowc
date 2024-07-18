use std::collections::HashMap;
use crate::{ast::*, error::CompileError, lexer::*, print_errln};
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

		let mut variables= self.parse_function_decl_parameters();

		let args_end_pos = self.current_token.span.end;
		let token_ret_type_specifier = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, args_end_pos, self.lexer, "While parsing function return type specifier (Arrow operator \"->\")");
		});
		if token_ret_type_specifier.kind != TokenKind::Arrow
		{
			print_errln!(CompileError::Syntax, token_ret_type_specifier.span.start, self.lexer, "Expected return type specifier after parameter list in function declaration.");
		}

		let token_ret_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, token_ret_type_specifier.span.end, self.lexer, "While parsing function return type.");
		});
		if !Self::is_type(&token_ret_type.kind)
		{
			print_errln!(CompileError::Syntax, token_ret_type.span.start, self.lexer, "Expected function return type after return type specifier.");
		}

		let return_type = Self::kind_2_type(&token_ret_type.kind);

		// TODO: Parse scope, right here self.current_token is a left curly bracket
		let token_scope_start = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, token_ret_type.span.end, self.lexer, "While parsing function declaration.");
		});
		if token_scope_start.kind != TokenKind::LeftCurly
		{
			print_errln!(CompileError::Syntax, token_scope_start.span.start, self.lexer, "Expected scope begin operator \"{{\" after function return type.");
		}

		self.advance_token();

		let mut statements: Vec<Statement> = Vec::new();	
		while let Some(stmt) = self.parse_statement(&mut variables)
		{
			statements.push(stmt);
			break;											/* JUST FOR NOW */
		}

		let locals: Vec<Variable> = variables.values().cloned().collect();
		return (identifier, Function::new(statements, locals, return_type));

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

					// NOTE: (to my future self getting a headache) because arguments will be pushed on the stack from right to left,
					// The stack location (this variable will exist in the future) will just be positive
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