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
pub enum Statement
{
	Scope(Scope),
	If(IfInfo),
	For(ForLoopInfo),
	Assign(VarUpdateInfo),
	FunctionCall(FunctionCallInfo),
	Return(Option<BinExpr>),
}

#[derive(Debug, Clone)]
pub struct IfInfo
{
	pub condition: BinExpr,
	pub then_block: Box<Statement>,
	pub else_block: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct ForLoopInfo
{
	pub initializer: Option<Box<Statement>>,
	pub condition: Option<BinExpr>,
	pub update: Option<Box<Statement>>,
	pub code_block: Box<Statement>,
	pub stack_size: usize,
}

#[derive(Debug, Clone)]
pub struct VarUpdateInfo
{
	pub destination: Value,
	pub value: BinExpr
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
	Dereference(DereferenceInfo),
	FuncCall(FunctionCallInfo),
}

#[derive(Debug, Clone)]
pub struct DereferenceInfo
{
	pub expression: Box<BinExpr>,
	pub dereference_count: u8,
	pub data_type: Type,
}

#[derive(Debug, Clone)]
pub struct FunctionCallInfo
{
	pub index: u8,
	pub arguments: Vec<BinExpr>,
}

#[derive(Debug, Clone)]
pub struct BinExpr
{
	pub root: BinExprPart,
}

#[derive(Debug, Clone)]
pub enum BinExprPart
{
	SelfOperation(Box<BinExprSelfOperation>),
	Operation(Box<BinExprOperation>),
	Val(Value),
	TypeCast(Box<TypeCastInfo>),
}

#[derive(Debug, Clone)]
pub struct TypeCastInfo
{
	pub into_type: Type,
	pub from_type: Type,
	pub expression: BinExprPart,
}

#[derive(Debug, Clone)]
pub struct BinExprOperation
{
	pub operator: BinExprOperator,
	pub lhs: BinExprPart,
	pub rhs: BinExprPart,
}

// If yall know a better name for this let me know
#[derive(Debug, Clone)]
pub struct BinExprSelfOperation
{
	pub operator: BinExprOperator,
	pub expression: BinExprPart,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BinExprOperator
{
	BoolAnd,
	BoolOr,
	BoolEq,
	BoolNotEq,
	BoolGreater,
	BoolLess,
	BoolGreaterEq,
	BoolLessEq,

	BitwiseOr,
	BitwiseXor,
	BitwiseAnd,
	BitwiseRightShift,
	BitwiseLeftShift,
	Add,
	Sub,
	Mul,
	Div,
	Modulo,
	BitwiseNot,
	BoolNot,
	AddressOf,
	Dereference,
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Type
{
	pub kind: TypeKind,
	
	// Doesnt matter if self.kind != TypeKind::Pointer
	pub points_to: TypeKind,	
	pub pointer_level: u8,		
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TypeKind
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
	Pointer,

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

impl IfInfo
{
	pub fn new(condition: BinExpr, then_block: Statement, else_block: Option<Statement>) -> Self
	{
		return Self {
			condition,
			then_block: Box::new(then_block),
			else_block: if let Some(else_block) = else_block { Some(Box::new(else_block)) } else { None },
		};
	}
}

impl ForLoopInfo
{
	pub fn new(initializer: Option<Statement>, condition: Option<BinExpr>, update: Option<Statement>, code_block: Statement, stack_size: usize) -> Self
	{
		return Self {
			initializer: if let Some(statement) = initializer { Some(Box::new(statement)) } else { None },
			condition,
			update: if let Some(statement) = update { Some(Box::new(statement)) } else { None },
			code_block: Box::new(code_block),
			stack_size
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

impl BinExprSelfOperation
{
	pub fn new(operator: BinExprOperator, expression: BinExprPart) -> Self
	{
		return Self {
			operator, 
			expression,
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

impl TypeKind
{
	// Will return None if the given token kind is not a type
	pub fn from_token_kind(token_kind: &TokenKind) -> Option<Self>
	{
		match token_kind {
			TokenKind::Void => return Some(Self::Void),
			TokenKind::I8 	=> return Some(Self::I8),
			TokenKind::U8 	=> return Some(Self::U8),
			TokenKind::I16 	=> return Some(Self::I16),
			TokenKind::U16 	=> return Some(Self::U16),
			TokenKind::I32 	=> return Some(Self::I32),
			TokenKind::U32 	=> return Some(Self::U32),
			TokenKind::I64 	=> return Some(Self::I64),
			TokenKind::U64 	=> return Some(Self::U64),
			TokenKind::F32 	=> return Some(Self::F32),
			TokenKind::F64 	=> return Some(Self::F64),
			_ 				=> return None
		};
	}

	pub fn size(&self) -> u8
	{
		return match self
		{
			Self::Void 											=> 0,
			Self::I8  | Self::U8 								=> 1,
			Self::I16 | Self::U16								=> 2,
			Self::I32 | Self::U32 | Self::F32					=> 4,
			Self::I64 | Self::U64 | Self::F64 | Self::Pointer 	=> 8,
		}
	}

	pub fn is_signed(&self) -> bool
	{
		return match self
		{
			Self::I8  | Self::I16 | Self::I32 | Self::I64 |
			Self::F32 | Self::F64 => true,
			_ => false,
		}
	}

	pub fn is_integer(&self) -> bool
	{
		return *self as u8 >= Self::I8 as u8 && *self as u8 <= Self::Pointer as u8;
	}
}

impl Type
{
	pub fn new(kind: TypeKind) -> Self
	{
		return Self {
			kind,
			points_to: TypeKind::Void,
			pointer_level: 0,
		};
	}

	pub fn new_ptr(kind: TypeKind, points_to: TypeKind, pointer_level: u8) -> Self
	{
		return Self {
			kind,
			points_to,
			pointer_level,
		};
	}

	pub fn size(&self) -> u8
	{
		return self.kind.size();
	}

	pub fn is_signed(&self) -> bool
	{
		return self.kind.is_signed();
	}

	pub fn is_integer(&self) -> bool
	{
		return self.kind.is_integer();
	}

	pub fn is_pointer(&self) -> bool
	{
		return self.kind == TypeKind::Pointer;
	}

	pub fn dereference(&self, count: u8) -> Type
	{
		if self.kind != TypeKind::Pointer || count > self.pointer_level
		{
			panic!("Type.dereference was not called on a pointer data type.");
		}

		let new_ptr_level = self.pointer_level - count;
		if new_ptr_level == 0
		{
			return Type::new(self.points_to);
		}

		return Type::new_ptr(TypeKind::Pointer, self.points_to, new_ptr_level);
	}
}

impl std::fmt::Display for Type
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		let _ = write!(f, "{}", format!("{:?}", self.kind).to_lowercase());
		return Ok(());
	}
}

impl BinExprOperator
{
	const LOWEST_PRECEDENCE: u8 = 1;
	const HIGHEST_PRECEDENCE: u8 = 9;
	
	pub fn from_token_kind(token_kind: &TokenKind, is_prev_operator: bool) -> Option<Self>
	{
		return Some(match token_kind
		{
			TokenKind::BoolAnd 				=> Self::BoolAnd,
			TokenKind::BoolOr 				=> Self::BoolOr,
			TokenKind::BoolEq 				=> Self::BoolEq,
			TokenKind::BoolNotEq 			=> Self::BoolNotEq,
			TokenKind::BoolGreater 			=> Self::BoolGreater,
			TokenKind::BoolLess 			=> Self::BoolLess,
			TokenKind::BoolGreaterEq 		=> Self::BoolGreaterEq,
			TokenKind::BoolLessEq 			=> Self::BoolLessEq,

			TokenKind::BitwiseOr 			=> Self::BitwiseOr,
			TokenKind::BitwiseXor 			=> Self::BitwiseXor,
			TokenKind::BitwiseAnd 			=> if is_prev_operator { Self::AddressOf } else { Self::BitwiseAnd },
			TokenKind::BitwiseRightShift 	=> Self::BitwiseRightShift,
			TokenKind::BitwiseLeftShift 	=> Self::BitwiseLeftShift,
			TokenKind::Plus 				=> Self::Add,
			TokenKind::Minus 				=> Self::Sub,
			TokenKind::Asterisk 			=> if is_prev_operator { Self::Dereference } else { Self::Mul },
			TokenKind::ForwardSlash 		=> Self::Div,
			TokenKind::Percent 				=> Self::Modulo,
			TokenKind::BitwiseNot			=> Self::BitwiseNot,
			TokenKind::BoolNot				=> Self::BoolNot,
			_ => return None
		});
	}
	
	pub fn precedence(&self) -> u8
	{
		return match *self
		{
			Self::BoolAnd | Self::BoolOr											=> 1,
			Self::BoolEq | Self::BoolNotEq | Self::BoolGreater | Self::BoolLess	|
			Self::BoolGreaterEq	| Self::BoolLessEq									=> 2,
			Self::BitwiseOr 														=> 3,
			Self::BitwiseXor 														=> 4,
			Self::BitwiseAnd 														=> 5,
			Self::BitwiseRightShift | Self::BitwiseLeftShift 						=> 6,
			Self::Add | Self::Sub 													=> 7,
			Self::Mul | Self::Div | Self::Modulo 									=> 8,
			Self::BitwiseNot | Self::BoolNot | Self::AddressOf | Self::Dereference	=> 9,
		};
	}

	pub fn is_boolean(&self) -> bool
	{
		return (*self as u8 >= Self::BoolAnd as u8 && *self as u8 <= Self::BoolLessEq as u8) || *self == Self::BoolNot;
	}

	pub fn is_self_operator(&self) -> bool
	{
		return *self as u8 >= Self::BitwiseNot as u8 && *self as u8 <= Self::Dereference as u8;
	}
}

// impl Value
// {
	// pub fn is_constant(&self) -> bool
	// {
	// 	return match *self
	// 	{
	// 		Value::Var(_) | Value::FuncCall(_) => false,
	// 		_ => true,
	// 	};
	// }
// }

impl TypeCastInfo
{
	pub fn new(into_type: Type, from_type: Type, expression: BinExprPart) -> Self
	{
		return Self {
			into_type,
			from_type,
			expression,
		}
	}
}

impl DereferenceInfo
{
	pub fn new(expression: BinExpr, dereference_count: u8, data_type: Type) -> Self
	{
		return Self {
			expression: Box::new(expression),
			dereference_count,
			data_type,
		};
	}
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