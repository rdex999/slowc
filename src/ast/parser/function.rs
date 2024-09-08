use crate::{ast::*, error::CompileError, lexer::*, print_errln};
use super::{Parser, variable::*};

pub struct FunctionManager
{
	index: u8,
	functions: HashMap<String, Function>,
}

impl FunctionManager
{
	pub fn new() -> Self
	{
		return Self {
			index: 0,
			functions: HashMap::new(),
		};
	}

	// Returns the index of the new function
	pub fn add(&mut self, mut function: Function) -> u8
	{
		function.index = self.index;
		self.functions.insert(function.identifier.clone(), function);
		self.index += 1;
		return self.index - 1;
	}

	pub fn get(&self, identifier: &str) -> Option<&Function>
	{
		return self.functions.get(identifier);
	}

	pub fn get_by_index(&self, index: u8) -> Option<&Function>
	{
		return self.functions.values().find(|function| function.index == index);
	}

	pub fn into_function_array(self) -> Vec<Function>
	{
		let mut array: Vec<Function> = self.functions.into_values().collect();
		array.sort_by_cached_key(|function| function.index);
		return array;
	}
}

impl<'a> Parser<'a>
{
	pub fn parse_function_decl(&mut self)
	{
		if let None = self.advance_token()
		{
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.end, "While parsing function.");
		}

		let attributes = self.parse_function_decl_attributes();
		
		let token_ident = self.current_token();
		
		if TokenKind::Ident != token_ident.kind
		{
			print_errln!(CompileError::Syntax, self.source, token_ident.span.start, "Expected function identifier after {KEYWORD_FUNC_DECL}");
		}
		let identifier = self.get_text(&token_ident.span);
		
