mod tokens;
mod common;
mod lex_general;

use core::str;
use std::iter::Peekable;

pub use tokens::*;

use super::{error::CompileError, print_err};

#[macro_export]
macro_rules! create_keyword {
	($heb:expr, $eng:expr) => {
		if cfg!(feature = "hebrew") 
		{ 
			$heb 
		} else 
		{ 
			$eng 
		}
	};
}

// Note: variables will be declared in the following format: "create i32 my_number = 420;"
// or "ויהי חתום32 מספר_או_משהו = 420;" read from right to left, and the semicolon if actually on the end of the sentence (at the left part)
pub const KEYWORD_VAR_DECL	: &str = create_keyword!("ויהי", "let");
pub const KEYWORD_VOID		: &str = create_keyword!("כלום", "void");
pub const KEYWORD_I32		: &str = create_keyword!("חתום32", "i32");
pub const KEYWORD_U32		: &str = create_keyword!("חיובי32", "u32");
pub const KEYWORD_I64		: &str = create_keyword!("חתום64", "i64");
pub const KEYWORD_FUNC_DECL	: &str = create_keyword!("פונקציה", "func");
pub const KEYWORD_RETURN	: &str = create_keyword!("החזר", "return");
pub const KEYWORD_GLOBAL	: &str = create_keyword!("גלובלי", "global");
pub const KEYWORD_EXTERN 	: &str = create_keyword!("חיצוני", "extern");

pub struct Lexer<'a>
{
	pub source: &'a str,
	itr: Peekable<str::Chars<'a>>,
	current: Option<char>,
	position: usize,
}



impl<'a> Iterator for Lexer<'a>
{
	type Item = Token;
	fn next(&mut self) -> Option<Self::Item> {
		
		while let Some(ch) = self.current
		{
			if ch == '/' && self.peek() != None && *self.peek().unwrap() == '/'
			{
				while self.current != None && self.current.unwrap() != '\n'
				{
					self.advance();
				}
				continue;
			}
			if Self::is_whitespace(ch)
			{
				self.advance();
			} else
			{
				break;
			}
		}

		// if self.position == self.source.len()
		// {
		// 	self.advance();
		// 	return Some(Token::new(
		// 		TokenKind::Eof, 
		// 		TextSpan::new(self.source.len(), self.source.len())
		// 	));
		// }
		
		if self.position >= self.source.len() || self.current == Option::None
		{
			return None;
		}
		
		let ch = self.current.unwrap();

		if Self::is_number_start(ch) 
		{
			return Some(self.lex_number());
		}

		if Self::is_op_start(ch)
		{
			return Some(self.lex_operator());
		}

		if Self::is_name_start(ch)
		{
			return Some(self.lex_name());
		}

		return None;
	}
}

impl<'a> Lexer<'a>
{
	pub fn new(source: &'a str) -> Lexer<'a>
	{
		let mut itr = source.chars().peekable();
		let current = itr.next();

		return Lexer
		{
			source,
			itr,	
			current,	
			position: 0,
		};
	}

	fn advance(&mut self) -> Option<char>
	{
		self.position += 1;
		self.current = self.itr.next();
		return self.current;
	}
}