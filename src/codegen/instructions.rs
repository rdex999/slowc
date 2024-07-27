use super::*;

#[derive(Debug)]
pub enum OpSize
{
	Byte,
	Word,
	Dword,
	Qword,
}

pub enum Destination
{
	Reg(Register),
	Location(LocationExpr)
}

pub enum Source
{
	Reg(Register),
	Location(LocationExpr),
	Constant(i64),
}

// Check out in the future: https://doc.rust-lang.org/std/mem/fn.variant_count.html
// For getting the amount of values in an enum
#[derive(Debug, Clone, Copy)]
pub enum Register
{
	RAX, EAX, AX, AL, AH,
	RBX, EBX, BX, BL, BH,
	RCX, ECX, CX, CL, CH,
	RDX, EDX, DX, DL, DH,

	RSI, ESI, SI, SIL,
	RDI, EDI, DI, DIL,
	RSP, ESP, SP, SPL,
	RBP, EBP, BP, BPL,

	R8, R8D, R8W, R8B,
	R9, R9D, R9W, R9B,
	R10, R10D, R10W, R10B,
	R11, R11D, R11W, R11B,
	R12, R12D, R12W, R12B,
	R13, R13D, R13W, R13B,
	R14, R14D, R14W, R14B,
	R15, R15D, R15W, R15B,
}

pub struct LocationExpr
{
	base: Register,
	base_multiplier: Option<usize>,
	offset: isize,
}

impl std::fmt::Display for Destination
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		let _ = match self
		{
			Destination::Reg(register) => write!(f, "{register}"),
			Destination::Location(_) => todo!(),
		};
		return Ok(());
	}
}

impl std::fmt::Display for Source
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		match self {
			Source::Reg(register) => write!(f, "{register}"),
			Source::Constant(value) => write!(f, "{value}"),
			Source::Location(location)	=> write!(f, "{location}"),
		}
	}
}

impl std::fmt::Display for Register
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		let _ = write!(f, "{}", format!("{:?}", self).to_lowercase());
		return Ok(());
	
	}
}

impl std::fmt::Display for OpSize
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		let _ = write!(f, "{}", format!("{:?}", self).to_lowercase());
		return Ok(());
	}
}

impl std::fmt::Display for LocationExpr
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		if let Some(multiplier) = self.base_multiplier
		{
			let _ = write!(f, "[{} * {multiplier} + {}]", self.base, self.offset);
		} else
		{
			let _ = write!(f, "[{} + {}]", self.base, self.offset);
		}

		return Ok(());
	}
}

impl LocationExpr
{
	pub fn new(base: Register, base_multiplier: Option<usize>, offset: isize) -> Self
	{
		return Self {
			base,
			base_multiplier,
			offset,
		};
	}
}

impl Register
{
	pub const COUNT: u8 = 68;
	pub const COUNT_FULL: u8 = 16;

	// The size of the register in bytes
	pub fn size(&self) -> u16
	{
		match self {
			Register::RAX | Register::RBX | Register::RCX | Register::RDX |
			Register::RSI | Register::RDI | Register::RSP | Register::RBP |
			Register::R8  | Register::R9  | Register::R10 | Register::R11 |
			Register::R12 | Register::R13 | Register::R14 | Register::R15 =>
			{
				return 8;
			},

			Register::EAX | Register::EBX | Register::ECX | Register::EDX |
			Register::ESI | Register::EDI | Register::ESP | Register::EBP |
			Register::R8D  | Register::R9D  | Register::R10D | Register::R11D |
			Register::R12D | Register::R13D | Register::R14D | Register::R15D =>
			{
				return 4;
			},

			Register::AX | Register::BX | Register::CX | Register::DX |
			Register::SI | Register::DI | Register::SP | Register::BP |
			Register::R8W  | Register::R9W  | Register::R10W | Register::R11W |
			Register::R12W | Register::R13W | Register::R14W | Register::R15W =>
			{
				return 2;
			},
			_ => return 1,
		}
	}
}

impl TryFrom<u8> for Register
{
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if value >= Register::COUNT
		{
			return Err(());
		}

		return unsafe { Ok(std::mem::transmute(value)) };
	}
}

impl<'a> CodeGen<'a>
{
	pub fn instr_add_spacing(&mut self)
	{
		self.write_text_segment("\n");
	}

	pub fn instr_mov(&mut self, destination: Destination, source: Source, size: OpSize)
	{
		self.write_text_segment(&format!("\n\tmov {size} {destination}, {source}"));
	}

	pub fn instr_push(&mut self, source: Source, size: OpSize)
	{
		self.write_text_segment(&format!("\n\tpush {size} {source}"));
	}

	pub fn instr_pop(&mut self, destination: Destination, size: OpSize)
	{
		self.write_text_segment(&format!("\n\tpop {size} {destination}"));
	}

	pub fn instr_ret(&mut self)
	{
		self.write_text_segment("\n\tret");
	}
}