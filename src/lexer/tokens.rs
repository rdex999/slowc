#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Token<'a>
{
	pub kind: TokenKind<'a>,
	pub span: TextSpan
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct TextSpan
{
	pub start: usize,
	pub end: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind<'a>
{
	Eof,
	IntLit(i64),
	Plus,
	Minus,
	Asterisk,
	ForwardSlash,
	Equal,
	// BackwardsSlash,
	LeftParen,
	RightParen,
	LeftCurly,
	RightCurly,
	Arrow,
	Comma,
	Semicolon,
	Ident(&'a str),
	VarDecl,
	FuncDecl,
	I32,
}

impl<'a> Token<'a>
{
	pub fn new(kind: TokenKind<'a>, span: TextSpan) -> Token
	{
		return Self
		{
			kind,
			span,
		}	
	}
}

impl TextSpan
{
	pub fn new(start: usize, end: usize) -> Self
	{
		return Self
		{
			start,
			end,
		}
	}
}