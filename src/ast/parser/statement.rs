use std::collections::HashMap;
use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::Parser;

impl<'a> Parser<'a>
{
	pub fn parse_statement(&mut self, mut variables: &mut HashMap<String, Variable>) -> Option<Statement>
	{
		match self.current_token.kind {
			TokenKind::VarDecl => return self.parse_var_decl(&mut variables),

			_ => { print_errln!(CompileError::Syntax, self.current_token.span.start, self.lexer, "Unexpected token found at statement beginning."); }
		}
	}

	pub fn parse_var_decl(&mut self, variables: &mut HashMap<String, Variable>) -> Option<Statement>
	{
		let stmt_pos = self.current_token.span;
		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, stmt_pos.end, self.lexer, "While parsing variable declaration. Expected identifier.");
		});

		let identifier: String;
		if let TokenKind::Ident(ident) = token_ident.kind
		{
			identifier = ident;
		} else
		{
			print_errln!(CompileError::Syntax, token_ident.span.start, self.lexer, "Expected identifier after {KEYWORD_VAR_DECL}.");
		}

		let token_data_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, token_ident.span.end, self.lexer, "While parsing variable declaration. Expected data type.");
		});
		if !Self::is_type(&token_data_type.kind)
		{
			print_errln!(CompileError::Syntax, token_data_type.span.start, self.lexer, "Expected data type after variable identifier.");
		}
		let data_type = Self::kind_2_type(&token_data_type.kind);

		variables.insert(identifier, Variable::new(data_type.clone()));		/* Dont kill me for using clone(), its a pure enum */

		let token_assign_or_semi = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, token_data_type.span.end, self.lexer, "While parsing variable declaration. Expected semicolon or assign operator (=).");
		});

		if token_assign_or_semi.kind == TokenKind::Semicolon
		{
			return None;
		} else if token_assign_or_semi.kind != TokenKind::Equal
		{
			print_errln!(CompileError::Syntax, token_assign_or_semi.span.start, self.lexer, "Expected assign operator (=) or semicolon.");
		}

		// Will get here is there is an initial assignment to the variable
		self.advance_token(); 	/* Skip equal token, now self.current_token is the first token of the expression */

		let expr = self.parse_expression(data_type, variables);
		return Some(Statement::Assign(expr));
	}
}