use super::*;
impl<'a> Lexer<'a>
{
	pub fn lex_number(&mut self) -> Token
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

	pub fn lex_operator(&mut self) -> Token
	{
		let ch = self.current.unwrap();
		let start = self.position;			/* Might need -1 */
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

	pub fn lex_name(&mut self) -> Token
	{
		let start = self.position;
		let end: usize;
		let ch = self.current.unwrap();
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
			KEYWORD_VAR_DECL 	=> kind = TokenKind::VarDecl,
			KEYWORD_I32			=> kind = TokenKind::I32,
			KEYWORD_FUNC_DECL	=> kind = TokenKind::FuncDecl,
			KEYWORD_GLOBAL		=> kind = TokenKind::Global,
			KEYWORD_EXTERN		=> kind = TokenKind::Extern,
			_ => kind = TokenKind::Ident
		}

		return Token::new(
			kind,
			TextSpan::new(start, end)
		);
	}
}