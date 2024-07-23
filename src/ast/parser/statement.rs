use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

impl<'a> Parser<'a>
{
	pub fn parse_statement(&mut self, mut variables: &mut LocalVariables) -> Option<Statement>
	{
		match self.current_token().kind {
			TokenKind::VarDecl => return self.parse_var_decl(&mut variables),

			// TokenKind::Ident => Some(self.parse_var_update(&mut variables)),

			_ => { print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Unexpected token found at statement beginning."); }
		}
	}

	fn parse_var_decl(&mut self, variables: &mut LocalVariables) -> Option<Statement>
	{
		let stmt_pos = self.current_token().span;
		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, stmt_pos.end, "While parsing variable declaration. Expected identifier.");
		});

		let identifier = self.get_text(&token_ident.span).to_string();
		if TokenKind::Ident != token_ident.kind
		{
			print_errln!(CompileError::Syntax, self.source, token_ident.span.start, "Expected identifier after {KEYWORD_VAR_DECL}.");
		}

		let token_data_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ident.span.end, "While parsing variable declaration. Expected data type.");
		});
		
		let data_type = Type::from_token_kind(&token_data_type.kind).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_data_type.span.start, "Expected data type after variable identifier.");
		});

		let new_var = variables.add_variable(identifier, data_type.clone());		/* Dont kill me for using clone(), its a pure enum */

		let token_assign_or_semi = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_data_type.span.end, "While parsing variable declaration. Expected semicolon or assign operator (=).");
		});

		if token_assign_or_semi.kind == TokenKind::Semicolon
		{
			return None;
		} else if token_assign_or_semi.kind != TokenKind::Equal
		{
			print_errln!(CompileError::Syntax, self.source, token_assign_or_semi.span.start, "Expected assign operator (=) or semicolon.");
		}

		// Will get here is there is an initial assignment to the variable
		self.advance_token(); 	/* Skip equal token, now self.current_token is the first token of the expression */

		let expr = self.parse_expression(data_type, variables);

		if self.current_token().kind != TokenKind::Semicolon
		{
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected semicolon.");
		}
		self.advance_token();
		return Some(Statement::Assign(VarUpdateInfo::new(
			Lvalue::Var(new_var),
			expr
		)));
	}


	fn parse_var_update(&mut self, variables: &mut LocalVariables) -> ()
	{
		let ident = self.get_text(&self.current_token().span);
	}
}