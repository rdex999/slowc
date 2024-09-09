use variable::LocalVariables;

use super::*;

impl<'a> Parser<'a>
{
	pub fn get_text(&self, text_span: &TextSpan) -> &'a str
	{
		return &self.source[text_span.start..text_span.end];
	}

	pub fn peek(&self, offset: isize) -> Option<Token>
	{
		if (self.position as isize + offset) as usize >= self.tokens.len()
		{
			return None;
		}

		return Some(self.tokens[(self.position as isize + offset) as usize]);
	}

	pub fn advance_token(&mut self) -> Option<Token>
	{
		if self.position >= self.tokens.len() - 1
		{
			self.has_passed_eof = true;
			return None;
		}

		self.position += 1;
		return Some(self.tokens[self.position]);
	}

	pub fn current_token(&self) -> Token
	{
		return self.tokens[self.position];
	}

	pub fn value_type(&self, value: &Value, variables: &LocalVariables) -> Type
	{
		match value {
			Value::I8(_)  => Type::new(TypeKind::I8),
			Value::U8(_)  => Type::new(TypeKind::U8),
			Value::I16(_) => Type::new(TypeKind::I16),
			Value::U16(_) => Type::new(TypeKind::U16),
			Value::I32(_) => Type::new(TypeKind::I32),
			Value::U32(_) => Type::new(TypeKind::U32),
			Value::I64(_) => Type::new(TypeKind::I64),
			Value::U64(_) => Type::new(TypeKind::U64),
			Value::F32(_) => Type::new(TypeKind::F32),
			Value::F64(_) => Type::new(TypeKind::F64),
			Value::Var(index) => 
			{
				let var = variables.get_variable_by_index(*index).unwrap();
				return var.data_type;
			},
			Value::FuncCall(func_call) => return self.func_manager.get_by_index(func_call.index).unwrap().return_type,
			Value::Dereference(info) => return info.data_type.dereference(info.dereference_count),
		}
	}

	pub fn bin_expr_part_type(&self, part: &BinExprPart, variables: &LocalVariables) -> Type
	{
		match part
		{
			BinExprPart::Operation(operation) => return self.bin_expr_part_type(&operation.lhs, variables),
			BinExprPart::SelfOperation(operation) => return self.bin_expr_part_type(&operation.expression, variables),
			BinExprPart::TypeCast(info) => return info.into_type,
			BinExprPart::Val(value) => self.value_type(value, variables),
		}
	}

	// Self is not mutated if None is returned
	pub fn parse_data_type(&mut self) -> Option<Type>
	{
		let position = self.position;
		let mut pointer_level: u8 = 0;
		while self.current_token().kind == TokenKind::Asterisk
		{
			pointer_level += 1;
			if let None = self.advance_token()
			{
				self.position = position;
				return None;
			}
		}

		let kind = if let Some(kind) = TypeKind::from_token_kind(&self.current_token().kind) 
		{ 
			kind 
		} else 
		{ 
			self.position = position;
			return None; 
		};
		self.advance_token();

		if pointer_level == 0
		{
			return Some(Type::new(kind));
		} else
		{
			return Some(Type::new_ptr(TypeKind::Pointer, kind, pointer_level));
		}
	}


	// Doesnt mutate self
	pub fn parse_data_type_non_mut(&mut self, offset: usize) -> Option<Type>
	{
		let position = self.position;
		self.position += offset;
		let data_type = self.parse_data_type();
		self.position = position;
		return data_type;
	}
}