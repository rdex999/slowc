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
	pub stack_size: usize,
	pub parameters_stack_size: usize,
	pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement
{
	Assign(VarUpdateInfo),
	FunctionCall(FunctionCallInfo),
	Return(Option<ExprType>),
}

#[derive(Debug, Clone)]
pub struct VarUpdateInfo
{
	pub destination: Value,
	pub value: ExprType
}

#[derive(Debug, Clone)]
pub struct FunctionCallInfo
{
	pub index: u8,
	pub arguments: Vec<ExprType>,
}

#[derive(Debug, Clone)]
pub enum Value
{
	I8(i8),		/* (Not funny) */
	U8(u8),		/* (Not funny) */
	I16(i16),		/* (Not funny) */
	U16(u16),		/* (Not funny) */
	I32(i32),		/* (Not funny) */
	U32(u32),		/* (Not funny) */
	I64(i64),		/* (Not funny) */
	U64(u64),		/* (Not funny) */
	Var(u8),		/* The variables index in the variables array */
	FuncCall(FunctionCallInfo),
}

#[derive(Debug, Clone)]
pub enum ExprType
{
	BinExprT(BinExpr),
}

#[derive(Debug, Clone)]
pub struct BinExpr
{
	pub root: BinExprPart,
	pub signed: bool
}

#[derive(Debug, Clone)]
pub enum BinExprPart
{
	Operation(Box<BinExprOperation>),
	Val(Value),
}

#[derive(Debug, Clone)]
pub struct BinExprOperation
{
	pub operator: BinExprOperator,
	pub lhs: BinExprPart,
	pub rhs: BinExprPart,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BinExprOperator
{
	Add,
	Sub,
	Mul,
	Div,
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Type
{
	Void,
	I8,
	U8,
	I16,
	U16,
	I32,
	U32,
	I64,
	U64,
}

#[derive(Debug, Clone, Copy)]
pub struct Variable
{
	pub data_type: Type,
	pub attributes: AttributeType,
	pub index: u8,
	pub location: isize,
}

impl Root
{
	pub fn new(functions: Vec<Function>) -> Self
	{
		return Self{
			functions,
		};
	}
}

impl Function
{
	pub fn new(identifier: String, return_type: Type, attributes: AttributeType, parameters_stack_size: usize) -> Self
	{
		return Self{
			identifier,
			index: u8::MAX,
			return_type,
			attributes,
			locals: Vec::new(),
			stack_size: 0,
			parameters_stack_size,
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
	pub fn new(root: BinExprPart, signed: bool) -> Self
	{
		return Self {
			root,
			signed
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
		return match self
		{
			Type::I8  | Type::U8 | Type::I16 | Type::U16 | 
			Type::I32 | Type::U32 | Type::I64 | Type::U64 => true,
			_ => false,
		}
	}
	
	// Will return None if the given token kind is not a type
	pub fn from_token_kind(token_kind: &TokenKind) -> Option<Type>
	{
		match token_kind {
			TokenKind::Void => return Some(Type::Void),
			TokenKind::I8 	=> return Some(Type::I8),
			TokenKind::U8 	=> return Some(Type::U8),
			TokenKind::I16 	=> return Some(Type::I16),
			TokenKind::U16 	=> return Some(Type::U16),
			TokenKind::I32 	=> return Some(Type::I32),
			TokenKind::U32 	=> return Some(Type::U32),
			TokenKind::I64 	=> return Some(Type::I64),
			TokenKind::U64 	=> return Some(Type::U64),
			_ 				=> return None
		};
	}

	pub fn size(&self) -> u8
	{
		return match self
		{
			Type::Void 				=> 0,
			Type::I8  | Type::U8 	=> 1,
			Type::I16 | Type::U16	=> 2,
			Type::I32 | Type::U32	=> 4,
			Type::I64 | Type::U64	=> 8,
		}
	}

	pub fn is_signed(&self) -> bool
	{
		return match self
		{
			Type::I8 | Type::I16 | Type::I32 | Type::I64 => true,
			_ => false,
		}
	}
}

impl std::fmt::Display for Type
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		let _ = write!(f, "{}", format!("{:?}", self).to_lowercase());
		return Ok(());
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
	pub const CDECL: 				AttributeType = EXTERN << 2;
	pub const FUNCTION_PARAMETER: 	AttributeType = CDECL << 3;
	
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
