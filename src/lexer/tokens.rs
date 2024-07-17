#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Token
{
	pub kind: TokenKind,
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
pub enum TokenKind
{
	None,
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
	Semicolon,
	Ident(String),
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