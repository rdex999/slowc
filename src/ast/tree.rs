pub struct Root
{
	functions: Vec<Function>,
}

// This will have a return type field, calling convenction, and other shit in the future
pub struct Function
{
	stmts: Vec<Statement>,
}

pub enum Statement
{
	VarDecl(VarDeclInfo),
}

pub struct VarDeclInfo
{
	identifier: String,	
	expr_type: ExprType,
}

pub enum ExprType
{
	BinExprT(BinExpr),
}

pub struct BinExpr
{
	operation: BinExprOp,
	lhs: BinExprPart,
	rhs: BinExprPart
}

pub enum BinExprOp
{
	BinAdd,
	BinSub,
	BinMul,
	BinDiv,
}

pub enum BinExprPart
{
	Expr(Box<BinExpr>),
	Val(Value),
}

pub enum Value
{
	I32(i32),		/* (Not funny) */

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