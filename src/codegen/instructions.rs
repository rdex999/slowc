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

pub struct Register
{
	kind: RegKind,
	size: RegSize
}

#[derive(Debug)]
pub enum RegKind
{
	RAX,
	RBX,
	RCX,
	RDX,
	RSI,
	RDI,
	RSP,
	RBP,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
}

pub enum RegSize
{
	L64,
	L32,
	L16,
	L8,
	H8
}

pub struct LocationExpr
{
	base: Register,
	base_multiplier: Option<isize>,
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
			Source::Location(_)	=> todo!(),
		}
	}
}

impl std::fmt::Display for Register
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		let mut name = format!("{:?}", self.kind).to_lowercase();
		match self.kind 
		{
			RegKind::RAX | RegKind::RBX | RegKind::RCX | RegKind::RDX =>
			{
				match self.size
				{
					RegSize::L64 	=> (),
					RegSize::L32 	=> name.replace_range(0..1, "e"),
					RegSize::L16 	=> {name.remove(0);},
					RegSize::L8 	=> {name.remove(0); name.replace_range(1..2, "l");},
					RegSize::H8 	=> {name.remove(0); name.replace_range(1..2, "h");},
				}	
			},
			
			
			RegKind::RDI | RegKind::RSI | RegKind::RSP | RegKind::RBP =>
			{
				match self.size 
				{
					RegSize::L64 	=> (),
					RegSize::L32 	=> name.replace_range(0..1, "e"),
					RegSize::L16 	=> {name.remove(0);},
					RegSize::L8 	=> {name.remove(0); name.push('l');},
					_ 				=> {panic!("Dev error! tried to use high 8 bits of register {name}.");}
				}	
			},

			RegKind::R8 | RegKind::R9 | RegKind::R10 | RegKind::R11 
			| RegKind::R12 | RegKind::R13 | RegKind::R14 | RegKind::R15 =>
			{
				match self.size
				{
					RegSize::L64	=> (),
					RegSize::L32	=> {name.push('d');},
					RegSize::L16	=> {name.push('w');},
					RegSize::L8		=> {name.push('b');},
					_				=> {panic!("Dev error! tried to high 8 bits of register {name}.");}
				}
			}
		}
		let _ = write!(f, "{name}");
		return Ok(());
	
	}
}

impl std::fmt::Display for OpSize
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
		write!(f, "{}", format!("{:?}", self).to_lowercase());
		return Ok(());
	}
}

impl Register
{
	pub fn new(kind: RegKind, size: RegSize) -> Self
	{
		return Self {
			kind,
			size,
		};
	}
}

impl RegSize
{
	pub fn _to_op_size(&self) -> OpSize
	{
		match self {
			RegSize::L64 				=> OpSize::Qword,	
			RegSize::L32 				=> OpSize::Dword,	
			RegSize::L16 				=> OpSize::Word,	
			RegSize::L8 | RegSize::H8 	=> OpSize::Byte,	
		}
	}
}

impl<'a> CodeGen<'a>
{
	pub fn instr_mov(&mut self, destination: Destination, source: Source, size: OpSize)
	{
		self.write_text_segment(&format!("\n\tmov {size} {destination}, {source}"));
	}
}