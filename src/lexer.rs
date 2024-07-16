mod tokens;
mod common;
mod lex_general;

use core::str;
use std::iter::Peekable;

pub use tokens::*;

use super::{error::CompileErrors, print_err};

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
const KEYWORD_VAR_DECL	: &str 	= create_keyword!("ויהי", "let");
const KEYWORD_FUNC_DECL	: &str 	= create_keyword!("פונקציה", "func");

pub struct Lexer<'a>
{
	source: &'a str,
	itr: Peekable<str::Chars<'a>>,
	current: Option<char>,
	position: usize,
}

impl<'a> Iterator for Lexer<'a>
{
	type Item = Token;
	fn next(&mut self) -> Option<Self::Item> {
		while let Some(current_ch) = self.current
		{
			if Self::is_whitespace(current_ch)
			{
				self.advance();
			} else
			{
				break;
			}
		}
		
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