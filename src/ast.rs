pub mod parser;

use std::collections::HashMap;

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
	pub operation: BinExprOp,
	pub lhs: BinExprPart,
	pub rhs: BinExprPart
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum BinExprOp
{
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum BinExprPart
{
	Expr(Box<BinExpr>),
	Val(Value),
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
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
	pub fn new(operation: BinExprOp, lhs: BinExprPart, rhs: BinExprPart) -> Self
	{
		return Self{
			operation,
			lhs,
			rhs
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