#[derive(Clone, Copy, Debug)]
pub struct Token
{
	pub kind: TokenKind,
	pub span: TextSpan
}

#[derive(Clone, Copy, Debug)]
pub struct TextSpan
{
	pub start: usize,
	pub end: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenKind
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
	Ident,
	VarDecl,
	FuncDecl,
	I32,
}

impl Token
{
	pub fn new(kind: TokenKind, span: TextSpan) -> Token
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