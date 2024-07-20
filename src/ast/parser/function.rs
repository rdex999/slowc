use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

impl<'a> Parser<'a>
{
	pub fn parse_function_decl(&mut self) -> (String, Function)
	{
		let first_token_pos = self.current_token().span.start;

		let src = self.source;
		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, first_token_pos, "While parsing function.");
		});
		let ident_span = token_ident.span;

		// TODO: Parse attributes (global)

		let identifier;
		if let TokenKind::Ident(ident) = token_ident.kind
		{
			identifier = ident.to_string();
		} else
		{
			print_errln!(CompileError::Syntax, src, ident_span.start, "Expected function identifier after {KEYWORD_FUNC_DECL}");
		}

		let token_left_paren = self.advance_token().unwrap_or_else(|| { 
			print_errln!(CompileError::UnexpectedEof, src, ident_span.end, "While parsing function."); 
		});
		if token_left_paren.kind != TokenKind::LeftParen
		{
			print_errln!(CompileError::Syntax, src, token_left_paren.span.start, "Expected argument list after function identifier.");
		}

		self.advance_token();
		let mut variables= self.parse_function_decl_parameters();

		let args_end_pos = self.current_token().span.end;
		let ret_type_specifier_span;
		let token_ret_type_specifier = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, args_end_pos, "While parsing function return type specifier (Arrow operator \"->\")");
		});
		ret_type_specifier_span = token_ret_type_specifier.span;

		if token_ret_type_specifier.kind != TokenKind::Arrow
		{
			print_errln!(CompileError::Syntax, src, ret_type_specifier_span.start, "Expected return type specifier after parameter list in function declaration.");
		}

		let ret_type_span;
		let token_ret_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, ret_type_specifier_span.end, "While parsing function return type.");
		});
		ret_type_span = token_ret_type.span;
		if !Self::is_type(&token_ret_type.kind)
		{
			print_errln!(CompileError::Syntax, src, ret_type_span.start, "Expected function return type after return type specifier.");
		}

		let return_type = Self::kind_2_type(&token_ret_type.kind);

		// TODO: Parse scope, right here self.current_token is a left curly bracket
		let token_scope_start = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, ret_type_span.end, "While parsing function declaration.");
		});
		if token_scope_start.kind != TokenKind::LeftCurly
		{
			print_errln!(CompileError::Syntax, src, token_scope_start.span.start, "Expected scope begin operator \"{{\" after function return type.");
		}

		self.advance_token();
		

		let mut statements: Vec<Statement> = Vec::new();	
		while let Some(stmt) = self.parse_statement(&mut variables)
		{
			statements.push(stmt);
			break;											/* JUST FOR NOW */
		}

		let locals = variables.into_var_array();
		self.advance_token();
		self.advance_token();
		self.advance_token();
		self.advance_token();
		self.advance_token();
		return (identifier, Function::new(statements, locals, return_type));

	}

	pub fn parse_function_decl_parameters(&mut self) -> LocalVariables
	{
		let src = self.source;
		let mut args = LocalVariables::new();
		loop 
		{
			let token = self.current_token();
			let token_span = token.span;
			match token.kind 
			{
				TokenKind::RightParen => break,
				TokenKind::Ident(identifier) =>
				{
					let ident = identifier.to_string();
					let arg_type_span;
					let token_arg_type = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, src, token_span.end, "While parsing function parameters.");
					});
					arg_type_span = token_arg_type.span;

					if !Self::is_type(&token_arg_type.kind)
					{
						print_errln!(CompileError::Syntax, src, arg_type_span.start, "Expected parameter type after identifier.");
					}

					// NOTE: (to my future self getting a headache) because arguments will be pushed on the stack from right to left,
					// The stack location (this variable will exist in the future) will just be positive
					args.add_variable(ident, Self::kind_2_type(&token_arg_type.kind));
					
					let token_comma = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, src, arg_type_span.end, "While parsing function parameters.");
					});

					match token_comma.kind {
						TokenKind::Comma => continue,
						TokenKind::RightParen => break,
						_ => { print_errln!(CompileError::Syntax, src, token_span.start, "Unexpected token while parsing function parameters."); }
					}
				
				},
				_ => { print_errln!(CompileError::Syntax, src, token_span.start, "Unexpected token while parsing function parameters."); }
			}
		}
		return args;	

	}
}