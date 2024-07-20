use super::*;
impl<'a> Lexer<'a>
{
	pub fn lex_number(&mut self) -> Token<'a>
	{
		let mut number: i64 = 0;
		let start = self.position;
		while let Some(ch) = self.current
		{
			if let Some(digit) = ch.to_digit(10)
			{
				number = number * 10 + digit as i64;
			} else
			{
				break;
			}
			self.advance();
		}
		return Token::new(
			TokenKind::IntLit(number), 
			TextSpan::new(start, self.position)
		);
	}

	pub fn lex_operator(&mut self) -> Token<'a>
	{
		let ch = self.current.unwrap();
		let start = self.position - 1;
		let end = self.position;
		let kind: TokenKind;
		let next_ch = self.advance().unwrap_or('\0');

		match ch {
			'+' => kind = TokenKind::Plus,
			'-' => 
			{
				if next_ch == '>'
				{
					self.advance();
					kind = TokenKind::Arrow;
				} else
				{
					kind = TokenKind::Minus;
				}
			},
			'*' => kind = TokenKind::Asterisk,
			'/' => kind = TokenKind::ForwardSlash,
			'=' => kind = TokenKind::Equal,
			'(' => kind = TokenKind::LeftParen,
			')' => kind = TokenKind::RightParen,
			'{' => kind = TokenKind::LeftCurly,
			'}' => kind = TokenKind::RightCurly,
			';' => kind = TokenKind::Semicolon,
			',' => kind = TokenKind::Comma,
			
			_ => {
				let mut op = String::from(ch);
				op.push(next_ch);
				print_err!(CompileError::NoSuchOperator(&op[..]), "");
			}
		}

		return Token::new(
			kind,
			TextSpan::new(start, end)
		);
	}

	pub fn lex_name(&mut self) -> Token<'a>
	{
		let start = self.position;
		let end: usize;
		let mut ch = self.current.unwrap();
		
		loop
		{
			if !Self::is_name_part(ch)
			{
				break;
			}
			if let Some(next_ch) = self.advance()
			{
				ch = next_ch;
			}
		}	
		
		end = self.position;
		let kind: TokenKind;
		let name = &self.source[start..end];

		match name {
			KEYWORD_VAR_DECL 	=> kind = TokenKind::VarDecl,
			KEYWORD_FUNC_DECL	=> kind = TokenKind::FuncDecl,
			KEYWORD_I32			=> kind = TokenKind::I32,
			_ => kind = TokenKind::Ident(name)
		}

		return Token::new(
			kind,
			TextSpan::new(start, end)
		);
	}
}