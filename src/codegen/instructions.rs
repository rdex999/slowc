use super::*;

pub type OpSize = u8;
pub const OP_BYTE: OpSize = 1;
pub const OP_WORD: OpSize = 2;
pub const OP_DWORD: OpSize = 4;
pub const OP_QWORD: OpSize = 8;

#[derive(Clone, Copy)]
pub struct Placeholder
{
	pub kind: PlaceholderKind,
	pub data_type: Type,
}

#[derive(Clone, Copy)]
pub enum PlaceholderKind
{
	Reg(Register),
	Location(LocationExpr),
	Integer(u64),
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Lable
{
	pub index: usize,
	pub kind: LableKind
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum LableKind
{
	DataSeg,
	TextSeg,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct LocationExpr
{
	base: LocationExprPart,
	offset: LocationExprPart,
	offset_multiplier: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum LocationExprPart
{
	Reg(Register),
	Offset(isize),
	Labl(Lable),
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

	XMM0,
	XMM1,
	XMM2,
	XMM3,
	XMM4,
	XMM5,
	XMM6,
	XMM7,
	XMM8,
	XMM9,
	XMM10,
	XMM11,
	XMM12,
	XMM13,
	XMM14,
	XMM15,
}


impl std::fmt::Display for Placeholder
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		match &self.kind {
			PlaceholderKind::Reg(register) => write!(f, "{register}"),
			PlaceholderKind::Integer(value) => write!(f, "{value}"),
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
		let _ = write!(f, "[{} + {}", self.base, self.offset);
		if let Some(multiplier) = self.offset_multiplier
		{
			let _ = write!(f, " * {multiplier}");
		}
		let _ = write!(f, "]");
		return Ok(());
	}
}

impl std::fmt::Display for LocationExprPart
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		match self
		{
			LocationExprPart::Reg(register) 	=> { let _ = write!(f, "{register}"); },
			LocationExprPart::Offset(offset) 	=> { let _ = write!(f, "{offset}"); },
			LocationExprPart::Labl(lable) 		=> { let _ = write!(f, "{lable}"); },
		}

		return Ok(());
	}
}

impl std::fmt::Display for Lable
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		match self.kind
		{
			LableKind::TextSeg => { let _ = write!(f, "LT{}", self.index); },
			LableKind::DataSeg => { let _ = write!(f, "LD{}", self.index); },
		}

		return Ok(());
	}
}

impl LocationExpr
{
	pub fn new(base: LocationExprPart, offset: LocationExprPart, offset_multiplier: Option<usize>) -> Self
	{
		return Self {
			base,
			offset,
			offset_multiplier,
		};
	}
	
	pub fn from_placeholder(placeholder: &Placeholder) -> Self
	{
		match placeholder.kind
		{
			PlaceholderKind::Integer(value) => Self::new(LocationExprPart::Offset(value as isize), LocationExprPart::Offset(0), None),
			PlaceholderKind::Location(expr) => expr,
			PlaceholderKind::Reg(register) => Self::new(LocationExprPart::Reg(register), LocationExprPart::Offset(0), None),
		}
	}
}

impl Lable
{
	pub fn new(index: usize, kind: LableKind) -> Self
	{
		return Self {
			index,
			kind
		};
	}
}

impl Register
{
	pub const COUNT: u8 = 84;
	pub const COUNT_FULL: u8 = 16 + 16;

