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
		
		self.advance_token();
		let mut variables = self.parse_function_decl_parameters();
		
		let args_end_pos = self.current_token().span.end;
		let token_ret_type_specifier = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, args_end_pos, "While parsing function return type specifier (Arrow operator \"->\")");
		});
		
		if token_ret_type_specifier.kind != TokenKind::Arrow
		{
			print_errln!(CompileError::Syntax, self.source, token_ret_type_specifier.span.start, "Expected return type specifier (Arrow operator \"->\") after parameter list in function declaration.");
		}
		
		let token_ret_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ret_type_specifier.span.end, "While parsing function return type.");
		});
		
		let return_type = Type::from_token_kind(&token_ret_type.kind).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_ret_type.span.start, "Expected function return type after return type specifier.");
		});
		
		let token_scope_start = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_ret_type.span.end, "While parsing function declaration.");
		});
		
		let mut function = Function::new(identifier.to_string(), return_type, attributes);
		function.parameter_count = variables.get_variable_count();

		if token_scope_start.kind == TokenKind::Semicolon
		{
			self.advance_token();
			let locals = variables.get_variables_info(attributes);	
			function.locals = locals.vars;
			function.code_block.stack_size = locals.stack_size;
			function.parameters_stack_size = locals.parameters_stack_size;
			self.func_manager.add(function);
			return;
		} else if token_scope_start.kind != TokenKind::LeftCurly
		{
			print_errln!(CompileError::Syntax, self.source, token_ret_type.span.start, "Expected scope begin operator \"{{\" or semicolon after function return type.");
		}

		let mut code_block = self.parse_scope(&mut variables, &function);

		if function.return_type != Type::Void
		{
			let mut has_return_stmt = false;
			for statement in &code_block.statements
			{
				if let Statement::Return(_) = statement
				{
					has_return_stmt = true;
					break;
				}
			}
			if !has_return_stmt
			{
				print_errln!(CompileError::Syntax, self.source, token_ret_type.span.start, "Expected return statement because of the functions return type.");
			}
		}

		let locals = variables.get_variables_info(attributes);
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

	fn parse_function_decl_parameters(&mut self) -> LocalVariables
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

					let data_type = Type::from_token_kind(&token_arg_type.kind).unwrap_or_else(|| {
						print_errln!(CompileError::Syntax, self.source, token_arg_type.span.start, "Expected parameter type after identifier.");
					});

					if data_type == Type::Void
					{
						print_errln!(
							CompileError::TypeError(Type::I32, Type::Void), 
							self.source, 
							token_arg_type.span.start, 
							"Cannot declare variable of type \"{KEYWORD_VOID}\", it makes no sense."
						);
					}

					// NOTE: (to my future self getting a headache) because arguments will be pushed on the stack from right to left,
					// The stack location (this variable will exist in the future) will just be positive
					args.add_variable(ident, attribute::FUNCTION_PARAMETER, data_type);

					let token_comma = self.advance_token().unwrap_or_else(|| {
						print_errln!(CompileError::UnexpectedEof, self.source, token_arg_type.span.end, "While parsing function parameters.");
					});

					match token_comma.kind {
						TokenKind::Comma => { self.advance_token(); continue; },
						TokenKind::RightParen => break,
						_ => { print_errln!(CompileError::Syntax, self.source, token_span.start, "Unexpected token while parsing function parameters."); }
					}
				
				},
				_ => { print_errln!(CompileError::Syntax, self.source, token_span.start, "Unexpected token while parsing function parameters."); }
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
			let mut arguments: Vec<ExprType> = Vec::with_capacity((*function).locals.len() as usize);
			for parameter in &(*function).locals[..(*function).parameter_count as usize]
			{
				if self.current_token().kind == TokenKind::RightParen
				{
					break;
				}
				
				let data_type = parameter.data_type;
				let argument = self.parse_expression(data_type, variables);
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