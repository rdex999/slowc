use super::*;

#[derive(Clone, Copy)]
pub struct Placeholder
{
	pub kind: PlaceholderKind,
	pub size: u8,		/* In bytes */
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
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
	pub fn size(&self) -> u8
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

	pub fn from_op_size(base_register: Register, op_size: u8) -> Self
	{
		return Register::try_from(base_register as u8 + 4 - (op_size.trailing_zeros() as u8 + 1)).unwrap();
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
impl Placeholder
{
	pub fn new(kind: PlaceholderKind, size: u8) -> Self
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

impl PartialEq for Placeholder
{
	fn eq(&self, other: &Self) -> bool 
	{
		match self.kind
		{
			PlaceholderKind::Reg(register) =>
			{
				if let PlaceholderKind::Reg(other_register) = other.kind
				{
					return register == other_register;
				}
			}

			PlaceholderKind::I32(value) =>
			{
				if let PlaceholderKind::I32(other_value) = other.kind
				{
					return value == other_value;
				}
			}

			PlaceholderKind::U64(value) =>
			{
				if let PlaceholderKind::U64(other_value) = other.kind
				{
					return value == other_value;
				}
			}

			PlaceholderKind::Location(location) =>
			{
				if let PlaceholderKind::Location(other_location) = other.kind
				{
					return location == other_location;
				}
			}
		} 
		return false;
	}
}

impl<'a> CodeGen<'a>
{
	fn size_2_opsize<'b>(size: u8) -> &'b str
	{
		match size
		{
			1 => return "byte",
			2 => return "word",
			4 => return "dword",
			8 => return "qword",
			_ => panic!("Dev error! size_2_opsize({size}) called with a size thats not a power of 2."),
		}
	}

	pub fn instr_add_spacing(&mut self)
	{
		self.write_text_segment("\n");
	}

	pub fn instr_mov(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		let mut source_placeholder = *source;
		if let PlaceholderKind::Location(_) = destination.kind
		{
			if let PlaceholderKind::Location(_) = source.kind
			{
				source_placeholder = Placeholder::new(
					PlaceholderKind::Reg(Register::from_op_size(Register::RAX, source.size)), 
					source.size
				);
				self.instr_mov(&source_placeholder, source);
			}
		}

		if destination == source
		{
			return;
		}

		self.write_text_segment(&format!("\n\tmov {} {destination}, {}", Self::size_2_opsize(destination.size), source_placeholder));
	}

	pub fn instr_push(&mut self, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tpush {} {source}", Self::size_2_opsize(source.size)));
	}

	pub fn instr_pop(&mut self, destination: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tpop {} {destination}", Self::size_2_opsize(destination.size)));
	}

	pub fn instr_ret(&mut self)
	{
		self.write_text_segment("\n\tret");
	}

	pub fn instr_add(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tadd {} {destination}, {source}", Self::size_2_opsize(destination.size)));
	}

	pub fn instr_sub(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tsub {} {destination}, {source}", Self::size_2_opsize(destination.size)));
	}

	pub fn instr_imul(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\timul {} {destination}, {source}", Self::size_2_opsize(destination.size)));
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
			let source_reg = self.reg_alloc_allocate(source.size).unwrap();
			let source_placeholder = Placeholder::new(
				PlaceholderKind::Reg(source_reg), 
				source.size
			);
			self.instr_mov(&source_placeholder, source);
			self.write_text_segment(&format!("\n\tidiv {} {}", Self::size_2_opsize(source.size), source_placeholder));
			self.reg_alloc_free(source_reg);
		} else
		{
			self.write_text_segment(&format!("\n\tidiv {} {source}", Self::size_2_opsize(source.size)));
		}
		self.reg_alloc_free(rdx_allocated);
	}

	pub fn instr_xor(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\txor {} {destination}, {source}", Self::size_2_opsize(destination.size)));
	}

	pub fn instr_call(&mut self, identifier: &str)
	{
		self.write_text_segment(&format!("\n\tcall {identifier}"));
	}
}