	// The size of the register in bytes
	pub fn data_type(&self) -> Type
	{
		if *self as u8 >= Register::XMM0 as u8 && *self as u8 <= Register::XMM15 as u8
		{
			return Type::new(TypeKind::F64);
		}

		match self {
			Register::RAX | Register::RBX | Register::RCX | Register::RDX |
			Register::RSI | Register::RDI | Register::RSP | Register::RBP |
			Register::R8  | Register::R9  | Register::R10 | Register::R11 |
			Register::R12 | Register::R13 | Register::R14 | Register::R15 =>
			{
				return Type::new(TypeKind::U64);
			},

			Register::EAX | Register::EBX | Register::ECX | Register::EDX |
			Register::ESI | Register::EDI | Register::ESP | Register::EBP |
			Register::R8D  | Register::R9D  | Register::R10D | Register::R11D |
			Register::R12D | Register::R13D | Register::R14D | Register::R15D =>
			{
				return Type::new(TypeKind::U32);
			},

			Register::AX | Register::BX | Register::CX | Register::DX |
			Register::SI | Register::DI | Register::SP | Register::BP |
			Register::R8W  | Register::R9W  | Register::R10W | Register::R11W |
			Register::R12W | Register::R13W | Register::R14W | Register::R15W =>
			{
				return Type::new(TypeKind::U16);
			},
			_ => return Type::new(TypeKind::U8),
		}
	}

	pub fn is_general(&self) -> bool
	{
		return *self as u8 >= Register::RAX as u8 && *self as u8 <= Register::R15B as u8;
	}

	pub fn from_op_size(base_register: Register, op_size: OpSize) -> Self
	{
		if base_register as u8 >= Register::XMM0 as u8 && base_register as u8 <= Register::XMM15 as u8
		{
			return base_register;
		}
		return Register::try_from(base_register as OpSize + 4 - (op_size.trailing_zeros() as OpSize + 1)).unwrap();
	}

	pub fn default_for_type(data_type: Type) -> Self
	{
		if data_type.is_integer()
		{
			return Register::RAX.of_size(data_type.size());
		}
		return Register::XMM0;
	}

	pub fn base_register(&self) -> Register
	{
		let idx = *self as u8;
		if idx >= Register::RAX as u8 && idx <= Register::DH as u8
		{
			return Register::try_from(idx - idx % 5).unwrap();
		} else if idx >= Register::RSI as u8 && idx <= Register::R15B as u8
		{
			return Register::try_from(idx - idx % 4).unwrap();
		}
		return Register::try_from(idx).unwrap();
	}

	pub fn of_size(&self, size: OpSize) -> Register
	{
		let base = self.base_register();
		return Register::from_op_size(base, size);
	}
}

impl TryFrom<OpSize> for Register
{
	type Error = ();

	fn try_from(value: OpSize) -> Result<Self, Self::Error> {
		if value >= Register::COUNT
		{
			return Err(());
		}

		return unsafe { Ok(std::mem::transmute(value)) };
	}
}
impl Placeholder
{
	pub fn new(kind: PlaceholderKind, data_type: Type) -> Self
	{
		return Self {
			kind,
			data_type,
		};
	}

	pub fn is_constant(&self) -> bool
	{
		match self.kind
		{
			PlaceholderKind::Integer(_) => return true,
			_ => return false,
		}
	}

	pub fn is_register(&self) -> bool
	{
		return match self.kind
		{
			PlaceholderKind::Reg(_) => true,
			_ => false,
		};
	}

	pub fn is_location(&self) -> bool
	{
		return match self.kind
		{
			PlaceholderKind::Location(_) => true,
			_ => false
		};
	}

	// If the placeholder is not a register false is returned, if its a register, then return placeholder.reg == register
	pub fn is_register_eq(&self, register: Register) -> bool
	{
		if let PlaceholderKind::Reg(reg) = self.kind
		{
			return reg == register;
		}

		return false;
	}

