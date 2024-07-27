use super::*;

pub struct RegisterAllocator
{
	registers: [RegisterInfo; Register::COUNT as usize],

}

struct RegisterInfo
{
	register: Register,
	is_free: bool,
}

impl RegisterInfo
{
	pub fn new(register: Register) -> Self
	{
		return Self {
			register,
			is_free: true,
		};
	}
}

impl RegisterAllocator
{
	pub fn new() -> Self
	{
		let registers: [RegisterInfo; Register::COUNT as usize] = core::array::from_fn(|i| {
			let reg = Register::try_from(i as u8).unwrap();
			return RegisterInfo::new(reg);
		});

		return Self {
    		registers,
		};
	}
}