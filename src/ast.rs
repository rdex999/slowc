pub mod parser;

use std::{collections::HashMap, isize};
use attribute::AttributeType;

use crate::lexer::TokenKind;

#[derive(Debug)]
pub struct Root
{
	pub functions: Vec<Function>
}


// This will have a return type field, calling convenction, and other shit in the future
#[derive(Debug, Clone)]
pub struct Function
{
	pub identifier: String,
	pub index: u8,
	pub return_type: Type,
	pub attributes: AttributeType,
	pub parameter_count: u8,
	pub locals: Vec<Variable>,
	pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement
{
	Assign(VarUpdateInfo),
	FunctionCall(FunctionCallInfo),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VarUpdateInfo
{
	pub destination: Value,
	pub value: ExprType
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FunctionCallInfo
{
	pub index: u8,
	pub arguments: Vec<ExprType>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Value
{
	I32(i32),		/* (Not funny) */
	Var(u8),		/* The variables index in the variables array */
	FuncCall(FunctionCallInfo),
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
	pub operator: BinExprOperator,
	pub lhs: BinExprPart,
	pub rhs: BinExprPart,
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
	pub attributes: AttributeType,
	pub index: u8,
	pub location: isize,
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
			index: u8::MAX,
			return_type,
			attributes,
			locals: Vec::new(),
			parameter_count: 0,
			statements: Vec::new(),
		};
	}
	
	pub fn add_statement(&mut self, statement: Statement)
	{
		self.statements.push(statement);
	}
}

impl FunctionCallInfo
{
	pub fn new(index: u8, arguments: Vec<ExprType>) -> Self
	{
		return Self {
			index,
			arguments,
		};
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
	pub fn new(data_type: Type, attributes: AttributeType, index: u8) -> Self 
	{
		return Self {
			data_type,
			attributes,
			index,
			location: 0,		/* Doesnt realy matter */
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

	pub fn size(&self) -> u16
	{
		match self
		{
			Type::I32 => return 4,
		}
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
	
	pub const GLOBAL: 				AttributeType = 0b1;
	pub const EXTERN: 				AttributeType = GLOBAL << 1;
	pub const FUNCTION_PARAMETER: 	AttributeType = EXTERN << 2;
	
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