	pub fn of_type(&self, data_type: Type) -> Placeholder
	{
		if let PlaceholderKind::Reg(register) = self.kind
		{
			return Placeholder::new(PlaceholderKind::Reg(register.of_size(data_type.size())), data_type);
		}
		return Placeholder::new(self.kind, data_type);
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

			PlaceholderKind::Integer(value) =>
			{
				if let PlaceholderKind::Integer(other_value) = other.kind
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
	fn size_2_opsize<'b>(size: OpSize) -> &'b str
	{
		match size
		{
			OP_BYTE => return "byte",
			OP_WORD	=> return "word",
			OP_DWORD => return "dword",
			OP_QWORD => return "qword",
			_ => panic!("Dev error! size_2_opsize({size}) called with a size thats not a power of 2."),
		}
	}

	pub fn instr_add_spacing(&mut self)
	{
		self.write_text_segment("\n");
	}

	pub fn instr_mov(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		if destination == source
		{
			return;
		}

		let mut source_placeholder = *source;
		if let PlaceholderKind::Location(_) = destination.kind
		{
			if let PlaceholderKind::Location(_) = source.kind
			{
				source_placeholder = Placeholder::new(
					PlaceholderKind::Reg(Register::default_for_type(source.data_type)), 
					source.data_type
				);
				self.instr_mov(&source_placeholder, source);
			}
		}

		if destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tmov {} {destination}, {source_placeholder}", Self::size_2_opsize(destination.data_type.size())));
			return;
		}
		if destination.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\tmovsd {destination}, {source_placeholder}"));
		} else if destination.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\tmovss {destination}, {source_placeholder}"));
		}
	}

	pub fn instr_movzx(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!(
			"\n\tmovzx {} {destination}, {} {source}",
			Self::size_2_opsize(destination.data_type.size()),
			Self::size_2_opsize(source.data_type.size())
		));
	}

	pub fn instr_movsx(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!(
			"\n\tmovsx {} {destination}, {} {source}",
			Self::size_2_opsize(destination.data_type.size()),
			Self::size_2_opsize(source.data_type.size())
		));
	}

	pub fn instr_lea(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tlea {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
	}

	// Convert single floating point (64/32 bit) into an integer
	pub fn instr_cvttsf2si(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		let destination = if destination.data_type.size() < 4 { destination.of_type(Type::new(TypeKind::U32)) } else { *destination };
		if source.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\tcvttsd2si {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
		} else if source.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\tcvttss2si {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			panic!("instr_cvttsf2si called with a non floating point source.");
		}
	}

	// Convert single integer into single floating point (32/64 bit)
	pub fn instr_cvtsi2sf(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		let source = if source.data_type.size() < 4 { source.of_type(Type::new(TypeKind::U32)) } else { *source };
		if destination.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\tcvtsi2sd {destination}, {} {source}", Self::size_2_opsize(source.data_type.size())));
		} else if destination.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\tcvtsi2ss {destination}, {} {source}", Self::size_2_opsize(source.data_type.size()) ));
		} else
		{
			panic!("instr_cvttsi2sf called with a non floating point destination.");
		}
	}

	pub fn instr_cvtsf2sf(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		if source.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\tcvtsd2ss {destination}, {source}"));
		} else if source.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\tcvtss2sd {destination}, {source}"));
		} else
		{
			panic!("instr_cvttsi2sf called with a non floating point source.");
		}
	}

	pub fn instr_push(&mut self, source: &Placeholder)
	{
		if let PlaceholderKind::Reg(register) = source.kind
		{
			if register.is_general()
			{
				self.write_text_segment(&format!("\n\tpush {} {source}", Self::size_2_opsize(source.data_type.size())));
				return;
			}

			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::new(TypeKind::U64)),
				&Placeholder::new(PlaceholderKind::Integer(source.data_type.size() as u64), Type::new(TypeKind::U64))
			);

			self.instr_mov(
				&Placeholder::new(PlaceholderKind::Location(LocationExpr::new(
					LocationExprPart::Reg(Register::RSP),
					LocationExprPart::Offset(0),
					None,
					)), source.data_type), 
				source
			);
			return;
		}
		self.write_text_segment(&format!("\n\tpush {} {source}", Self::size_2_opsize(source.data_type.size())));
	}

	pub fn instr_pop(&mut self, destination: &Placeholder)
	{
		if let PlaceholderKind::Reg(register) = destination.kind
		{
			if register.is_general()
			{
				self.write_text_segment(&format!("\n\tpop {} {destination}", Self::size_2_opsize(destination.data_type.size())));
				return;
			}

			self.instr_mov(
				destination,
				&Placeholder::new(PlaceholderKind::Location(LocationExpr::new(
					LocationExprPart::Reg(Register::RSP),
					LocationExprPart::Offset(0),
					None
					)), destination.data_type)
			);

			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::new(TypeKind::U64)), 
				&Placeholder::new(PlaceholderKind::Integer(destination.data_type.size() as u64), Type::new(TypeKind::U64))
			);
			return;
		}
		self.write_text_segment(&format!("\n\tpop {} {destination}", Self::size_2_opsize(destination.data_type.size())));
	}

	pub fn instr_ret(&mut self)
	{
		self.write_text_segment("\n\tret");
	}

	pub fn instr_add(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		if destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tadd {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
		} else if destination.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\taddsd {destination}, {source}"));
		} else if destination.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\taddss {destination}, {source}"));
		}
	}

	pub fn instr_sub(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		if destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tsub {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
		} else if destination.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\tsubsd {destination}, {source}"));
		} else if destination.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\tsubss {destination}, {source}"));
		}
	}

	pub fn instr_mul(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		let mut allocated_register = None;
		let mut src_placeholder = *source;
		if source.is_constant()
		{
			allocated_register = self.reg_alloc_allocate(source.data_type);
			src_placeholder = Placeholder::new(PlaceholderKind::Reg(allocated_register.unwrap()), src_placeholder.data_type);
			self.instr_mov(&src_placeholder, &source);
		}

		if destination.data_type.is_integer()
		{
			if destination.data_type.is_signed()
			{
				if destination.data_type.size() == OP_BYTE
				{
					if !destination.is_register_eq(Register::AL)	
					{
						self.instr_mov(
							&Placeholder::new(PlaceholderKind::Reg(Register::AL), destination.data_type), 
							destination
						);
					}
					self.write_text_segment(&format!("\n\timul {} {src_placeholder}", Self::size_2_opsize(src_placeholder.data_type.size())));

					let al_placeholder = Placeholder::new(PlaceholderKind::Reg(Register::AL), destination.data_type);
					if *destination != al_placeholder
					{
						self.instr_mov(&destination, &al_placeholder);
					}
				} else		/* If its not a byte multiplication */
				{
					self.write_text_segment(&format!("\n\timul {} {destination}, {src_placeholder}", Self::size_2_opsize(destination.data_type.size())));
				}
			} else 		/* If doing an unsigned multiplication */
			{
				let rax = Register::from_op_size(Register::RAX, destination.data_type.size());
				if !destination.is_register_eq(rax)	
				{
					self.instr_mov(
						&Placeholder::new(PlaceholderKind::Reg(rax), destination.data_type), 
						destination
					);
				}
				if !src_placeholder.is_register()
				{
					allocated_register = self.reg_alloc_allocate(source.data_type);
					self.instr_mov(&src_placeholder, &source);
				}

				self.write_text_segment(&format!("\n\tmul {} {src_placeholder}", Self::size_2_opsize(src_placeholder.data_type.size())));

				if !destination.is_register_eq(rax)	
				{
					self.instr_mov(
						destination,
						&Placeholder::new(PlaceholderKind::Reg(rax), destination.data_type)
					);
				}
			}
		} else		/* If doing floating point multiplication */
		{
			let mut dst_placeholder = *destination;
			let mut allocated_dst_reg = None;

			if !destination.is_register()
			{
				allocated_dst_reg = self.reg_alloc_allocate(dst_placeholder.data_type);
				dst_placeholder = Placeholder::new(PlaceholderKind::Reg(allocated_dst_reg.unwrap()), dst_placeholder.data_type);
				self.instr_mov(&dst_placeholder, destination);
			}

			if destination.data_type == Type::new(TypeKind::F64)
			{
				self.write_text_segment(&format!("\n\tmulsd {dst_placeholder}, {src_placeholder}"));
			} else if destination.data_type == Type::new(TypeKind::F32)
			{
				self.write_text_segment(&format!("\n\tmulss {dst_placeholder}, {src_placeholder}"));
			}

			if *destination != dst_placeholder
			{
				self.instr_mov(destination, &dst_placeholder);
			}

			if let Some(allocated_dst_reg) = allocated_dst_reg
			{
				self.reg_alloc_free(allocated_dst_reg);
			}
		}

		if let Some(allocated_register) = allocated_register
		{
			self.reg_alloc_free(allocated_register);
		}
	}

	pub fn instr_div(&mut self, destination: &Placeholder, source: &Placeholder, get_modulo_in_dest: bool)
	{
		let mut dst = *destination;
		let mut src = *source;
		let mut src_reg = None;
		let mut dst_reg = None;
		
		if source.is_constant()
		{
			src_reg = self.reg_alloc_allocate(source.data_type);
			src = Placeholder::new(PlaceholderKind::Reg(src_reg.unwrap()), source.data_type);
			self.instr_mov(&src, &source);
		}

		if destination.data_type.is_integer()
		{
			let rax = Register::from_op_size(Register::RAX, destination.data_type.size());
			let rax_placeholder = Placeholder::new(PlaceholderKind::Reg(rax), destination.data_type);
			if !destination.is_register_eq(rax)
			{
				self.instr_mov(&rax_placeholder, destination);			
			}

			let rdx_allocated = Register::from_op_size(Register::RDX, src.data_type.size());
			
			let ah_placeholder = Placeholder::new(PlaceholderKind::Reg(Register::AH), src.data_type);
			let rdx_placeholder = Placeholder::new(PlaceholderKind::Reg(rdx_allocated), src.data_type);
			if destination.data_type.is_signed()
			{
				match src.data_type.size()
				{
					OP_BYTE  => self.instr_cbw(),
					OP_WORD  => self.instr_cwd(),
					OP_DWORD => self.instr_cdq(),
					OP_QWORD => self.instr_cqo(),
					_ => todo!("Unimplemented Sign convertion."),
				}
				self.write_text_segment(&format!("\n\tidiv {} {src}", Self::size_2_opsize(src.data_type.size())));
			} else
			{
				if src.data_type.size() == OP_BYTE
				{
					self.instr_xor(&ah_placeholder, &ah_placeholder);
				} else
				{
					if src.data_type.size() != OP_BYTE && !destination.is_register_eq(rdx_allocated)
					{
						self.reg_alloc_allocate_forced(rdx_allocated);
					}
					self.instr_xor(&rdx_placeholder, &rdx_placeholder);
				}
				self.write_text_segment(&format!("\n\tdiv {} {src}", Self::size_2_opsize(src.data_type.size())));
			}

			if !destination.is_register_eq(rax) && !get_modulo_in_dest
			{
				self.instr_mov(destination, &rax_placeholder);
			} else if get_modulo_in_dest
			{
				if src.data_type.size() == OP_BYTE && !destination.is_register_eq(Register::AH)
				{
					self.instr_mov(destination, &ah_placeholder);
				} else if src.data_type.size() != OP_BYTE && !destination.is_register_eq(rdx_allocated)
				{
					self.instr_mov(destination, &rdx_placeholder);
				}
			}

			if !src.data_type.is_signed() && src.data_type.size() != OP_BYTE && !destination.is_register_eq(rdx_allocated)
			{
				self.reg_alloc_free(rdx_allocated);
			}

		} else
		{
			if !destination.is_register()
			{
				dst_reg = self.reg_alloc_allocate(destination.data_type);
				dst = Placeholder::new(PlaceholderKind::Reg(dst_reg.unwrap()), destination.data_type);
				self.instr_mov(&dst, &destination);
			}

			if src.data_type == Type::new(TypeKind::F64)
			{
				self.write_text_segment(&format!("\n\tdivsd {dst}, {src}"));
			} else if src.data_type == Type::new(TypeKind::F32)
			{
				self.write_text_segment(&format!("\n\tdivss {dst}, {src}"));
			}
		}

		if let Some(src_reg) = src_reg
		{
			self.reg_alloc_free(src_reg);
		}

		if let Some(dst_reg) = dst_reg
		{
			self.reg_alloc_free(dst_reg);
		}
	}

	pub fn instr_xor(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\txor {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
	}
	
	pub fn instr_or(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tor {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
	}

	pub fn instr_not(&mut self, destination: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tnot {} {destination}", Self::size_2_opsize(destination.data_type.size())));
	}

	pub fn instr_call(&mut self, identifier: &str)
	{
		self.write_text_segment(&format!("\n\tcall {identifier}"));
	}

	pub fn instr_cbw(&mut self)
	{
		self.write_text_segment("\n\tcbw");
	}
	
	pub fn instr_cwd(&mut self)
	{
		self.write_text_segment("\n\tcwd");
	}

	pub fn instr_cdq(&mut self)
	{
		self.write_text_segment("\n\tcdq");
	}

	pub fn instr_cqo(&mut self)
	{
		self.write_text_segment("\n\tcqo");
	}

	pub fn instr_cdqe(&mut self)
	{
		self.write_text_segment("\n\tcdqe");
	}

	pub fn instr_cmp(&mut self, lhs: &Placeholder, rhs: &Placeholder)
	{
		let mut lhs_placeholder = *lhs;
		let mut allocated_register = None;
		if (lhs.is_location() && rhs.is_location()) || (lhs.is_constant() && rhs.is_constant())
		{
			allocated_register = self.reg_alloc_allocate(lhs.data_type);
			lhs_placeholder = Placeholder::new(PlaceholderKind::Reg(allocated_register.unwrap()), lhs.data_type);
			self.instr_mov(&lhs_placeholder, lhs);
		}

		if lhs_placeholder.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tcmp {} {lhs_placeholder}, {rhs}", Self::size_2_opsize(lhs_placeholder.data_type.size())));
		} else if lhs_placeholder.data_type == Type::new(TypeKind::F64)
		{
			self.write_text_segment(&format!("\n\tucomisd {lhs_placeholder}, {rhs}"));
		} else if lhs_placeholder.data_type == Type::new(TypeKind::F32)
		{
			self.write_text_segment(&format!("\n\tucomiss {lhs_placeholder}, {rhs}"));
		}

		if let Some(register) = allocated_register
		{
			self.reg_alloc_free(register);
		}
	}

	pub fn instr_sete(&mut self, destination: &Placeholder)
	{
		let destination = destination.of_type(Type::new(TypeKind::U8));
		self.write_text_segment(&format!("\n\tsete {} {destination}", Self::size_2_opsize(destination.data_type.size())));
	}

	pub fn instr_setne(&mut self, destination: &Placeholder)
	{
		let destination = destination.of_type(Type::new(TypeKind::U8));
		self.write_text_segment(&format!("\n\tsetne {} {destination}", Self::size_2_opsize(destination.data_type.size())));
	}

	pub fn instr_setg(&mut self, destination: &Placeholder)
	{
		let destination = destination.of_type(Type::new(TypeKind::U8));
		if destination.data_type.is_signed() && destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tsetg {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			self.write_text_segment(&format!("\n\tseta {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		}
	}

	pub fn instr_setl(&mut self, destination: &Placeholder)
	{
		let destination = destination.of_type(Type::new(TypeKind::U8));
		if destination.data_type.is_signed() && destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tsetl {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			self.write_text_segment(&format!("\n\tsetb {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		}
	}

	pub fn instr_setge(&mut self, destination: &Placeholder)
	{
		let destination = destination.of_type(Type::new(TypeKind::U8));
		if destination.data_type.is_signed() && destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tsetge {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			self.write_text_segment(&format!("\n\tsetae {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		}
	}

	pub fn instr_setle(&mut self, destination: &Placeholder)
	{
		let destination = destination.of_type(Type::new(TypeKind::U8));
		if destination.data_type.is_signed() && destination.data_type.is_integer()
		{
			self.write_text_segment(&format!("\n\tsetle {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			self.write_text_segment(&format!("\n\tsetbe {} {destination}", Self::size_2_opsize(destination.data_type.size())));
		}
	}

	pub fn instr_setz(&mut self, destination: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tsetz {} {destination}", Self::size_2_opsize(destination.data_type.size())));
	}

	pub fn instr_test(&mut self, lhs: &Placeholder, rhs: &Placeholder)
	{
		let mut lhs_placeholder = *lhs;
		let mut allocated_register = None;	
		if (lhs.is_location() && rhs.is_location()) || (lhs.is_constant() && (rhs.is_location() || rhs.is_constant()))
		{
			allocated_register = self.reg_alloc_allocate(lhs.data_type);
			lhs_placeholder = Placeholder::new(PlaceholderKind::Reg(allocated_register.unwrap()), lhs.data_type);
			self.instr_mov(&lhs_placeholder, lhs);
		}

		self.write_text_segment(&format!("\n\ttest {} {lhs_placeholder}, {rhs}", Self::size_2_opsize(lhs_placeholder.data_type.size())));

		if let Some(allocated_register) = allocated_register
		{
			self.reg_alloc_free(allocated_register);
		}
	}

	pub fn instr_jmp(&mut self, lable: Lable)
	{
		self.write_text_segment(&format!("\n\tjmp {lable}"));
	}

	pub fn instr_jz(&mut self, lable: Lable)
	{
		self.write_text_segment(&format!("\n\tjz {lable}"));
	}

	pub fn instr_jnz(&mut self, lable: Lable)
	{
		self.write_text_segment(&format!("\n\tjnz {lable}"));
	}

	pub fn instr_shr(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		let mut src_placeholder = *source;
		if !source.is_constant() && !source.is_register_eq(Register::from_op_size(Register::RCX, source.data_type.size()))
		{
			self.reg_alloc_allocate_forced(Register::CL);
			src_placeholder = Placeholder::new(PlaceholderKind::Reg(Register::CL), source.data_type);
			self.instr_mov(
				&Placeholder::new(
					PlaceholderKind::Reg(Register::from_op_size(Register::RCX, source.data_type.size())), 
					source.data_type
				), 
				source
			);
		}

		if src_placeholder.is_constant()
		{
			self.write_text_segment(&format!("\n\tshr {} {destination}, {src_placeholder}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			self.write_text_segment(&format!(
				"\n\tshr {} {destination}, {}", Self::size_2_opsize(destination.data_type.size()), Register::CL
			));
		}

		if !source.is_constant()
		{
			self.reg_alloc_free(Register::CL);
		}
	}

	pub fn instr_shl(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		let mut src_placeholder = *source;
		if !source.is_constant() && !source.is_register_eq(Register::from_op_size(Register::RCX, source.data_type.size()))
		{
			self.reg_alloc_allocate_forced(Register::CL);
			src_placeholder = Placeholder::new(PlaceholderKind::Reg(Register::CL), source.data_type);
			self.instr_mov(
				&Placeholder::new(
					PlaceholderKind::Reg(Register::from_op_size(Register::RCX, source.data_type.size())), 
					source.data_type
				), 
				source
			);
		}

		if src_placeholder.is_constant()
		{
			self.write_text_segment(&format!("\n\tshl {} {destination}, {src_placeholder}", Self::size_2_opsize(destination.data_type.size())));
		} else
		{
			self.write_text_segment(&format!(
				"\n\tshl {} {destination}, {}", Self::size_2_opsize(destination.data_type.size()), Register::CL
			));
		}

		if !source.is_constant()
		{
			self.reg_alloc_free(Register::CL);
		}
	}

	pub fn instr_and(&mut self, destination: &Placeholder, source: &Placeholder)
	{
		self.write_text_segment(&format!("\n\tand {} {destination}, {source}", Self::size_2_opsize(destination.data_type.size())));
	}
}