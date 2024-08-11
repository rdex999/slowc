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
	pub parameters_stack_size: usize,
	pub locals: Vec<Variable>,
	pub code_block: Scope,
}

#[derive(Debug, Clone)]
pub struct Scope
{
	pub statements: Vec<Statement>,
	pub stack_size: usize,
	// Will have more crap in the near future.
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement
{
	Scope(Scope),
	If(IfInfo),
	Assign(VarUpdateInfo),
	FunctionCall(FunctionCallInfo),
	Return(Option<BinExpr>),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IfInfo
{
	condition: BinExpr,
	then_block: Box<Statement>,
	else_block: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct VarUpdateInfo
{
	pub destination: Value,
	pub value: BinExpr
}

#[derive(Debug, Clone)]
pub struct FunctionCallInfo
{
	pub index: u8,
	pub arguments: Vec<BinExpr>,
}

#[derive(Debug, Clone)]
pub enum Value
{
	I8(i8),			/* (Not funny) */
	U8(u8),			/* (Not funny) */
	I16(i16),		/* (Not funny) */
	U16(u16),		/* (Not funny) */
	I32(i32),		/* (Not funny) */
	U32(u32),		/* (Not funny) */
	I64(i64),		/* (Not funny) */
	U64(u64),		/* (Not funny) */
	F32(f32),		/* (Not funny) */
	F64(f64),		/* (Not funny) */
	Var(u8),		/* The variables index in the variables array */
	FuncCall(FunctionCallInfo),
}

#[derive(Debug, Clone)]
pub struct BinExpr
{
	pub root: BinExprPart,
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

	BoolEq,
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

	F32,
	F64,
}

#[derive(Debug, Clone, Copy)]
pub struct Variable
{
	pub data_type: Type,
	pub attributes: AttributeType,
	pub index: u8,
	pub location: isize,
	pub scope: u8,
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
	pub fn new(identifier: String, return_type: Type, attributes: AttributeType) -> Self
	{
		return Self{
			identifier,
			index: u8::MAX,
			return_type,
			attributes,
			parameters_stack_size: 0,
			locals: Vec::new(),
			parameter_count: 0,
			code_block: Scope::new(Vec::new())
		};
	}
	
}

impl Scope
{
	pub fn new(statements: Vec<Statement>) -> Self
	{
		return Self {
			statements,
			stack_size: 0,
		};
	}

	pub fn add_statement(&mut self, statement: Statement)
	{
		self.statements.push(statement);
	}
}

#[allow(dead_code)]
impl IfInfo
{
	pub fn new(condition: BinExpr, then_block: Box<Statement>, else_block: Option<Box<Statement>>) -> Self
	{
		return Self {
			condition,
			then_block,
			else_block,
		};
	}
}

impl FunctionCallInfo
{
	pub fn new(index: u8, arguments: Vec<BinExpr>) -> Self
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
			root,
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
	pub fn new(data_type: Type, attributes: AttributeType, index: u8, scope: u8) -> Self 
	{
		return Self {
			data_type,
			attributes,
			index,
			location: 0,		/* Doesnt realy matter */
			scope,
		};
	}
}

impl VarUpdateInfo
{
	pub fn new(destination: Value, value: BinExpr) -> Self
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
			Type::I8  | Type::U8  | Type::I16 | Type::U16 | 
			Type::I32 | Type::U32 | Type::I64 | Type::U64 |
			Type::F32 | Type::F64 => true,
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
			TokenKind::F32 	=> return Some(Type::F32),
			TokenKind::F64 	=> return Some(Type::F64),
			_ 				=> return None
		};
	}

	pub fn size(&self) -> u8
	{
		return match self
		{
			Type::Void 							=> 0,
			Type::I8  | Type::U8 				=> 1,
			Type::I16 | Type::U16				=> 2,
			Type::I32 | Type::U32 | Type::F32	=> 4,
			Type::I64 | Type::U64 | Type::F64 	=> 8,
		}
	}

	pub fn is_signed(&self) -> bool
	{
		return match self
		{
			Type::I8  | Type::I16 | Type::I32 | Type::I64 |
			Type::F32 | Type::F64 => true,
			_ => false,
		}
	}

	pub fn is_integer(&self) -> bool
	{
		return *self as u8 >= Type::I8 as u8 && *self as u8 <= Type::U64 as u8;
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
	// const LOWEST_PRECEDENCE: u8 = 1;
	// const HIGHEST_PRECEDENCE: u8 = 3;
	
	pub fn from_token_kind(token_kind: &TokenKind) -> Option<Self>
	{
		return Some(match token_kind
		{
			TokenKind::Plus 		=> BinExprOperator::Add,
			TokenKind::Minus 		=> BinExprOperator::Sub,
			TokenKind::Asterisk 	=> BinExprOperator::Mul,
			TokenKind::ForwardSlash => BinExprOperator::Div,

			TokenKind::BoolEq 		=> BinExprOperator::BoolEq,
			_ => return None
		});
	}
	
	pub fn precedence(&self) -> u8
	{
		return match *self
		{
			BinExprOperator::BoolEq => 1,
			BinExprOperator::Add | BinExprOperator::Sub => 2,
			BinExprOperator::Mul | BinExprOperator::Div => 3,
		};
	}

	pub fn is_boolean(&self) -> bool
	{
		return *self as u8 >= BinExprOperator::BoolEq as u8;
	}
}

impl Value
{
	// pub fn is_constant(&self) -> bool
	// {
	// 	return match *self
	// 	{
	// 		Value::Var(_) | Value::FuncCall(_) => false,
	// 		_ => true,
	// 	};
	// }
}

pub mod attribute
{
	pub type AttributeType = u16;
	use super::*;
	
	pub const GLOBAL: 				AttributeType = 0b1 << 0;
	pub const EXTERN: 				AttributeType = 0b1 << 1;
	pub const SYS_V_ABI_X86_64: 	AttributeType = 0b1 << 2;
	pub const FUNCTION_PARAMETER: 	AttributeType = 0b1 << 3;
	
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
