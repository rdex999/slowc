mod tokens;

pub use tokens::*;

pub struct Lexer<'a>
{
	source: &'a str,
	iter: core::str::Chars<'a>,
	current: Option<char>,
	position: usize,
}

impl Lexer<'_>
{
	pub fn new<'a>(source: &'a str) -> Lexer<'a>
	{
		let mut iter = source.chars();	
		let current = iter.next();

		return Lexer
		{
			source,
			iter,	
			current,	
			position: 0,
		};
	}

	// Could just make a vectors of tokens, but its inefficient. This is much better
	pub fn next_token(&mut self) -> Token
	{
		if self.position >= self.source.len() || self.current == Option::None
		{
			return Token::new(
				TokenKind::Eof, 
				TextSpan::new(self.position, self.position)
			);
		}

		let ch = self.current.unwrap();
		if Self::is_number_start(ch)
		{
			let start = self.position;
			let end: usize;	
			let number = self.parse_number();	
			end = self.position;
			return Token::new(
				TokenKind::IntLit(number), 
				TextSpan::new(start, end)
			);
		}

		return Token::new(TokenKind::Eof, TextSpan::new(self.position, self.position));
	}

	fn advance(&mut self) -> Option<char>
	{
		self.position += 1;
		self.current = self.iter.next();
		return self.current;
	}

	fn is_number_start(ch: char) -> bool
	{
		return ch.is_digit(10);
	}

	fn parse_number(&mut self) -> i64
	{
		let mut result: i64 = 0;
		while let Some(ch) = self.current
		{
			if let Some(digit) = ch.to_digit(10)
			{
				result = result * 10 + digit as i64;
			} else
			{
				break;
			}
			self.advance();
		}
		return result;
	}

}