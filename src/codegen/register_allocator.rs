use super::*;

const ALLOCATABLE_REGS_COUNT: usize = Register::COUNT_FULL as usize - 3;

pub struct RegisterAllocator
{
	registers: [RegisterInfo; ALLOCATABLE_REGS_COUNT],

}

#[derive(Debug)]
struct RegisterInfo
{
	pub register: Register,
 
	// Using an option, because the register might not have lower parts (for example RDI doesnt have an high 8 bits sub-register)
	// In the future there will also be ZMM registers
	pub is_free: bool,					/* If the higher parts of the registers are free */
	pub is_l8_free: bool,
	pub is_h8_free: Option<bool>,		/* If exists, specifies if the high 8 bits of the low 16 bits are free */
}

impl RegisterInfo
{
	pub fn new(register: Register) -> Self
	{
		let is_free = true;
		let is_l8_free = true;
		let is_h8_free;

		match register
		{
			Register::RAX | Register::RBX | Register::RCX | Register::RDX => is_h8_free = Some(true),
			_ => is_h8_free = None,
		}

		return Self {
			register,
			is_free,
			is_l8_free,
			is_h8_free,
		};
	}

	// Size - in bytes
	pub fn allocate_sub_reg(&mut self, size: u8) -> Option<Register>
	{
		if !self.is_free
		{
			return None;
		}

		if size == 1
		{
			if self.is_l8_free
			{
				self.is_l8_free = false;
				return Some(Register::try_from(self.register as u8 + 3).unwrap());
			}
			if self.is_h8_free != None && self.is_h8_free.unwrap()
			{
				self.is_h8_free = Some(false);
				return Some(Register::try_from(self.register as u8 + 4).unwrap());
			}
		}

		if !self.is_l8_free || (self.is_h8_free != None && !self.is_h8_free.unwrap())
		{
			return None;
		}

		self.is_free = false;
		// Pink jelly in skull hurts
		if self.register as u8 >= Register::RAX as u8 && self.register as u8 <= Register::DH as u8
		{
			return Some(Register::try_from(self.register as u8 + (3 - (size as u8).trailing_zeros() as u8)).unwrap())
		}
		return Some(Register::try_from(self.register as u8 + (3 - (size as u8).trailing_zeros() as u8) ).unwrap())
	}

	pub fn free_sub_reg(&mut self, register: Register)
	{
		self.is_free = true;
		if register as u8 - self.register as u8 == 4 	/* Means it was a high 8 bits register (AH, BH, CH, DH) */
		{
			self.is_h8_free = Some(true);
			return;
		}
		self.is_l8_free = true;
	}
}

impl RegisterAllocator
{
	pub fn new() -> Self
	{
		let mut register_index: u8 = 0;
		let registers: [RegisterInfo; ALLOCATABLE_REGS_COUNT] = core::array::from_fn(|_| {
			loop
			{
				let reg = Register::try_from(register_index).unwrap();
				match reg
				{
					Register::RBX | Register::RCX | Register::RDX => register_index += 5,
					Register::RAX => {register_index += 5; continue;},
					Register::RSP | Register::RBP => {register_index += 4; continue;},
					_ => register_index += 4,
				}
				return RegisterInfo::new(reg);
			}
		});

		return Self {
    		registers,
		};
	}

	pub fn allocate(&mut self, size: u8) -> Option<Register>
	{
		for register in &mut self.registers
		{
			if let Some(register) = register.allocate_sub_reg(size)
			{
				return Some(register);
			}
		}	
		return None;
	}

	pub fn free(&mut self, register: Register)
	{
		// If its a register that has an high 8 bits sub-register, the offset is 4, and if not, then the offset is 3
		let last_sub_reg_offset = if register as u8 >= Register::RAX as u8 && register as u8 <= Register::DH as u8 { 4 } else { 3 };

		for reg in &mut self.registers
		{
			if register as u8 >= reg.register as u8 && register as u8 <= reg.register as u8 + last_sub_reg_offset
			{
				reg.free_sub_reg(register);
				return;
			}
		}
	}
}