use super::*;

impl<'a> Lexer<'a>
{
	pub fn is_number_start(ch: char) -> bool
	{
		return ch.is_digit(10);
	}
	
	// Operators such as (, ), *, +, -, ;, /, \, 
	pub fn is_op_start(ch: char) -> bool
	{
		return !ch.is_alphanumeric() && ch != '\'' && ch != '\"' && !ch.is_whitespace();
	}
	
	pub fn is_whitespace(ch: char) -> bool
	{
		return ch.is_whitespace();
	}

	pub fn is_name_start(ch: char) -> bool
	{
		return ch.is_alphabetic() || ch == '_';
	}

	pub fn is_name_part(ch: char) -> bool
	{
		return ch.is_alphanumeric() || ch == '_';
	}

	pub fn peek(&mut self) -> Option<&char>
	{
		return self.itr.peek();
	}
}