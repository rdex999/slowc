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
			Value::I32(_) => Type::I32,
			Value::U32(_) => Type::U32,
			Value::I64(_) => Type::I64,
			Value::U64(_) => Type::U64,
			Value::Var(index) => 
			{
				let var = variables.get_variable_by_index(*index).unwrap();
				return var.data_type;
			},
			Value::FuncCall(func_call) => return self.func_manager.get_by_index(func_call.index).unwrap().return_type,
		}
	}
}