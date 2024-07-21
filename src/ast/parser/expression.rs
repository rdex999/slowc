use parser::TokenKind;

use crate::{ast::*, error::CompileError, print_errln };
use super::{Parser, variable::*, };

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, data_type: Type, variables: &LocalVariables) -> ExprType
	{
		if data_type.is_bin_expr_type()
		{
			return ExprType::BinExprT(self.parse_bin_expression(data_type, variables, 0));
		} else
		{
			todo!("Dev error!! parse_expression() - None binary expressions are not yet supported.");
		}
	}

	fn parse_bin_expression(&mut self, data_type: Type, variables: &LocalVariables, _precedence: u8) -> BinExpr
	{
		let lhs = self.parse_bin_expression_part(data_type, variables);
		let operation = self.parse_bin_operator();
		let rhs = self.parse_bin_expression_part(data_type, variables);
		return BinExpr::new(operation, lhs, rhs);
	}

	fn parse_bin_operator(&mut self) -> BinExprOp
	{
		let token = self.current_token();
		self.advance_token();
		match token.kind
		{
			TokenKind::Plus => BinExprOp::Add,
			TokenKind::Minus => BinExprOp::Sub,
			TokenKind::Asterisk => BinExprOp::Mul,
			TokenKind::ForwardSlash => BinExprOp::Div,

			_ => { print_errln!(CompileError::Syntax, self.source, token.span.start, "None binary operator found in binary expression."); }
		}
	}

	fn parse_bin_expression_part(&mut self, _data_type: Type, _variables: &LocalVariables) -> BinExprPart
	{
		let token_value = self.current_token();
		self.advance_token();
		match token_value.kind {
			TokenKind::IntLit(value) => return BinExprPart::Val(Value::I32(value as i32)),

			_ => { print_errln!(CompileError::Syntax, self.source, token_value.span.start, "None binary token found in binary expression."); }
		}
	}

}