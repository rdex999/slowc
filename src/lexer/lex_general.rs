use super::*;
impl<'a> Lexer<'a>
{
	pub fn lex_number(&mut self) -> Token
	{
		let start = self.position;
		let mut number = String::with_capacity(8);
		while let Some(ch) = self.current
		{
			if !ch.is_digit(0x10) && ch != '.'
			{
				break;
			}

			number.push(ch);
			self.advance();
		}

		match number.parse::<i64>()
		{
			Ok(number) => 
			{
				return Token::new(
					TokenKind::IntLit(number), 
					TextSpan::new(start, self.position)
				);
			}
			Err(_) => 
			{
				let number: f64 = number.parse().unwrap();
				return Token::new(
					TokenKind::FloatLit(number), 
					TextSpan::new(start, self.position)
				);
			}
		}
	}

	pub fn lex_operator(&mut self) -> Token
	{
		let ch = self.current.unwrap();
		let start = self.position;			/* Might need -1 */
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
		
		let end = self.position;
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
			KEYWORD_VOID		=> kind = TokenKind::Void,
			KEYWORD_I8			=> kind = TokenKind::I8,
			KEYWORD_U8			=> kind = TokenKind::U8,
			KEYWORD_I16			=> kind = TokenKind::I16,
			KEYWORD_U16			=> kind = TokenKind::U16,
			KEYWORD_I32			=> kind = TokenKind::I32,
			KEYWORD_U32			=> kind = TokenKind::U32,
			KEYWORD_I64			=> kind = TokenKind::I64,
			KEYWORD_U64			=> kind = TokenKind::U64,
			KEYWORD_F32			=> kind = TokenKind::F32,
			KEYWORD_F64			=> kind = TokenKind::F64,
			KEYWORD_FUNC_DECL	=> kind = TokenKind::FuncDecl,
			KEYWORD_RETURN		=> kind = TokenKind::Return,
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