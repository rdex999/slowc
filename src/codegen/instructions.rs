use super::*;

#[derive(Debug, Clone, Copy)]
pub enum OpSize
{
	Byte,
	Word,
	Dword,
	Qword,
}

#[derive(Clone, Copy)]
pub struct Placeholder
{
	pub kind: PlaceholderKind,
	pub size: OpSize,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PlaceholderKind
{
	Reg(Register),
	Location(LocationExpr),
	I32(i32),
	U64(u64),
}

// Check out in the future: https://doc.rust-lang.org/std/mem/fn.variant_count.html
// For getting the amount of values in an enum
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Clone, Copy)]
pub struct LocationExpr
{
	base: Register,
	base_multiplier: Option<usize>,
	offset: isize,
}

impl std::fmt::Display for Placeholder
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		match &self.kind {
			PlaceholderKind::Reg(register) => write!(f, "{register}"),
			PlaceholderKind::I32(value) => write!(f, "{value}"),
			PlaceholderKind::U64(value) => write!(f, "{value}"),
			PlaceholderKind::Location(location)	=> write!(f, "{location}"),
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

	pub fn from_op_size(base_register: Register, op_size: OpSize) -> Self
	{
		return Register::try_from(base_register as u8 + 4 - (op_size as u8 + 1)).unwrap();
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

impl OpSize
{
	pub fn from_size(size: u16) -> Self
	{
		match size {
			1 => return OpSize::Byte,
			2 => return OpSize::Word,
			4 => return OpSize::Dword,
			8 => return OpSize::Qword,
			_ => panic!("Dev error! OpSize::from_size() called with size that is not a power of 2."),
		}
	}

	pub fn bytes(&self) -> u8
	{
		return 2u8.pow(*self as u32);
	}
}

impl Placeholder
{
	pub fn new(kind: PlaceholderKind, size: OpSize) -> Self
	{
		return Self {
			kind,
			size,
		};
	}

	pub fn is_constant(&self) -> bool
	{
		match self.kind
		{
			PlaceholderKind::I32(_) => return true,
			_ => return false,
		}
	}
}

impl<'a> CodeGen<'a>
{
	pub fn instr_add_spacing(&mut self)
	{
		self.write_text_segment("\n");
	}

	pub fn instr_mov(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tmov {} {destination}, {source}", destination.size));
	}

	pub fn instr_push(&mut self, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tpush {} {source}", source.size));
	}

	pub fn instr_pop(&mut self, destination: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tpop {} {destination}", destination.size));
	}

	pub fn instr_ret(&mut self)
	{
		self.write_text_segment("\n\tret");
	}

	pub fn instr_add(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tadd {} {destination}, {source}", destination.size));
	}

	pub fn instr_sub(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tsub {} {destination}, {source}", destination.size));
	}

	pub fn instr_imul(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\timul {} {destination}, {source}", destination.size));
	}

	pub fn instr_idiv(&mut self, source: &Placeholder)
	{
		let rdx_allocated = Register::from_op_size(Register::RDX, source.size);
		self.reg_alloc_allocate_forced(rdx_allocated);

		let rdx_placeholder = Placeholder::new(
			PlaceholderKind::Reg(rdx_allocated), 
			source.size
		);

		self.instr_xor(&rdx_placeholder, &rdx_placeholder);

		if source.is_constant()
		{
			let source_reg = self.reg_alloc_allocate(source.size.bytes()).unwrap();
			let source_placeholder = Placeholder::new(
				PlaceholderKind::Reg(source_reg), 
				source.size
			);
			self.instr_mov(&source_placeholder, source);
			self.write_text_segment(&format!("\n\tidiv {} {}", source.size, source_placeholder));
			self.reg_alloc_free(source_reg);
		} else
		{
			self.write_text_segment(&format!("\n\tidiv {} {source}", source.size));
		}
		self.reg_alloc_free(rdx_allocated);
	}

	pub fn instr_xor(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\txor {} {destination}, {source}", destination.size));
	}

	pub fn instr_call(&mut self, identifier: &str)
	{
		self.write_text_segment(&format!("\n\tcall {identifier}"));
	}
}