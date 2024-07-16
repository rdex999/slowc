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
		if let Some(next_ch) = self.advance()
		{
			let start = self.position - 1;
			let end = self.position;
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
					let mut op = String::from(ch);
					if Self::is_op_start(next_ch) { op.push(next_ch);}
					print_err!(CompileErrors::NoSuchOperator(&op[..]), "");
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
			KEYWORD_VAR_DECL => kind = TokenKind::VarDecl,
			_ => kind = TokenKind::Ident(name)
		}

		return Token::new(
			kind,
			TextSpan::new(start, end)
		);
	}
}