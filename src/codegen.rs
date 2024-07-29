mod common;
mod instructions;
mod register_allocator;

use instructions::*;
use register_allocator::*;
use super::{ast::*, CompileError, print_err};

const _OUT_OBJECT_FILE_PATH: &str = "/tmp/slowc_compiled.obj";
const OUT_ASM_FILE_PATH: &str = "/tmp/slowc_compiled.asm";

const ALLOCATABLE_REGS_COUNT: usize = Register::COUNT_FULL as usize - 3;

pub struct CodeGen<'a>
{
	ir: &'a Root,
	registers: [RegisterInfo; ALLOCATABLE_REGS_COUNT],
	attribute_segment: String,
	data_segment: String,
	text_segment: String,
}

impl<'a> CodeGen<'a>
{

	pub fn new(ir: &'a Root) -> Self
	{
		let data_segment = String::from("segment .data");
		let text_segment = String::from("\nsegment .text");

		return Self {
			ir,
			registers: Self::reg_alloc_init(),
			attribute_segment: String::new(),
			data_segment,
			text_segment,
		};
	}
	
	pub fn generate(mut self)
	{
		for function in &self.ir.functions
		{
			self.gen_function(&function);
		}

		if cfg!(debug_assertions)
		{
			self.reg_alloc_check_leaks();
		}

		let mut final_asm = String::with_capacity(self.attribute_segment.len() + self.data_segment.len() + self.text_segment.len() + 1);
		final_asm.push_str(&self.attribute_segment);
		final_asm.push_str(&self.data_segment);
		final_asm.push_str(&self.text_segment);
		final_asm.push('\n');

		std::fs::write(OUT_ASM_FILE_PATH, final_asm).unwrap_or_else(|err| {
			print_err!(CompileError::FileWriteError(OUT_ASM_FILE_PATH), "Could not write to temporary assembly file. {err}");
		});
	}

	fn gen_function(&mut self, function: &Function)
	{
		self.decl_attribute(&function.identifier, function.attributes);
		self.write_lable_text_seg(&function.identifier);

		self.instr_push(&Placeholder::new(PlaceholderKind::Reg(Register::RBP), OpSize::Qword));
		self.instr_mov(
			&Placeholder::new(PlaceholderKind::Reg(Register::RBP), OpSize::Qword),
			&Placeholder::new(PlaceholderKind::Reg(Register::RSP), OpSize::Qword)
		);

		self.instr_add_spacing();

		self.gen_code_block(&function.statements, &function.locals);
	
		self.gen_function_return();
	}

	fn gen_function_return(&mut self)
	{
		self.instr_mov(
			&Placeholder::new(PlaceholderKind::Reg(Register::RSP), OpSize::Qword),
			&Placeholder::new(PlaceholderKind::Reg(Register::RBP), OpSize::Qword)
		);
		self.instr_pop(&Placeholder::new(PlaceholderKind::Reg(Register::RBP), OpSize::Qword));
		self.instr_ret();
	}

	fn gen_code_block(&mut self, statements: &Vec<Statement>, locals: &Vec<Variable>)
	{
		for statement in statements
		{
			self.gen_statement(&statement, locals);
		}
	}

	fn gen_statement(&mut self, statement: &Statement, locals: &Vec<Variable>)
	{
		match statement
		{
			Statement::Assign(assign_data) => self.gen_assign_stmt(assign_data, locals),
			_ => todo!(),
		}
	}

	fn gen_assign_stmt(&mut self, assign_data: &VarUpdateInfo, locals: &Vec<Variable>)
	{
		self.gen_expression(&assign_data.value, locals);
	}

	fn gen_expression(&mut self, expression: &ExprType, locals: &Vec<Variable>) -> Placeholder
	{
		match expression {
			ExprType::BinExprT(bin_expr) => return self.gen_bin_expr(bin_expr, locals),
		}
	}

	fn gen_bin_expr(&mut self, bin_expr: &BinExpr, locals: &Vec<Variable>) -> Placeholder
	{
		match &bin_expr.root
		{
			BinExprPart::Val(value) => return self.gen_value(value, locals),
			BinExprPart::Operation(op) => return self.gen_bin_expr_recurse(op, locals, bin_expr.signed)
		}
	}

	fn gen_value(&mut self, value: &Value, _locals: &Vec<Variable>) -> Placeholder 
	{
		match value
		{
			Value::I32(number) => return Placeholder::new(PlaceholderKind::I32(*number), OpSize::Dword),
			_ => todo!(),
		}	
	}

	fn gen_bin_operation(&mut self, operator: BinExprOperator, lhs: &Placeholder, rhs: &Placeholder, signed: bool) -> Placeholder 
	{
		// TODO: Make an is_writable function in Placeholder, and check if lhs is a writable, so no need to move to RAX and stuff
		let rax = Register::from_op_size(Register::RAX, lhs.size);
		let destination = Placeholder::new(PlaceholderKind::Reg(rax), lhs.size);
		self.instr_mov(&destination, lhs);

		match operator {
			BinExprOperator::Add => self.instr_add(&destination, rhs),
			BinExprOperator::Sub => self.instr_sub(&destination, rhs),
			BinExprOperator::Mul =>
			{
				if signed
				{
					self.instr_imul(&destination, rhs);
				} else
				{
					todo!();
				}
			}
			_ => todo!(),
		}
		return Placeholder::new(PlaceholderKind::Reg(rax), lhs.size);
	}

	fn gen_bin_expr_recurse(&mut self, operation: &Box<BinExprOperation>, locals: &Vec<Variable>, signed: bool) -> Placeholder
	{
		match &operation.lhs
		{
			BinExprPart::Val(lhs) =>
			{
				match &operation.rhs
				{
					BinExprPart::Val(rhs) =>
					{
						let lhs = self.gen_value(lhs, locals);
						let rhs = self.gen_value(rhs, locals);
						return self.gen_bin_operation(operation.operator, &lhs, &rhs, signed);
					},

					BinExprPart::Operation(op) =>
					{
						let rhs = self.gen_bin_expr_recurse(&op, locals, signed);
						let register = self.reg_alloc_allocate(rhs.size.bytes()).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(register), rhs.size);
						self.instr_mov(
							&rhs_placeholder, 
							&rhs
						);
						
						let lhs = self.gen_value(lhs, locals);

						let result = self.gen_bin_operation(operation.operator, &lhs, &rhs_placeholder, signed);
						self.reg_alloc_free(register);
						return result;
					}
				}
			},

			BinExprPart::Operation(op) =>
			{
				let lhs = self.gen_bin_expr_recurse(&op, locals, signed);
				let register = self.reg_alloc_allocate(lhs.size.bytes()).unwrap();
				let lhs_placeholder = &Placeholder::new(PlaceholderKind::Reg(register), lhs.size);
				let result;
				self.instr_mov(
					&lhs_placeholder, 
					&lhs
				);

				match &operation.rhs
				{
					BinExprPart::Val(value) => 
					{
						let rhs = self.gen_value(value, locals);
						result = self.gen_bin_operation(operation.operator, &lhs_placeholder, &rhs, signed);
					},

					BinExprPart::Operation(rhs_op) => 
					{
						let rhs = self.gen_bin_expr_recurse(rhs_op, locals, signed);
						let rhs_reg = self.reg_alloc_allocate(rhs.size.bytes()).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(rhs_reg), rhs.size);
						self.instr_mov(&rhs_placeholder, &rhs);
						result = self.gen_bin_operation(operation.operator, &lhs_placeholder, &rhs_placeholder, signed);
						self.reg_alloc_free(rhs_reg);
					}
				}
				self.reg_alloc_free(register);
				return result;
			}
		}
	}
}