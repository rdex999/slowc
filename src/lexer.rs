mod tokens;

use core::{str, panic};
use std::iter::Peekable;

pub use tokens::*;

use crate::{error::CompileErrors, print_err};

// Note: variables will be declared in the following format: "create i32 my_number = 420;"
// or "ויהי חתום32 מספר_או_משהו = 420;" read from right to left, and the semicolon if actually on the end of the sentence (at the left part)
const KEYWORD_CREATE: &str = if cfg!(feature = "hebrew") { "ויהי" } else { "create" };

pub struct Lexer<'a>
{
	source: &'a str,
	iter: Peekable<str::Chars<'a>>,
	current: Option<char>,
	position: usize,
}

impl<'a> Lexer<'a>
{
	pub fn new(source: &'a str) -> Lexer<'a>
	{
		let mut iter = source.chars().peekable();
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
			return Token::new(
				TokenKind::Eof, 
				TextSpan::new(self.position, self.position)
			);
		}
		
		let ch = self.current.unwrap();
		let start = self.position;
		let end: usize;

		if Self::is_number_start(ch)
		{
			let number = self.parse_number();	
			end = self.position;
			return Token::new(
				TokenKind::IntLit(number), 
				TextSpan::new(start, end)
			);
		}

		if Self::is_op_start(ch)
		{
			if let Some(_next_ch) = self.advance()
			{
				end = self.position;
				let kind: TokenKind;

				match ch {
					'+' => kind = TokenKind::Plus,
					'-' => kind = TokenKind::Minus,
					'*' => kind = TokenKind::Asterisk,
					'/' => kind = TokenKind::ForwardSlash,
					'(' => kind = TokenKind::LeftParen,
					')' => kind = TokenKind::RightParen,
					';' => kind = TokenKind::Semicolon,

					_ => {
						panic!("Dev error!!!!!\nLexer, operator match default");
					}
				}

				return Token::new(
					kind,
					TextSpan::new(start, end)
				);

			} else
			{
				print_err!(CompileErrors::UnexpectedEof, "Opetator \"{ch}\" found at the end of the file.");
			}
		}

		if Self::is_name_start(ch)
		{
			let mut name = String::from(ch);
			while let Some(next_ch) = self.advance()
			{
				if Self::is_name_part(next_ch)
				{
					name.push(next_ch);
				} else
				{
					break;
				}	
			}
			end = self.position;
			let kind: TokenKind;

			match &name[..] {
				KEYWORD_CREATE => kind = TokenKind::Create,
				_ => kind = TokenKind::Ident(name)
			}

			return Token::new(
				kind,
				TextSpan::new(start, end)
			);
		}

		return Token::new(
			TokenKind::Eof, 
			TextSpan::new(self.position, self.position)
		);
	}

	fn advance(&mut self) -> Option<char>
	{
		self.position += 1;
		self.current = self.iter.next();
		return self.current;
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
	
	fn is_number_start(ch: char) -> bool
	{
		return ch.is_digit(10);
	}
	
	// Operators such as (, ), *, +, -, ;, /, \, 
	fn is_op_start(ch: char) -> bool
	{
		return !ch.is_alphanumeric() && ch != '\'' && ch != '\"' && !ch.is_whitespace();
	}
	
	fn is_whitespace(ch: char) -> bool
	{
		return ch.is_whitespace();
	}

	fn is_name_start(ch: char) -> bool
	{
		return ch.is_alphabetic() || ch == '_';
	}

	fn is_name_part(ch: char) -> bool
	{
		return ch.is_alphanumeric() || ch == '_';
	}

	// fn peek(&mut self) -> Option<char>
	// {
	// 	let next = self.iter.peek().cloned();
	// 	return next;
	// }

}