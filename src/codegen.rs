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
		self.instr_mov(Destination::Reg(Register::new(RegKind::RAX, RegSize::L64)), Source::Constant(1234), OpSize::Qword);
		self.instr_mov(Destination::Reg(Register::new(RegKind::RAX, RegSize::L32)), Source::Constant(1234), OpSize::Dword);
		self.instr_mov(Destination::Reg(Register::new(RegKind::RAX, RegSize::L16)), Source::Constant(1234), OpSize::Word);
		self.instr_mov(Destination::Reg(Register::new(RegKind::RAX, RegSize::L8)), Source::Constant(1234), OpSize::Byte);
		self.instr_mov(Destination::Reg(Register::new(RegKind::RAX, RegSize::H8)), Source::Constant(69), OpSize::Byte);
		// self.gen_code_block(&function.statements);
	}

}