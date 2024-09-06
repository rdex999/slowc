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
	// Eof,
	IntLit(i64),
	FloatLit(f64),
	BoolAnd,
	BoolOr,
	BoolEq,
	BoolNotEq,
	BoolGreater,
	BoolLess,
	BoolGreaterEq,
	BoolLessEq,
	BitwiseOr,
	BitwiseXor,
	BitwiseAnd,
	BitwiseRightShift,
	BitwiseLeftShift,
	BitwiseNot,
	Plus,
	Minus,
	Asterisk,
	ForwardSlash,
	Percent,
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
	Void,
	I8,
	U8,
	I16,
	U16,
	I32,
	U32,
	I64,
	U64,
	F32,
	F64,
	As,
	FuncDecl,
	Return,
	Global,
	Extern,
	If,
	Else,
	For,
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