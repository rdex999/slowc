use super::*;
use crate::{ast::*, error::CompileError, print_errln, print_wrnln };
use super::{Parser, variable::*, };

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, data_type: Option<Type>, variables: &LocalVariables) -> BinExpr 
	{
		if let Some(data_type) = data_type
		{
			return self.parse_bin_expression(data_type, variables);
		}
		let data_type = self.get_expression_type(variables);
		return self.parse_bin_expression(data_type, variables);
	}

	// Doesnt actually mutate self
	pub fn get_expression_type(&mut self, variables: &LocalVariables) -> Type
	{
		let position = self.position;

		while self.current_token().kind == TokenKind::LeftParen || BinExprOperator::from_token_kind(&self.current_token().kind, true) != None
		{ 
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected closing parenthese.");
			});
		}

		if let Some(data_type) = Type::from_token_kind(&self.current_token().kind)
		{
			self.position = position;
			return data_type;
		} else
		{
			let value = self.parse_value(None, variables, false).unwrap_or_else(|| {
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Unexpected token found in binary expression.");
			});
			self.position = position;
			return self.value_type(&value, variables);	
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

	fn parse_bin_expression_part(&mut self, mut data_type: Type, variables: &LocalVariables) -> BinExprPart
	{
		let mut root;
		
		if let Some(expression) = self.parse_self_operator(data_type, variables)
		{
			root = expression;
		} else
		{
			root = self.parse_bin_expression_high_precedence(
				data_type, 
				variables, 
				BinExprOperator::LOWEST_PRECEDENCE + 1
			);
		}

		while let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind, false)
		{
			if operator.is_self_operator()
			{
				print_errln!(
					CompileError::Syntax, 
					self.source, 
					self.current_token().span.start, 
					"Expected two-side operator (+, -, *, /, ...), found {}", self.get_text(&self.current_token().span)
				);
			}	
			
			self.parse_bin_operator();

			// After a && or a || the numeric expression can have a different data type. For example: if 5 > 6 && 1.420 < 2.5
			if operator == BinExprOperator::BoolAnd || operator == BinExprOperator::BoolOr
			{
				data_type = self.get_expression_type(variables);
			}

			let rhs = self.parse_bin_expression_high_precedence(
				data_type, 
				variables, 
				BinExprOperator::LOWEST_PRECEDENCE + 1 // if operator.precedence() == BinExprOperator::HIGHEST_PRECEDENCE {operator.precedence()} else {operator.precedence() + 1}
			);

			root = BinExprPart::Operation(Box::new(BinExprOperation::new(operator, root, rhs)));
		}
		return root;
	}

	fn parse_self_operator(&mut self, data_type: Type, variables: &LocalVariables) -> Option<BinExprPart>
	{
		if let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind, true)
		{
			if !operator.is_self_operator()
			{
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected value or self operator (~, !, &).");
			}

			let operator_token = self.current_token();
			if operator == BinExprOperator::AddressOf 
			{
				if data_type != Type::U64		/* TODO: Switch to pointer-type */
				{
					print_errln!(
						CompileError::TypeError(data_type, Type::U64), 		/* TODO: Switch to pointer-type */
						self.source, 
						operator_token.span.start, 
						"Expected pointer data type ( * ) or {}.", Type::U64.to_string()
					);
				}

				self.parse_bin_operator();
				let value = self.parse_value(None, variables, true).unwrap_or_else(|| {
					print_errln!(
						CompileError::Syntax, 
						self.source, 
						operator_token.span.end, 
						"The address-of operator ( & ) cannot be applied to expressions. (As they are not stored in RAM)"
					);
				});
				return Some(BinExprPart::SelfOperation(Box::new(BinExprSelfOperation::new(operator, BinExprPart::Val(value)))));
			}

			self.parse_bin_operator();
			let expression = self.parse_bin_expression_high_precedence(data_type, variables, operator.precedence());
			return Some(BinExprPart::SelfOperation(Box::new(BinExprSelfOperation::new(operator, expression))));
		}
		return None;
	}

	fn parse_bin_expression_high_precedence(&mut self, data_type: Type, variables: &LocalVariables, precedence: u8) -> BinExprPart
	{
		let mut root;
		if let Some(expression) = self.parse_self_operator(data_type, variables)
		{
			root = expression;
		} else
		{
			root = self.parse_value_expr(data_type, variables);
		}

		while let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind, false)
		{
			if operator.precedence() < precedence
			{
				break;
			}

			self.parse_bin_operator();	

			// FIXME: Fix myself
			let rhs = self.parse_bin_expression_high_precedence(
				data_type, 
				variables, 
				if operator.precedence() == BinExprOperator::HIGHEST_PRECEDENCE {operator.precedence()} else {operator.precedence() + 1}
			);

			root = BinExprPart::Operation(Box::new(BinExprOperation::new(operator, root, rhs)));
		}
		return root;

	}

	fn parse_value_expr(&mut self, data_type: Type, variables: &LocalVariables) -> BinExprPart
	{
		let result;
		if self.current_token().kind == TokenKind::LeftParen
		{
			let next_token = self.peek(1).unwrap_or_else(|| {
				print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing expression.");
			});
			// If casting to a data type. (u64)420
			if let Some(_) = Type::from_token_kind(&next_token.kind)
			{
				result = self.parse_type_cast(variables, Some(data_type));
				
				// Else, if its just normal parentheses 5 * (2 + 10)
			} else
			{
				self.advance_token().unwrap_or_else(|| {
					print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing expression.");
				});
				result = self.parse_bin_expression_part(data_type, variables);
	
				if self.current_token().kind != TokenKind::RightParen
				{
					print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected closing parenthese.");
				}
	
				self.advance_token().unwrap_or_else(|| {
					print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing expression.");
				});
			}
		} else
		{
			result = BinExprPart::Val(self.parse_value(Some(data_type), variables, false).unwrap_or_else(|| {
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "None-binary token found in binary expression.");
			}));
		}
		return result;
	}
	
	fn parse_bin_operator(&mut self) -> BinExprOperator
	{
		let token = self.current_token();
		self.advance_token();
		if let Some(operator) = BinExprOperator::from_token_kind(&token.kind, false)
		{
			return operator;
		} 
		print_errln!(CompileError::Syntax, self.source, token.span.start, "None-binary operator found in binary expression.");
	}

	fn parse_type_cast(&mut self, variables: &LocalVariables, data_type: Option<Type>) -> BinExprPart
	{
		let token_into_type = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.end, "While parsing expression.");
		});
		let into_type = Type::from_token_kind(&token_into_type.kind).unwrap_or_else(|| {
			panic!("Dev error, parse_type_cast called on a non type cast.");
		});
		
		if into_type == Type::Void
		{
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Cannot cast to {} data type.", Type::Void.to_string());
		}
		
		if data_type != None && into_type != data_type.unwrap()
		{
			print_errln!(
				CompileError::TypeError(data_type.unwrap(), into_type), 
				self.source, 
				self.current_token().span.start, 
				"Can only cast to the expressions data type."
			);
		}
		
		let token_right_paren = self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing type cast.");
		});
		if token_right_paren.kind != TokenKind::RightParen
		{
			print_errln!(CompileError::Syntax, self.source, token_right_paren.span.start, "Expected closing parenthese on type cast.");
		}
		self.advance_token().unwrap_or_else(|| {
			print_errln!(CompileError::UnexpectedEof, self.source, token_right_paren.span.end, "While parsing expression.");
		});

		let from_type = self.get_expression_type(variables);
		let expression = self.parse_value_expr(from_type, variables);
		if from_type == into_type
		{
			print_wrnln!(self.source, token_into_type.span.start, "Type cast ignored, casting to the same data type.");
			return expression;
		}

		return BinExprPart::TypeCast(Box::from(TypeCastInfo::new(into_type, from_type, expression)));
	}
}