mod common;
mod instructions;
use instructions::*;

use super::{ast::*, CompileError, print_err};

const _OUT_OBJECT_FILE_PATH: &str = "/tmp/slowc_compiled.obj";
const OUT_ASM_FILE_PATH: &str = "/tmp/slowc_compiled.asm";

pub struct CodeGen<'a>
{
	ir: &'a Root,
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

		self.instr_push(Source::Reg(Register::RBP), OpSize::Qword);
		self.instr_mov(Destination::Reg(Register::RBP), Source::Reg(Register::RSP), OpSize::Qword);
		self.instr_add_spacing();

		self.gen_code_block(&function.statements, &function.locals);
	
		self.gen_function_return();
	}

	fn gen_function_return(&mut self)
	{
		self.instr_mov(Destination::Reg(Register::RSP), Source::Reg(Register::RBP), OpSize::Qword);
		self.instr_pop(Destination::Reg(Register::RBP), OpSize::Qword);
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

	fn gen_expression(&mut self, expression: &ExprType, locals: &Vec<Variable>)
	{
		match expression {
			ExprType::BinExprT(bin_expr) => self.gen_bin_expr(bin_expr, locals),
		}
	}

	fn gen_bin_expr(&mut self, bin_expr: &BinExpr, locals: &Vec<Variable>)
	{

	}
}