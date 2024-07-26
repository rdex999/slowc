use super::{ast::*, CompileError, print_err};

const OUT_OBJECT_FILE_PATH: &str = "/tmp/slowc_compiled.obj";
const OUT_ASM_FILE_PATH: &str = "/tmp/slowc_compiled.asm";

pub struct CodeGen
{
	ir: Root,
	out_asm_file: std::fs::File
}

impl CodeGen
{
	pub fn new(ir: Root) -> Self
	{
		let out_asm_file = std::fs::File::create(OUT_ASM_FILE_PATH).unwrap_or_else(|err| {
			/* Using a Usage error because the compiler was probably used on a none-linux system. */
			print_err!(CompileError::Usage, "Could not create temporary assembly file at \"{OUT_ASM_FILE_PATH}\". {err}");	
		});
		return Self {
			ir,
			out_asm_file
		};
	}
	
	pub fn generate(mut self)
	{
	}
}