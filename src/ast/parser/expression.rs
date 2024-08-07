use parser::TokenKind;
use crate::{ast::*, error::CompileError, print_errln };
use super::{Parser, variable::*, };

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, data_type: Type, variables: &LocalVariables) -> ExprType
	{
		if data_type.is_bin_expr_type()
		{
			return ExprType::BinExprT(self.parse_bin_expression(data_type, variables));
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
		let expression_root = self.parse_bin_expression_part(data_type, variables, BinExprOperator::LOWEST_PRECEDENCE);
		return BinExpr::new(expression_root);
	}

	fn parse_bin_expression_part(&mut self, data_type: Type, variables: &LocalVariables, precedence: u8) -> BinExprPart
	{
		let lhs;
		if self.current_token().kind == TokenKind::LeftParen
		{
			self.advance_token();
			lhs = self.parse_bin_expression_part(data_type, variables, BinExprOperator::LOWEST_PRECEDENCE);
			if self.current_token().kind != TokenKind::RightParen
			{
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected closing parenthese. \")\"");
			}
			self.advance_token();
		} else 
		{
			lhs = BinExprPart::Val(self.parse_value(Some(data_type), variables, false).unwrap_or_else(|| {
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "None-binary token found in binary expression.");
			}));	
		}

		if let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
		{
			if operator.precedence() >= precedence
			{
				self.parse_bin_operator();
				// Ik recursion is bad, but it an advantage is this situation - 
				// The same could be done with a stack data structure, (Vec) but this would require heap allcating memory which is slow af
				// In this way, everything is on the stack, the because its just an expression the recursion is not big, so the stack wont commit suicide
				let rhs = self.parse_bin_expression_part(data_type, variables, operator.precedence());

				if let Some(next_operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
				{
					if next_operator.precedence() >= precedence
					{
						self.parse_bin_operator();
						let low_precedence_part = self.parse_bin_expression_part(data_type, variables, next_operator.precedence());
						// Makes me think about my lifes deceisions
						return BinExprPart::Operation(Box::new(BinExprOperation::new(
							next_operator,
							BinExprPart::Operation(Box::new(BinExprOperation::new(operator, lhs, rhs))),
							low_precedence_part
						)));

					}
				}
				return BinExprPart::Operation(Box::new(BinExprOperation::new(operator, lhs, rhs)));
			} else
			{
				return lhs;
			}
		} else
		{
			return lhs;
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