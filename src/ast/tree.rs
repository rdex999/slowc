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
	pub stmts: Vec<Statement>
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement
{
	VarDecl(VarDeclInfo),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct VarDeclInfo
{
	pub identifier: String,	
	pub expr_type: ExprType,
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
	BinAdd,
	BinSub,
	BinMul,
	BinDiv,
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
	pub fn new(stmts: Vec<Statement>) -> Self
	{
		return Self{
			stmts,
		};
	}
}

impl VarDeclInfo
{
#[allow(dead_code)]
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