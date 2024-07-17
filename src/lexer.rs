mod tokens;
mod common;
mod lex_general;

use core::str;
use std::iter::Peekable;

pub use tokens::*;

use super::{error::CompileError, print_err};

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
pub const KEYWORD_FUNC_DECL	: &str = create_keyword!("פונקציה", "func");
pub const KEYWORD_I32		: &str = create_keyword!("חתום32", "i32");

pub struct Lexer<'a>
{
	source: &'a str,
	itr: Peekable<str::Chars<'a>>,
	current: Option<char>,
	position: usize,
}

pub struct LineInfo
{
	pub line_index: usize,
	pub column: usize,
	pub line_contents: String	
}

impl LineInfo
{
	pub fn new(line_index: usize, column: usize, line_contents: String) -> Self
	{
		return Self {
			line_index,
			column,
			line_contents
		};
	}
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

	// Takes O(n), line, column, line_contents
	pub fn get_line_from_index(&self, mut index: usize) -> Option<LineInfo>
	{
		if index >= self.source.len()
		{
			return None;
		}

		let mut idx_in_source: usize = 0;
		for (i, line) in self.source.lines().enumerate()
		{
			idx_in_source += line.len();
			
			if idx_in_source > index
			{
				return Some(LineInfo::new(
					i, 
					index % idx_in_source,
					line.to_string()
				));
			}
			
			index -= line.len() + 1;
		}

		return None;
	}
}