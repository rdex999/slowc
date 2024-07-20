use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

impl<'a> Parser<'a>
{
	pub fn parse_statement(&mut self, mut variables: &mut LocalVariables) -> Option<Statement>
	{
		match self.current_token().kind {
			TokenKind::VarDecl => return self.parse_var_decl(&mut variables),

			_ => { print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Unexpected token found at statement beginning."); }
		}
	}

	pub fn parse_var_decl(&mut self, variables: &mut LocalVariables) -> Option<Statement>
	{
		let src = self.source;
		let stmt_pos = self.current_token().span;
		let ident_span;
		let token_ident = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, stmt_pos.end, "While parsing variable declaration. Expected identifier.");
		});
		ident_span = token_ident.span;

		let identifier;
		if let TokenKind::Ident(ident) = token_ident.kind
		{
			identifier = ident.to_string();
		} else
		{
			print_errln!(CompileError::Syntax, src, ident_span.start, "Expected identifier after {KEYWORD_VAR_DECL}.");
		}

		let data_type_span;
		let token_data_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, ident_span.end, "While parsing variable declaration. Expected data type.");
		});
		data_type_span = token_data_type.span;
		if !Self::is_type(&token_data_type.kind)
		{
			print_errln!(CompileError::Syntax, src, data_type_span.start, "Expected data type after variable identifier.");
		}
		let data_type = Self::kind_2_type(&token_data_type.kind);

		let new_var = variables.add_variable(identifier, data_type.clone());		/* Dont kill me for using clone(), its a pure enum */

		let token_assign_or_semi = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, src, data_type_span.end, "While parsing variable declaration. Expected semicolon or assign operator (=).");
		});

		if token_assign_or_semi.kind == TokenKind::Semicolon
		{
			return None;
		} else if token_assign_or_semi.kind != TokenKind::Equal
		{
			print_errln!(CompileError::Syntax, src, token_assign_or_semi.span.start, "Expected assign operator (=) or semicolon.");
		}

		// Will get here is there is an initial assignment to the variable
		self.advance_token(); 	/* Skip equal token, now self.current_token is the first token of the expression */

		let expr = self.parse_expression(data_type, variables);
		return Some(Statement::Assign(VarUpdateInfo::new(
			Writable::Var(new_var),
			expr
		)));
	}
}