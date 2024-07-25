pub mod parser;

use std::collections::HashMap;
use attribute::AttributeType;

use crate::lexer::TokenKind;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Root
{
	pub functions: Vec<Function>
}


// This will have a return type field, calling convenction, and other shit in the future
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function
{
	pub identifier: String,
	pub index: usize,
	pub return_type: Type,
	pub attributes: AttributeType,
	pub locals: Vec<Variable>,
	pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement
{
	Assign(VarUpdateInfo),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VarUpdateInfo
{
	pub destination: Value,
	pub value: ExprType
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Value
{
	I32(i32),		/* (Not funny) */
	Var(usize),		/* The variables index in the variables array */
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ExprType
{
	BinExprT(BinExpr),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BinExpr
{
	pub root: BinExprPart,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BinExprPart
{
	Operation(Box<BinExprOperation>),
	Val(Value),
}

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum Type
{
	I32,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
// TODO: add attributes, which will include is_function_parameter
pub struct Variable
{
	pub data_type: Type,
	pub index: usize,
}

impl Root
{
	#[allow(dead_code)]
	pub fn new(functions: Vec<Function>) -> Self
	{
		return Self{
			functions,
		};
	}
}

impl Function
{
	pub fn new(identifier: String, return_type: Type, attributes: AttributeType) -> Self
	{
		return Self{
			identifier,
			index: usize::MAX,
			return_type,
			attributes,
			locals: Vec::new(),
			stmts: Vec::new(),
		};
	}
	
	pub fn add_statement(&mut self, statement: Statement)
	{
		self.stmts.push(statement);
	}
}

impl BinExpr
{
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
	pub fn new(data_type: Type, index: usize) -> Self 
	{
		return Self {
			data_type,
			index
		};
	}
}

impl VarUpdateInfo
{
	pub fn new(destination: Value, value: ExprType) -> Self
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
	const LOWEST_PRECEDENCE: u8 = 1;
	
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
	
	pub fn precedence(&self) -> u8
	{
		match *self
		{
			BinExprOperator::Add | BinExprOperator::Sub => return 1,
			BinExprOperator::Mul | BinExprOperator::Div => return 2,
		}
	}
}

pub mod attribute
{
	pub type AttributeType = u16;
	use super::*;
	
	const GLOBAL: AttributeType = 0b1;
	const EXTERN: AttributeType = GLOBAL << 1;
	
	pub fn from_token_kind(token_kind: &TokenKind) -> Option<AttributeType>
	{
		match token_kind
		{
			TokenKind::Global => Some(GLOBAL),
			TokenKind::Extern => Some(EXTERN),
			_ => return None
		}
	}
}
