use parser::TokenKind;
use crate::{ast::*, error::CompileError, print_errln };
use super::{Parser, variable::*, };

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, data_type: Type, variables: &LocalVariables) -> BinExpr 
	{
		if data_type.is_bin_expr_type()
		{
			return self.parse_bin_expression(data_type, variables);
		} else
		{
			todo!("Dev error!! parse_expression() - None binary expressions are not yet supported.");
		}
	}
	
	pub fn parse_value(&mut self, data_type: Option<Type>, variables: &LocalVariables, is_lvalue: bool) -> Option<Value>
	{
		let first_token = self.current_token();
		match first_token.kind
		{
			TokenKind::IntLit(value) => 
			{
				self.advance_token();
				if is_lvalue
				{
					print_errln!(CompileError::Syntax, self.source, first_token.span.start, "Expected modifiable lvalue.");
				}
	
				if let Some(data_type) = data_type
				{
					return match data_type {
						Type::I8  => Some(Value::I8(value as i8)),
						Type::U8  => Some(Value::U8(value as u8)),
						Type::I16 => Some(Value::I16(value as i16)),
						Type::U16 => Some(Value::U16(value as u16)),
						Type::I32 => Some(Value::I32(value as i32)),
						Type::U32 => Some(Value::U32(value as u32)),
						Type::I64 => Some(Value::I64(value)),
						Type::U64 => Some(Value::U64(value as u64)),
	
						_ => { print_errln!(CompileError::TypeError(data_type, Type::I32), self.source, first_token.span.start, ""); }
					}
				}
				return Some(Value::I32(value as i32));
			},

			TokenKind::FloatLit(value) =>
			{
				self.advance_token();
				if is_lvalue
				{
					print_errln!(CompileError::Syntax, self.source, first_token.span.start, "Expected modifiable lvalue.");
				}

				if let Some(data_type) = data_type
				{
					return match data_type
					{
						Type::F32 => Some(Value::F32(value as f32)),
						Type::F64 => Some(Value::F64(value)),
						_ => { print_errln!(CompileError::TypeError(data_type, Type::F32), self.source, first_token.span.start, ""); }
					}
				}
				return Some(Value::F64(value));
			}
			
			TokenKind::Ident =>
			{
				if let Some(next_token) = self.peek(1)
				{
					if next_token.kind == TokenKind::LeftParen
					{
						let function_call = Value::FuncCall(self.parse_function_call(variables));
						let func_ret_type = self.value_type(&function_call, variables);
						if data_type != None && func_ret_type != data_type.unwrap()
						{
							print_errln!(
								CompileError::TypeError(data_type.unwrap(), func_ret_type), 
								self.source, 
								first_token.span.start, 
								"When parsing function call."
							);
						}
						return Some(function_call);
					}
				}
	
				self.advance_token();
				let ident = self.get_text(&first_token.span);
				
				let var = variables.get_variable(ident).unwrap_or_else(|| {
					print_errln!(CompileError::UnknownIdentifier(ident), self.source, first_token.span.start, "");
				});
				
				if let Some(data_type) = data_type
				{
					if var.data_type != data_type
					{
						print_errln!(CompileError::TypeError(data_type, var.data_type), self.source, first_token.span.start, "");
					}
				}
				return Some(Value::Var(var.index));
			},
	
			_ => return None
		}
	}

	fn parse_bin_expression(&mut self, data_type: Type, variables: &LocalVariables) -> BinExpr
	{
		let expression_root = self.parse_bin_expression_part(data_type, variables);
		return BinExpr::new(expression_root);
	}

	fn parse_bin_expression_part(&mut self, data_type: Type, variables: &LocalVariables) -> BinExprPart
	{
		let mut root = self.parse_bin_expression_high_precedence(
			data_type, 
			variables, 
			BinExprOperator::Add.precedence() + 1
		);
	
		while let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
		{
			self.parse_bin_operator();

			if operator.is_boolean() && data_type.size() != 1
			{
				print_errln!(CompileError::TypeError(data_type, Type::U8), self.source, self.current_token().span.start, "Found boolean operator in a non-boolean expression.");
			}

			let rhs = self.parse_bin_expression_high_precedence(
				data_type, 
				variables, 
				operator.precedence()
			);

			root = BinExprPart::Operation(Box::new(BinExprOperation::new(operator, root, rhs)));
		}
		return root;
	}

	fn parse_bin_expression_high_precedence(&mut self, data_type: Type, variables: &LocalVariables, precedence: u8) -> BinExprPart
	{
		let mut root = self.parse_value_expr(data_type, variables);

		while let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
		{
			if operator.precedence() < precedence
			{
				break;
			}

			if operator.is_boolean() && data_type.size() != 1
			{
				print_errln!(CompileError::TypeError(data_type, Type::U8), self.source, self.current_token().span.start, "Found boolean operator in a non-boolean expression.");
			}

			self.parse_bin_operator();	
			let rhs = self.parse_value_expr(data_type, variables);
			root = BinExprPart::Operation(Box::new(BinExprOperation::new(operator, root, rhs)));
		}
		return root;

	}

	fn parse_value_expr(&mut self, data_type: Type, variables: &LocalVariables) -> BinExprPart
	{
		if self.current_token().kind == TokenKind::LeftParen
		{
			let value;
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing expression.");
			});

			value = self.parse_bin_expression_part(data_type, variables);

			if self.current_token().kind != TokenKind::RightParen
			{
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected closing parenthese.");
			}

			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing expression.");
			});

			return value;
		} else
		{
			return BinExprPart::Val(self.parse_value(Some(data_type), variables, false).unwrap_or_else(|| {
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "None-binary token found in binary expression.");
			}));
		}
	}
	
	fn parse_bin_operator(&mut self) -> BinExprOperator
	{
		let token = self.current_token();
		self.advance_token();
		if let Some(operator) = BinExprOperator::from_token_kind(&token.kind)
		{
			return operator;
		} 
		print_errln!(CompileError::Syntax, self.source, token.span.start, "None-binary operator found in binary expression.");
	}


}