mod common;

use super::{ast::*, CompileError, print_err};

const _OUT_OBJECT_FILE_PATH: &str = "/tmp/slowc_compiled.obj";
const OUT_ASM_FILE_PATH: &str = "/tmp/slowc_compiled.asm";

pub struct CodeGen<'a>
{
	ir: &'a Root,
	attribute_segment: String,
	data_segment: String,
}

impl<'a> CodeGen<'a>
{
	pub fn new(ir: &'a Root) -> Self
	{
		let data_segment = String::from("segment .data");

		return Self {
			ir,
			attribute_segment: String::new(),
			data_segment,
		};
	}
	
	pub fn generate(mut self)
	{
		for function in &self.ir.functions
		{
			self.gen_function(&function);
		}

		let mut final_asm = String::with_capacity(self.attribute_segment.len() + self.data_segment.len() + 1);
		final_asm.push_str(&self.attribute_segment);
		final_asm.push_str(&self.data_segment);
		final_asm.push('\n');

		std::fs::write(OUT_ASM_FILE_PATH, final_asm).unwrap_or_else(|err| {
			print_err!(CompileError::FileWriteError(OUT_ASM_FILE_PATH), "Could not write to temporary assembly file. {err}");
		});
	}

	fn gen_function(&mut self, function: &Function)
	{
		self.decl_attribute(&function.identifier, function.attributes);
	}
}