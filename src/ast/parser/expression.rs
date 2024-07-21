use parser::TokenKind;

use crate::{ast::*, error::CompileError, print_errln };
use super::{Parser, variable::*, };

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, data_type: Type, variables: &LocalVariables) -> ExprType
	{
		if data_type.is_bin_expr_type()
		{
			return ExprType::BinExprT(self.parse_bin_expression(data_type, variables));
		} else
		{
			todo!("Dev error!! parse_expression() - None binary expressions are not yet supported.");
		}
	}

	fn parse_bin_expression(&mut self, data_type: Type, variables: &LocalVariables) -> BinExpr
	{
		let expression_root = self.parse_bin_expression_part(data_type, variables);
		return BinExpr::new(expression_root);
	}

	fn parse_bin_expression_part(&mut self, data_type: Type, variables: &LocalVariables) -> BinExprPart
	{
		let start_span = self.current_token().span;	
		let lhs = self.parse_expression_item(data_type, variables).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, start_span.start, "None binary token found in binary expression.");
		});	

		if let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
		{
			self.parse_bin_operator();
			let next_part = self.parse_bin_expression_part(data_type, variables);
			return BinExprPart::Operation(Box::new(BinExprOperation::new(operator, BinExprPart::Val(lhs), next_part)));
		} else
		{
			return BinExprPart::Val(lhs);
		}
	}

	fn parse_bin_operator(&mut self) -> BinExprOperator
	{
		let token = self.current_token();
		self.advance_token();
		if let Some(operator) = BinExprOperator::from_token_kind(&token.kind)
		{
			return operator;
		} 
		print_errln!(CompileError::Syntax, self.source, token.span.start, "None binary operator found in binary expression.");
	}
	
	fn parse_expression_item(&mut self, _data_type: Type, _variables: &LocalVariables) -> Option<Value>
	{
		let token = self.current_token();
		self.advance_token();
		match token.kind {
			TokenKind::IntLit(value) => return Some(Value::I32(value as i32)),

			_ => return None
		}
	}

}