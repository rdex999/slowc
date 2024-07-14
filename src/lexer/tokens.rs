#[derive(Debug)]
pub struct Token
{
	pub kind: TokenKind,
	#[allow(dead_code)]
	pub span: TextSpan,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TextSpan
{
	start: usize,
	end: usize,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind
{
	Eof,
	IntLit(i64),
	// Plus,
	// Minus,
	// Asterisk,
	// ForwardSlash,
	// BackwardsSlash,
}

impl Token
{
	pub fn new(kind: TokenKind, span: TextSpan) -> Self
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