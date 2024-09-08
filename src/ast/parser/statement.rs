use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

impl<'a> Parser<'a>
{
	pub fn parse_statement(&mut self, variables: &mut LocalVariables, function: &Function) -> Option<Statement>
	{
		match self.current_token().kind {
			TokenKind::LeftCurly 					=> return Some(Statement::Scope(self.parse_scope(variables, function))),
			TokenKind::VarDecl 						=> return self.parse_var_decl(variables),
			TokenKind::If							=> return Some(self.parse_if_stmt(variables, function)),
			TokenKind::For							=> return Some(self.parse_for_stmt(variables, function)),
			TokenKind::Return 						=> return Some(self.parse_return_stmt(variables, function)),
			TokenKind::Semicolon 					=> { self.advance_token(); return None; },
			TokenKind::Asterisk 	=> return Some(self.parse_var_update(variables)),
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

		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ident.span.end, "While parsing variable declaration. Expected data type.");
		});
		
		let data_type = self.parse_data_type().unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_ident.span.end, "Expected data type after variable identifier.");
		});

		if data_type == Type::new(TypeKind::Void)
		{
			print_errln!(
				CompileError::TypeError(Type::new(TypeKind::I32), Type::new(TypeKind::Void)), 
				self.source, 
				self.current_token().span.start,
				"Cannot declare variable of type \"{KEYWORD_VOID}\", it makes no sense."
			);
		}

		// FIXME: Might be able to use the variable inside its declaration expression.
		let new_var = variables.add_variable(identifier, 0, data_type);

		let token_assign_or_semi = self.current_token();

		self.advance_token().unwrap_or_else(|| {
			print_errln!(
				CompileError::UnexpectedEof, 
				self.source, 
				self.current_token().span.start, 
				"While parsing variable declaration. Expected semicolon or assign operator ( = )."
			);
		});

		if token_assign_or_semi.kind == TokenKind::Semicolon
		{
			return None;
		} else if token_assign_or_semi.kind != TokenKind::Equal
		{
			print_errln!(CompileError::Syntax, self.source, token_assign_or_semi.span.start, "Expected assign operator ( = ) or semicolon ( ; ).");
		}

		// Will get here is there is an initial assignment to the variable
		let expr = self.parse_expression(Some(data_type), variables);

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
				let rvalue = self.parse_expression(Some(self.value_type(&destination, &variables)), variables);
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

		if function.return_type != Type::new(TypeKind::Void)
		{
			let expr = self.parse_expression(Some(function.return_type), variables);
			stmt = Statement::Return(Some(expr));
		}

		if self.current_token().kind != TokenKind::Semicolon
		{
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected semicolon.");
		}
		self.advance_token();

		return stmt;
	}

	fn parse_if_stmt(&mut self, variables: &mut LocalVariables, function: &Function) -> Statement
	{
		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing {KEYWORD_IF} statement.");
		});

		let expression = self.parse_expression(None, variables);

		let then_statement = self.parse_statement(variables, function).unwrap_or_else(|| {
			print_errln!(
				CompileError::Syntax, 
				self.source, 
				self.current_token().span.start, 
				"The \"then\" block of an \"{KEYWORD_IF}\" statement cannot be a variable declaration without a value."
			);
		});

		let mut else_statement = None;

		if self.current_token().kind == TokenKind::Else
		{
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing {KEYWORD_ELSE} statement.");
			});

			else_statement = Some(self.parse_statement(variables, function).unwrap_or_else(|| {
				print_errln!(
					CompileError::Syntax, 
					self.source, 
					self.current_token().span.start, 
					"The \"{KEYWORD_ELSE}\" of an \"{KEYWORD_IF}\" statement cannot be a variable declaration without a value."
				);
			}));
		}

		return Statement::If(IfInfo::new(expression, then_statement, else_statement));
	}

	fn parse_for_stmt(&mut self, variables: &mut LocalVariables, function: &Function) -> Statement
	{
		// For loop syntax: 	for let i i32 = 0; i < 420; i += 1;
		// 							<CODE>

		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing {KEYWORD_FOR} statement.");
		});

		// Start a scope before parsing initializer for statement (because it is most likely to be a variable declaration)
		variables.start_scope();

		let initializer = self.parse_statement(variables, function);
		let condition;
		let update;
		let code_block;

		// Condition parsing
		if self.current_token().kind == TokenKind::Semicolon 
		{
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing {KEYWORD_FOR} loop condition.");
			});
			condition = None;
		} else
		{
			condition = Some(self.parse_expression(None, variables));
			if self.current_token().kind != TokenKind::Semicolon
			{
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected semicolon after {KEYWORD_FOR} loop condition.");
			}
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing {KEYWORD_FOR} loop condition.");
			});
		}
		
		update = self.parse_statement(variables, function);

		code_block = self.parse_statement(variables, function).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "{KEYWORD_FOR} loop code block must be a valid statement.");
		});

		let stack_size = variables.end_scope();
		let for_stmt = Statement::For(ForLoopInfo::new(initializer, condition, update, code_block, stack_size));
		return for_stmt;
	}
}