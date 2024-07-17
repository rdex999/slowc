use std::collections::HashMap;

#[derive(Debug)]
pub struct Root
{
	pub functions: HashMap<String, Function>
}

// This will have a return type field, calling convenction, and other shit in the future
#[derive(Debug)]
pub struct Function
{
	pub stmts: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement
{
	VarDecl(VarDeclInfo),
}

#[derive(Debug)]
pub struct VarDeclInfo
{
	pub identifier: String,	
	pub expr_type: ExprType,
}

#[derive(Debug)]
pub enum ExprType
{
	BinExprT(BinExpr),
}

#[derive(Debug)]
pub struct BinExpr
{
	pub operation: BinExprOp,
	pub lhs: BinExprPart,
	pub rhs: BinExprPart
}

#[derive(Debug)]
pub enum BinExprOp
{
	BinAdd,
	BinSub,
	BinMul,
	BinDiv,
}

#[derive(Debug)]
pub enum BinExprPart
{
	Expr(Box<BinExpr>),
	Val(Value),
}

#[derive(Debug)]
pub enum Value
{
	I32(i32),		/* (Not funny) */

}

impl Root
{
	pub fn new(functions: HashMap<String, Function>) -> Self
	{
		return Self{
			functions,
		};
	}
}

impl Function
{
	pub fn new(stmts: Vec<Statement>) -> Self
	{
		return Self{
			stmts,
		};
	}
}

impl VarDeclInfo
{
	pub fn new(identifier: String, expr_type: ExprType) -> Self
	{
		return Self{
			identifier,
			expr_type,
		};
	}
}

impl BinExpr
{
	pub fn new(operation: BinExprOp, lhs: BinExprPart, rhs: BinExprPart) -> Self
	{
		return Self{
			operation,
			lhs,
			rhs
		};
	}
}