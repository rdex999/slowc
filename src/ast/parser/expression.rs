use parser::TokenKind;

use crate::{ast::*, error::CompileError, print_errln };
use super::{Parser, variable::*, };

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, data_type: Type, variables: &LocalVariables) -> ExprType
	{
		if Self::is_bin_expr_type(&data_type)
		{
			return ExprType::BinExprT(self.parse_bin_expression(data_type, variables));
		} else
		{
			todo!("Dev error!! parse_expression() - None binary expressions are not yet supported.");
		}

	}

	fn parse_bin_expression(&mut self, data_type: Type, variables: &LocalVariables) -> BinExpr
	{
		let lhs = self.parse_bin_expression_part(data_type, variables);
		let token_op = self.current_token();
		let operation = Self::kind_2_bin_op(&token_op.kind).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, token_op.span.start, "None binary operator found in binary expression.");
		});

		self.advance_token();
		let rhs = self.parse_bin_expression_part(data_type, variables);
		return BinExpr::new(operation, lhs, rhs);
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