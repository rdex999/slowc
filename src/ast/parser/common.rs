use super::*;

impl<'a> Parser<'a>
{
	// Will return None if the given token kind is not a type
	pub fn kind_2_type(token_kind: &TokenKind) -> Option<Type>
	{
		match token_kind {
			TokenKind::I32 => return Some(Type::I32),
			_ => return None
		};
	}

	pub fn get_text(&self, text_span: &TextSpan) -> &'a str
	{
		return &self.source[text_span.start..text_span.end];
	}

	pub fn is_bin_expr_type(data_type: &Type) -> bool
	{
		return *data_type == Type::I32;
	}

	// Returns None is this function was called with a none binary operator
	pub fn kind_2_bin_op(token_kind: &TokenKind) -> Option<BinExprOp>
	{
		match token_kind {
			TokenKind::Plus => Some(BinExprOp::Add),
			TokenKind::Minus => Some(BinExprOp::Sub),
			TokenKind::Asterisk => Some(BinExprOp::Mul),
			TokenKind::ForwardSlash => Some(BinExprOp::Div),

			_ => return None
		}
	}

}