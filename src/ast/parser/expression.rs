use std::collections::HashMap;
use crate::{ast::*, /* error::CompileError, lexer::*, print_errln */};
use super::Parser;

impl<'a> Parser<'a>
{
	pub fn parse_expression(&mut self, _data_type: Type, _variables: &HashMap<String, Variable>) -> ExprType
	{
		// Made this crap just so rust doesnt yell at me
		return ExprType::BinExprT(BinExpr::new(BinExprOp::BinAdd, BinExprPart::Val(Value::I32(5)), BinExprPart::Val(Value::I32(5))));
	}

}