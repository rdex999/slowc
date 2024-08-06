mod common;
mod instructions;
mod register_allocator;
mod expression;
mod function;

use instructions::*;
use register_allocator::*;
use super::{ast::*, CompileError, print_err};

const OUT_OBJECT_FILE_PATH: &str = "/tmp/slowc_compiled.obj";
const OUT_ASM_FILE_PATH: &str = "/tmp/slowc_compiled.asm";

pub struct CodeGen<'a>
{
	ir: &'a Root,
	registers: [RegisterInfo; ALLOCATABLE_REGS_COUNT],
	attribute_segment: String,
	data_segment: String,
	text_segment: String,

	data_seg_var_index: usize
}

impl<'a> CodeGen<'a>
{

	pub fn new(ir: &'a Root) -> Self
	{
		let attribute_segment = String::from("bits 64");
		let data_segment = String::from("\nsegment .data");
		let text_segment = String::from("\nsegment .text");

		return Self {
			ir,
			registers: Self::reg_alloc_init(),
			attribute_segment,
			data_segment,
			text_segment,
			data_seg_var_index: 0
		};
	}
	
	pub fn generate<'b>(mut self) -> &'b str
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

		std::process::Command::new("nasm")
			.args(["-f", "elf64"])
			.arg("-g")
			.args(["-o", OUT_OBJECT_FILE_PATH])
			.arg(OUT_ASM_FILE_PATH)
			.spawn()
			.expect("Dev error! failed to execute nasm.");

		return &OUT_OBJECT_FILE_PATH;
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
			Statement::Assign(assign_data) 					=> self.gen_assign_stmt(assign_data, locals),
			Statement::FunctionCall(function_call_info) 	=> { self.gen_function_call(locals, function_call_info); } 
			Statement::Return(expression) 				=> self.gen_return_stmt(locals, expression),
		}
	}

	fn gen_assign_stmt(&mut self, assign_data: &VarUpdateInfo, locals: &Vec<Variable>)
	{
		let source = self.gen_expression(&assign_data.value, locals);
		let	src_reg = self.reg_alloc_allocate(source.data_type).unwrap();
		let src_placeholder = Placeholder::new(
			PlaceholderKind::Reg(src_reg), 
			source.data_type
		);
		self.instr_mov(&src_placeholder, &source);

		let destination = self.gen_value_access(locals, &assign_data.destination);

		self.instr_mov(&destination, &src_placeholder);

		self.reg_alloc_free(src_reg);
	}

	fn gen_return_stmt(&mut self, locals: &Vec<Variable>, expression: &Option<ExprType>)
	{
		let expr;
		if let Some(exp) = expression
		{
			expr = exp;
		} else
		{
			self.gen_function_return();
			return;
		}

		let expr_placeholder = self.gen_expression(expr, locals);
		let rax = Register::from_op_size(Register::RAX, expr_placeholder.data_type.size());

		// I hate Rust
		if let PlaceholderKind::Reg(reg) = expr_placeholder.kind
		{
			if reg != rax
			{
				self.instr_mov(
					&Placeholder::new(PlaceholderKind::Reg(rax), expr_placeholder.data_type), 
					&expr_placeholder
				);
			}
		} else 
		{
			self.instr_mov(
				&Placeholder::new(PlaceholderKind::Reg(rax), expr_placeholder.data_type), 
				&expr_placeholder
			);
		}

		self.gen_function_return();
	}
}