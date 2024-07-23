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
		let expression_root = self.parse_bin_expression_part(data_type, variables, BinExprOperator::LOWEST_PRECEDENCE);
		return BinExpr::new(expression_root);
	}

	fn parse_bin_expression_part(&mut self, data_type: Type, variables: &LocalVariables, precedence: u8) -> BinExprPart
	{
		let start_span = self.current_token().span;	
		let lhs = self.parse_rvalue(data_type, variables).unwrap_or_else(|| {
			print_errln!(CompileError::Syntax, self.source, start_span.start, "None binary token found in binary expression.");
		});	

		if let Some(operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
		{
			if operator.precedence() >= precedence
			{
				self.parse_bin_operator();
				// Ik recursion is bad, but it an advantage is this situation - 
				// The same could be done with a stack data structure, (Vec) but this would require heap allcating memory which is slow af
				// In this way, everything is on the stack, the because its just an expression the recursion is not big, so the stack wont commit suicide
				let rhs = self.parse_bin_expression_part(data_type, variables, operator.precedence());

				if let Some(next_operator) = BinExprOperator::from_token_kind(&self.current_token().kind)
				{
					if next_operator.precedence() >= precedence
					{
						self.parse_bin_operator();
						let low_precedence_part = self.parse_bin_expression_part(data_type, variables, next_operator.precedence());
						// Makes me think about my lifes deceisions
						return BinExprPart::Operation(Box::new(BinExprOperation::new(
							next_operator,
							BinExprPart::Operation(Box::new(BinExprOperation::new(operator, BinExprPart::Val(lhs), rhs))),
							low_precedence_part
						)));

					}
				}
				return BinExprPart::Operation(Box::new(BinExprOperation::new(operator, BinExprPart::Val(lhs), rhs)));
			} else
			{
				return BinExprPart::Val(lhs);
			}
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
	
	fn parse_rvalue(&mut self, data_type: Type, variables: &LocalVariables) -> Option<Rvalue>
	{
		let token = self.current_token();
		self.advance_token();
		match token.kind {
			TokenKind::IntLit(value) => return Some(Rvalue::I32(value as i32)),
			TokenKind::Ident => 
			{
				let ident = &self.get_text(&token.span);
				if let Some(var) = variables.get_variable(ident)
				{
					if var.data_type != data_type
					{
						print_errln!(CompileError::TypeError(data_type, var.data_type), self.source, token.span.start, "When parsing variable.");
					}
					return Some(Rvalue::Var(var.index));
				} else
				{
					print_errln!(CompileError::UnknownIdentifier(ident), self.source, token.span.start, "No such variable.");
				}
			}
			_ => return None
		}
	}

	pub fn parse_lvalue(&mut self, variables: &LocalVariables) -> Lvalue
	{
		if self.current_token().kind != TokenKind::Ident
		{
			print_errln!(CompileError::Syntax, self.source, self.current_token().span.start, "Expected modifiable lvalue.");
		}

		let ident = self.get_text(&self.current_token().span);
		let var = variables.get_variable(ident).unwrap_or_else(|| {
			print_errln!(CompileError::UnknownIdentifier(ident), self.source, self.current_token().span.start, "");
		});
		self.advance_token();

		return Lvalue::Var(*var);
	}

}