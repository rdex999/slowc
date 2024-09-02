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

		while self.current_token().kind == TokenKind::LeftParen 
		{ 
			self.advance_token().unwrap_or_else(|| {
				print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected closing parenthese.");
			});
		}

		let value = self.parse_value(None, variables, false).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Unexpected token found in binary expression.");
		});

		self.position = position;
		return self.value_type(&value, variables);	
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
		let mut root = self.parse_bin_expression_high_precedence(
			data_type, 
			variables, 
			BinExprOperator::LOWEST_PRECEDENCE + 1
		);

		while let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
		{
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

	fn parse_bin_expression_high_precedence(&mut self, data_type: Type, variables: &LocalVariables, precedence: u8) -> BinExprPart
	{
		let mut root = self.parse_value_expr(data_type, variables);

		while let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
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

		// if self.current_token().kind == TokenKind::As
		// {
		// 	let to_type_tok = self.advance_token().unwrap_or_else(|| {
		// 		print_errln!(CompileError::UnexpectedEof, self.source, self.current_token().span.start, "While parsing type cast.");
		// 	});
		// 	let to_type = Type::from_token_kind(&to_type_tok.kind).unwrap_or_else(|| {
		// 		print_errln!(CompileError::Syntax, self.source, to_type_tok.span.start, "Expected data type after \"{KEYWORD_AS}\" keyword.");
		// 	});

		// 	if to_type == Type::Void
		// 	{
		// 		print_errln!(CompileError::Syntax, self.source, to_type_tok.span.start, "Cannot cast to {} data type.", Type::Void.to_string());
		// 	}

		// 	if to_type != data_type
		// 	{
		// 		print_errln!(CompileError::TypeError(data_type, to_type), self.source, to_type_tok.span.start, "Can only cast to the expressions") 
		// 	}

		// 	self.advance_token().unwrap_or_else(|| {
		// 		print_errln!(CompileError::UnexpectedEof, self.source, to_type_tok.span.end, "While parsing expression.");
		// 	});


		// }
		return result;
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