use variable::LocalVariables;

use super::*;

impl<'a> Parser<'a>
{
	pub fn get_text(&self, text_span: &TextSpan) -> &'a str
	{
		return &self.source[text_span.start..text_span.end];
	}

	pub fn _peek(&self, offset: usize) -> Option<Token>
	{
		if self.position + offset >= self.tokens.len()
		{
			return None;
		}

		return Some(self.tokens[self.position + offset]);
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
			Value::Var(index) => 
			{
				let var = variables.get_variable_by_index(*index).unwrap();
				return var.data_type;
			}
		}
	}
}