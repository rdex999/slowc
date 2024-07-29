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

}

impl<'a> CodeGen<'a>
{
	pub fn init_register_allocator() -> [RegisterInfo; ALLOCATABLE_REGS_COUNT]
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
	
	pub fn allocate(&mut self, size: u8) -> Option<Register>
	{
		for i in 0..self.registers.len()
		{
			if let Some(allocated_register) = self.allocate_sub_reg(i, size)
			{
				return Some(allocated_register);
			}
		}	
		return None;
	}
	
	pub fn free(&mut self, register: Register)
	{
		// If its a register that has an high 8 bits sub-register, the offset is 4, and if not, then the offset is 3
		let last_sub_reg_offset = if register as u8 >= Register::RAX as u8 && register as u8 <= Register::DH as u8 { 4 } else { 3 };
		
		for i in 0..self.registers.len()
		{
			let reg_info = self.registers[i];
			if register as u8 >= reg_info.register as u8 && register as u8 <= reg_info.register as u8 + last_sub_reg_offset
			{
				self.free_sub_reg(i, register);
				return;
			}
		}
	}
	
	pub fn check_leaks(&self)
	{
		for i in 0..self.registers.len()
		{
			if !self.is_all_sub_regs_free(i)
			{
				println!("Found allocated register:\n{:#?}", self.registers[i]);
			}
		}
	}
	// Size - in bytes
	fn allocate_sub_reg(&mut self, reg_info_idx: usize, size: u8) -> Option<Register>
	{
		let reg_info = &mut self.registers[reg_info_idx];

		if !reg_info.is_free
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
	
	// Returns whether should save the register (push)
	fn _allocate_sub_reg_forced(&mut self, reg_info_idx: usize, register: Register)
	{
		if let Some(_) = self.allocate_sub_reg(reg_info_idx, register.size() as u8)
		{
			return;
		}
	
		self.registers[reg_info_idx].push_count += 1;

		self.instr_push(&Placeholder::new(
			PlaceholderKind::Reg(self.registers[reg_info_idx].register), 
			OpSize::from_size(self.registers[reg_info_idx].register.size())
		));
	}
	
	// Returns whether should pop the register
	fn free_sub_reg(&mut self, reg_info_idx: usize, register: Register)
	{
		if self.registers[reg_info_idx].push_count != 0
		{
			self.registers[reg_info_idx].push_count -= 1;
			self.instr_pop(&Placeholder::new(
				PlaceholderKind::Reg(self.registers[reg_info_idx].register), 
				OpSize::from_size(self.registers[reg_info_idx].register.size())	
			));
			
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
	
	fn is_all_sub_regs_free(&self, reg_info_idx: usize) -> bool
	{
		let reg_info = self.registers[reg_info_idx];

		if reg_info.is_h8_free != None && !reg_info.is_h8_free.unwrap()
		{
			return false;
		}
		return reg_info.is_free && reg_info.is_l8_free;
	}
}