		let token_left_paren = self.advance_token().unwrap_or_else(|| { 
			print_errln!(CompileError::UnexpectedEof, self.source, token_ident.span.end, "While parsing function."); 
		});
		if token_left_paren.kind != TokenKind::LeftParen
		{
			print_errln!(CompileError::Syntax, self.source, token_left_paren.span.start, "Expected argument list after function identifier.");
		}
		
		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing function parameters.");
		});

		let mut variables = self.parse_function_decl_parameters(attributes);

		// Skip Closing parenthese, as its the exit condition for self.parse_function_decl_parameters(attributes);
		let token_ret_type_specifier = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing function parameters.");
		});
		
		if token_ret_type_specifier.kind != TokenKind::Arrow
		{
			print_errln!(CompileError::Syntax, self.source, token_ret_type_specifier.span.start, "Expected return type specifier (Arrow operator \"->\") after parameter list in function declaration.");
		}

		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing functions return type.");
		});

		let return_type = self.parse_data_type().unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_ret_type_specifier.span.end, "Expected function return type after return type specifier.");
		});

		let token_scope_start = self.current_token();
		
		let mut function = Function::new(identifier.to_string(), return_type, attributes);
		function.parameter_count = variables.get_variable_count();
		
		if token_scope_start.kind == TokenKind::Semicolon
		{
			self.advance_token();
			function.code_block.stack_size = variables.end_scope();
			let locals = variables.get_variables_info();	
			function.locals = locals.vars;
			function.parameters_stack_size = locals.parameters_stack_size;
			self.func_manager.add(function);
			return;
		} else if token_scope_start.kind != TokenKind::LeftCurly
		{
			print_errln!(CompileError::Syntax, self.source, token_scope_start.span.start, "Expected scope begin operator \"{{\" or semicolon after function return type.");
		}
		
		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_scope_start.span.start, "While parsing function scope.");
		});

		let mut code_block = Scope::new(Vec::new());

		while self.current_token().kind != TokenKind::RightCurly
		{
			if let Some(statement) = self.parse_statement(&mut variables, &function)
			{
				code_block.add_statement(statement);
			}
		}
		self.advance_token();
		code_block.stack_size = variables.end_scope();

		let locals = variables.get_variables_info();
		function.locals = locals.vars;
		function.parameters_stack_size = locals.parameters_stack_size;

		code_block.stack_size += locals.parameters_stack_size;
		function.code_block = code_block;
		self.func_manager.add(function);
	}

	pub fn parse_scope(&mut self, variables: &mut LocalVariables, function: &Function) -> Scope
	{
		let mut scope = Scope::new(Vec::new());

		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing scope start.");
		});

		variables.start_scope();

		while self.current_token().kind != TokenKind::RightCurly
		{
			if let Some(statement) = self.parse_statement(variables, function)
			{
				scope.add_statement(statement);
			}
		}

		self.advance_token();

		scope.stack_size = variables.end_scope();
		return scope;
	}

	// NOTE: This function creates the LocalVariables struct and starts a scope, the callee must end the scope. end_scope()
	fn parse_function_decl_parameters(&mut self, attributes: AttributeType) -> LocalVariables
	{
		let mut args = LocalVariables::new(attributes);
		args.start_scope();
		loop 
		{
			let token_ident = self.current_token();
			match token_ident.kind 
			{
				TokenKind::RightParen => break,
				TokenKind::Ident =>
				{
					let ident = self.get_text(&token_ident.span).to_string();
					
					self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, self.source, token_ident.span.end, "While parsing function parameters.");
					});

					let data_type = self.parse_data_type().unwrap_or_else(|| {
						print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected parameter type after identifier.");
					});

					if data_type == Type::new(TypeKind::Void)
					{
						print_errln!(
							CompileError::Syntax,
							self.source, 
							token_ident.span.end, 
							"Cannot declare variable of type \"{KEYWORD_VOID}\", it makes no sense."
						);
					}

					// NOTE: (to my future self getting a headache) because arguments will be pushed on the stack from right to left,
					// The stack location (this variable will exist in the future) will just be positive
					args.add_variable(ident, attribute::FUNCTION_PARAMETER, data_type);

					let token_comma = self.current_token();

					
					match token_comma.kind {
						TokenKind::Comma => 
						{ 
							self.advance_token().unwrap_or_else(|| {
								print_errln!(CompileError::UnexpectedEof, self.source, token_comma.span.end, "While parsing function parameters.");
							});
							continue; 
						},
						TokenKind::RightParen => break,
						_ => { print_errln!(CompileError::Syntax, self.source, token_comma.span.start, "Unexpected token while parsing function parameters."); }
					}
				
				},
				_ => { print_errln!(CompileError::Syntax, self.source, token_ident.span.start, "Unexpected token while parsing function parameters."); }
			}
		}
		return args;	

	}

	fn parse_function_decl_attributes(&mut self) -> AttributeType
	{
		let mut token = self.current_token();
		let mut attributes = 0;
		while let Some(attr) = attribute::from_token_kind(&token.kind)
		{
			attributes |= attr;
			token = self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.source.len() - 1, "While parsing function attributes.");
			});
		}
		attributes |= attribute::SYS_V_ABI_X86_64; 	/* Just for now, in the future there will be more calling convenctions */
		return attributes;
	}

	pub fn parse_function_call(&mut self, variables: &LocalVariables) -> FunctionCallInfo
	{
		let identifier = self.get_text(&self.current_token().span);

		// Could clone the value, but i just love pointers sooo much (i do)
		let function: *const Function = self.func_manager.get(identifier).unwrap_or_else(|| {
			print_errln!(CompileError::UnknownIdentifier(identifier), self.source, self.current_token().span.start, "No such function.");
		});

		let token_left_paren = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing function.");
		});
		if token_left_paren.kind != TokenKind::LeftParen
		{
			print_errln!(CompileError::Syntax, self.source, token_left_paren.span.start, "Expected open parenthese.");
		}

		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing function call.");
		});

		unsafe 
		{
			let mut arguments: Vec<BinExpr> = Vec::with_capacity((*function).locals.len() as usize);
			for parameter in &(*function).locals[..(*function).parameter_count as usize]
			{
				if self.current_token().kind == TokenKind::RightParen
				{
					break;
				}
				
				let data_type = parameter.data_type;
				let argument = self.parse_expression(Some(data_type), variables);
				arguments.push(argument);

				if self.current_token().kind == TokenKind::RightParen
				{
					break;
				}

				if self.current_token().kind != TokenKind::Comma
				{
					print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected argument seperator \",\" or closing parenthese.");
				}

				self.advance_token().unwrap_or_else(|| {
					print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing function call arguments.");
				});
			}
			
			if self.current_token().kind != TokenKind::RightParen
			{
				print_errln!(
					CompileError::Syntax, 
					self.source, 
					token_left_paren.span.start, 
					"The function \"{}\" takes {} parameters but more were given.", (*function).identifier, (*function).parameter_count
				);
			}

			if arguments.len() != (*function).parameter_count as usize 
			{
				print_errln!(
					CompileError::Syntax, 
					self.source, 
					token_left_paren.span.start,
					"The function \"{}\" takes {} parameters but {} were given.", (*function).identifier, (*function).parameter_count, arguments.len()
				);
			}
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "After function call.");
			});
			
			return FunctionCallInfo::new((*function).index, arguments);
		}

	}

}