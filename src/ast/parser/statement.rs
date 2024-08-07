use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

impl<'a> Parser<'a>
{
	pub fn parse_statement(&mut self, variables: &mut LocalVariables, function: &Function) -> Option<Statement>
	{
		match self.current_token().kind {
			TokenKind::LeftCurly 	=> return Some(Statement::Scope(self.parse_scope(variables, function))),
			TokenKind::VarDecl 		=> return self.parse_var_decl(variables),
			TokenKind::Return 		=> return Some(self.parse_return_stmt(variables, function)),
			TokenKind::Ident 		=> 
			{
				if let Some(next_token) = self.peek(1)
				{
					if next_token.kind == TokenKind::LeftParen
					{
						let stmt = Some(Statement::FunctionCall(self.parse_function_call(variables)));
						if self.current_token().kind != TokenKind::Semicolon
						{
							print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected semicolon.");
						}	
						self.advance_token();
						return stmt;
					}
				}
				return Some(self.parse_var_update(variables));
			},

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
		if let Some(_) = variables.get_variable(&identifier[..])
		{
			print_errln!(CompileError::Syntax, self.source, token_ident.span.start, "Variable \"{identifier}\" was already declared.");
		}

		let token_data_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ident.span.end, "While parsing variable declaration. Expected data type.");
		});
		
		let data_type = Type::from_token_kind(&token_data_type.kind).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_data_type.span.start, "Expected data type after variable identifier.");
		});

		if data_type == Type::Void
		{
			print_errln!(
				CompileError::TypeError(Type::I32, Type::Void), 
				self.source, 
				token_data_type.span.start, 
				"Cannot declare variable of type \"{KEYWORD_VOID}\", it makes no sense."
			);
		}

		let new_var = variables.add_variable(identifier, 0, data_type.clone());		/* Dont kill me for using clone(), its a pure enum */

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
			Value::Var(new_var.index),
			expr
		)));
	}


	fn parse_var_update(&mut self, variables: &mut LocalVariables) -> Statement
	{
		let destination = self.parse_value(None, variables, true).unwrap();

		match self.current_token().kind
		{
			TokenKind::Equal =>
			{
				self.advance_token();
				let rvalue = self.parse_expression(self.value_type(&destination, &variables), variables);
				if self.current_token().kind != TokenKind::Semicolon
				{
					print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected semicolon.");
				}
				self.advance_token();
				return Statement::Assign(VarUpdateInfo::new(destination, rvalue));
			}

			_ => { print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected assignment operator, such as =, +=, ..."); }
		}
	}

	fn parse_return_stmt(&mut self, variables: &mut LocalVariables, function: &Function) -> Statement
	{
		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing {KEYWORD_RETURN} statement.");
		});

		let mut stmt = Statement::Return(None);

		if function.return_type != Type::Void
		{
			let expr = self.parse_expression(function.return_type, variables);
			stmt = Statement::Return(Some(expr));
		}

		if self.current_token().kind != TokenKind::Semicolon
		{
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected semicolon.");
		}
		self.advance_token();

		return stmt;
	}
}