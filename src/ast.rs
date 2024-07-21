pub mod parser;

use std::collections::HashMap;

use crate::lexer::TokenKind;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Root
{
	pub functions: HashMap<String, Function>
}

// This will have a return type field, calling convenction, and other shit in the future
#[derive(Debug)]
#[allow(dead_code)]
pub struct Function
{
	pub stmts: Vec<Statement>,
	pub return_type: Type
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement
{
	Assign(VarUpdateInfo),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct VarUpdateInfo
{
	pub destination: Writable,
	pub value: ExprType
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Writable
{
	Var(Variable),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ExprType
{
	BinExprT(BinExpr),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BinExpr
{
	pub root: BinExprPart,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum BinExprPart
{
	Operation(Box<BinExprOperation>),
	Val(Value),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BinExprOperation
{
	operator: BinExprOperator,
	lhs: BinExprPart,
	rhs: BinExprPart,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum BinExprOperator
{
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Value
{
	I32(i32),		/* (Not funny) */
	Var(Variable),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum Type
{
	I32,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Variable
{
	data_type: Type,
}

impl Root
{
#[allow(dead_code)]
	pub fn new(functions: HashMap<String, Function>) -> Self
	{
		return Self{
			functions,
		};
	}
}

impl Function
{
#[allow(dead_code)]
	pub fn new(stmts: Vec<Statement>, return_type: Type) -> Self
	{
		return Self{
			stmts,
			return_type
		};
	}
}

impl BinExpr
{
#[allow(dead_code)]
	pub fn new(root: BinExprPart) -> Self
	{
		return Self {
			root
		};
	}
}

impl BinExprOperation
{
	pub fn new(operator: BinExprOperator, lhs: BinExprPart, rhs: BinExprPart) -> Self
	{
		return Self {
			operator,
			lhs,
			rhs,
		};
	}
}

impl Variable
{
	pub fn new(data_type: Type) -> Self 
	{
		return Self {
			data_type,
		};
	}
}

impl VarUpdateInfo
{
	pub fn new(destination: Writable, value: ExprType) -> Self
	{
		return Self {
			destination,
			value
		};
	}
}

impl Type
{
	pub fn is_bin_expr_type(&self) -> bool
	{
		return *self == Type::I32;
	}

	// Will return None if the given token kind is not a type
	pub fn from_token_kind(token_kind: &TokenKind) -> Option<Type>
	{
		match token_kind {
			TokenKind::I32 => return Some(Type::I32),
			_ => return None
		};
	}
}

impl BinExprOperator
{
	pub fn from_token_kind(token_kind: &TokenKind) -> Option<Self>
	{
		match token_kind
		{
			TokenKind::Plus => Some(BinExprOperator::Add),
			TokenKind::Minus => Some(BinExprOperator::Sub),
			TokenKind::Asterisk => Some(BinExprOperator::Mul),
			TokenKind::ForwardSlash => Some(BinExprOperator::Div),

			_ => return None
		}
	}
	
	pub fn _precedence(&self) -> u8
	{
		match *self
		{
			BinExprOperator::Add | BinExprOperator::Sub => return 1,
			BinExprOperator::Mul | BinExprOperator::Div => return 2,
		}
	}
}