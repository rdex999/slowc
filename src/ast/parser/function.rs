use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

impl<'a> Parser<'a>
{
	pub fn parse_function_decl(&mut self) -> (String, Function)
	{
		let first_token_pos = self.current_token().span.start;

		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, first_token_pos, "While parsing function.");
		});

		// TODO: Parse attributes (global)

		if TokenKind::Ident != token_ident.kind
		{
			print_errln!(CompileError::Syntax, self.source, token_ident.span.start, "Expected function identifier after {KEYWORD_FUNC_DECL}");
		}
		let identifier = self.get_text(&token_ident.span).to_string();

		let token_left_paren = self.advance_token().unwrap_or_else(|| { 
			print_errln!(CompileError::UnexpectedEof, self.source, token_ident.span.end, "While parsing function."); 
		});
		if token_left_paren.kind != TokenKind::LeftParen
		{
			print_errln!(CompileError::Syntax, self.source, token_left_paren.span.start, "Expected argument list after function identifier.");
		}

		self.advance_token();
		let mut variables = self.parse_function_decl_parameters();

		let args_end_pos = self.current_token().span.end;
		let token_ret_type_specifier = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, args_end_pos, "While parsing function return type specifier (Arrow operator \"->\")");
		});

		if token_ret_type_specifier.kind != TokenKind::Arrow
		{
			print_errln!(CompileError::Syntax, self.source, token_ret_type_specifier.span.start, "Expected return type specifier after parameter list in function declaration.");
		}

		let token_ret_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ret_type_specifier.span.end, "While parsing function return type.");
		});

		let return_type = Self::kind_2_type(&token_ret_type.kind).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_ret_type.span.start, "Expected function return type after return type specifier.");
		});

		// TODO: Parse scope, right here self.current_token is a left curly bracket
		let token_scope_start = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ret_type.span.end, "While parsing function declaration.");
		});
		if token_scope_start.kind != TokenKind::LeftCurly
		{
			print_errln!(CompileError::Syntax, self.source, token_scope_start.span.start, "Expected scope begin operator \"{{\" after function return type.");
		}

		self.advance_token();
		

		let mut statements: Vec<Statement> = Vec::new();	
		while let Some(stmt) = self.parse_statement(&mut variables)
		{
			statements.push(stmt);
			break;											/* JUST FOR NOW */
		}

		self.advance_token();
		self.advance_token();
		self.advance_token();
		self.advance_token();
		self.advance_token();
		return (identifier, Function::new(statements, return_type));

	}

	pub fn parse_function_decl_parameters(&mut self) -> LocalVariables
	{
		let mut args = LocalVariables::new();
		loop 
		{
			let token = self.current_token();
			let token_span = token.span;
			match token.kind 
			{
				TokenKind::RightParen => break,
				TokenKind::Ident =>
				{
					let ident = self.get_text(&token_span).to_string();
					let token_arg_type = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, self.source, token_span.end, "While parsing function parameters.");
					});

					let data_type = Self::kind_2_type(&token_arg_type.kind).unwrap_or_else(|| {
						print_errln!(CompileError::Syntax, self.source, token_arg_type.span.start, "Expected parameter type after identifier.");
					});

					// NOTE: (to my future self getting a headache) because arguments will be pushed on the stack from right to left,
					// The stack location (this variable will exist in the future) will just be positive
					args.add_variable(ident, data_type);
					
					let token_comma = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, self.source, token_arg_type.span.end, "While parsing function parameters.");
					});

					match token_comma.kind {
						TokenKind::Comma => continue,
						TokenKind::RightParen => break,
						_ => { print_errln!(CompileError::Syntax, self.source, token_span.start, "Unexpected token while parsing function parameters."); }
					}
				
				},
				_ => { print_errln!(CompileError::Syntax, self.source, token_span.start, "Unexpected token while parsing function parameters."); }
			}
		}
		return args;	

	}
}