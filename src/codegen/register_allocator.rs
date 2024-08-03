use super::*;

const ALLOCATABLE_REGS_COUNT: usize = Register::COUNT_FULL as usize - 3;

#[derive(Debug, Clone, Copy)]
pub struct RegisterInfo
{
	pub register: Register,
	push_count: u8,	

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
			push_count: 0,
			is_free,
			is_l8_free,
			is_h8_free,
		};
	}

	pub fn is_used(&self) -> bool
	{
		return !self.is_free || !self.is_l8_free || (self.is_h8_free != None && !self.is_h8_free.unwrap());
	}
}

impl<'a> CodeGen<'a>
{
	pub fn reg_alloc_init() -> [RegisterInfo; ALLOCATABLE_REGS_COUNT]
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
		
    	return registers;
	}
	
	pub fn reg_alloc_allocate(&mut self, size: u8) -> Option<Register>
	{
		for i in 0..self.registers.len()
		{
			if let Some(allocated_register) = self.reg_alloc_allocate_sub_reg(i, size)
			{
				return Some(allocated_register);
			}
		}	
		return None;
	}

	pub fn reg_alloc_allocate_forced(&mut self, register: Register)
	{
		let index = Self::reg_alloc_register_2_index(register);
		self.reg_alloc_allocate_sub_reg_forced(index, register);
	}
	
	pub fn reg_alloc_free(&mut self, register: Register)
	{
		for i in 0..self.registers.len()
		{
			let reg_info = self.registers[i];
			if register as u8 >= reg_info.register as u8 && register as u8 <= reg_info.register as u8 + Self::reg_alloc_get_last_sub_reg_offset(register)
			{
				self.reg_alloc_free_sub_reg(i, register);
				return;
			}
		}
	}
	
	pub fn reg_alloc_check_leaks(&self)
	{
		for i in 0..self.registers.len()
		{
			if !self.reg_alloc_is_all_sub_regs_free(i)
			{
				println!("Found allocated register:\n{:#?}", self.registers[i]);
			}
		}
	}

	pub fn reg_alloc_save_used(&mut self)
	{
		for register_info in self.registers
		{
			if register_info.is_used()
			{
				self.instr_push(&Placeholder::new(
					PlaceholderKind::Reg(register_info.register), 
					register_info.register.size()
				));
			}
		}
	}

	pub fn reg_alloc_free_used(&mut self)
	{
		for i in (0..self.registers.len()).rev()
		{
			if self.registers[i].is_used()
			{
				self.instr_pop(&Placeholder::new(
					PlaceholderKind::Reg(self.registers[i].register), 
					self.registers[i].register.size()
				));
			}
		}
	}

	fn reg_alloc_register_2_index(register: Register) -> usize
	{
		let reg_int = register as usize;
		if reg_int >= Register::RAX as usize && reg_int <= Register::DH as usize
		{
			return reg_int / 5 - 1;
		}
		if reg_int < Register::RSP as usize
		{
			return (reg_int - (4 * 5)) / 4 + (4 - 1);
		}

		return (reg_int - (4 * 5)) / 4 + (4 - 1) - 2;
	}


	// Size - in bytes
	fn reg_alloc_allocate_sub_reg(&mut self, reg_info_idx: usize, size: u8) -> Option<Register>
	{
		let reg_info = &mut self.registers[reg_info_idx];

		if !reg_info.is_free || reg_info.push_count != 0
		{
			return None;
		}
	
		if size == 1
		{
			if reg_info.is_l8_free
			{
				reg_info.is_l8_free = false;
				return Some(Register::try_from(reg_info.register as u8 + 3).unwrap());
			}
			if reg_info.is_h8_free != None && reg_info.is_h8_free.unwrap()
			{
				reg_info.is_h8_free = Some(false);
				return Some(Register::try_from(reg_info.register as u8 + 4).unwrap());
			}
		}
	
		if !reg_info.is_l8_free || (reg_info.is_h8_free != None && !reg_info.is_h8_free.unwrap())
		{
			return None;
		}
	
		reg_info.is_free = false;
		// Pink jelly in skull hurts
		if reg_info.register as u8 >= Register::RAX as u8 && reg_info.register as u8 <= Register::DH as u8
		{
			return Some(Register::try_from(reg_info.register as u8 + (3 - (size as u8).trailing_zeros() as u8)).unwrap())
		}
		return Some(Register::try_from(reg_info.register as u8 + (3 - (size as u8).trailing_zeros() as u8) ).unwrap())
	}
	
	fn reg_alloc_get_last_sub_reg_offset(register: Register) -> u8
	{
		// If its a register that has an high 8 bits sub-register, the offset is 4, and if not, then the offset is 3
		return if register as u8 >= Register::RAX as u8 && register as u8 <= Register::DH as u8 { 4 } else { 3 };
	}

	fn reg_alloc_allocate_sub_reg_forced(&mut self, reg_info_idx: usize, register: Register)
	{
		if let Some(_) = self.reg_alloc_allocate_sub_reg(reg_info_idx, register.size() as u8)
		{
			return;
		}
	
		self.registers[reg_info_idx].push_count += 1;

		self.instr_push(&Placeholder::new(
			PlaceholderKind::Reg(self.registers[reg_info_idx].register), 
			self.registers[reg_info_idx].register.size()
		));
	}
	
	fn reg_alloc_free_sub_reg(&mut self, reg_info_idx: usize, register: Register)
	{
		if self.registers[reg_info_idx].push_count != 0
		{
			self.registers[reg_info_idx].push_count -= 1;
			self.instr_pop(&Placeholder::new(
				PlaceholderKind::Reg(self.registers[reg_info_idx].register), 
				self.registers[reg_info_idx].register.size()
			));

			// if self.registers[reg_info_idx].push_count == 0
			// {
			// 	self.registers[reg_info_idx].is_free = true;
			// 	self.registers[reg_info_idx].is_l8_free = true;
			// 	if self.registers[reg_info_idx].is_h8_free != None
			// 	{
			// 		self.registers[reg_info_idx].is_h8_free = Some(true);
			// 	}
			// }
			return;
		}
		
		let reg_info = &mut self.registers[reg_info_idx];

		reg_info.is_free = true;
		if register as u8 - reg_info.register as u8 == 4 	/* Means it was a high 8 bits register (AH, BH, CH, DH) */
		{
			reg_info.is_h8_free = Some(true);
			return;
		}
		reg_info.is_l8_free = true;
		return;
	}
	
	fn reg_alloc_is_all_sub_regs_free(&self, reg_info_idx: usize) -> bool
	{
		let reg_info = self.registers[reg_info_idx];

		if reg_info.is_h8_free != None && !reg_info.is_h8_free.unwrap()
		{
			return false;
		}
		return reg_info.is_free && reg_info.is_l8_free;
	}
}