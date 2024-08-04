use super::*;

impl<'a> CodeGen<'a>
{
	pub fn gen_function(&mut self, function: &Function)
	{
		self.decl_attribute(&function.identifier, function.attributes);
		
		if function.statements.len() == 0
		{
			return;
		}

		self.write_lable_text_seg(&function.identifier);

		// Save stack frame
		self.instr_push(&Placeholder::new(PlaceholderKind::Reg(Register::RBP), 8));
		self.instr_mov(
			&Placeholder::new(PlaceholderKind::Reg(Register::RBP), 8),
			&Placeholder::new(PlaceholderKind::Reg(Register::RSP), 8)
		);
		if function.stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), 8),
				&Placeholder::new(PlaceholderKind::Constant(function.stack_size as u64), 8), 
			);
		}

		self.instr_add_spacing();

		// Store parameters according to the functions calling convenction.
		if function.attributes & attribute::SYS_V_ABI_X86_64 != 0
		{
			self.store_parameters_sys_v_abi_x86_64(function);
		}


		self.gen_code_block(&function.statements, &function.locals);
	
		self.gen_function_return();
	}

	pub fn gen_function_return(&mut self)
	{
		self.instr_mov(
			&Placeholder::new(PlaceholderKind::Reg(Register::RSP), 8),
			&Placeholder::new(PlaceholderKind::Reg(Register::RBP), 8)
		);
		self.instr_pop(&Placeholder::new(PlaceholderKind::Reg(Register::RBP), 8));
		self.instr_ret();
	}

	pub fn gen_function_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo) -> Option<Placeholder>
	{
		let function = &self.ir.functions[function_call_info.index as usize];
		
		self.reg_alloc_save_used();

		if function.attributes & attribute::SYS_V_ABI_X86_64 != 0
		{
			self.gen_sys_v_abi_x86_64_call(locals, function_call_info);
		}
		
		self.reg_alloc_free_used();
		
		let return_size = function.return_type.size();
		if function.return_type == Type::Void 
		{
			return None;
		} else 
		{ 
			return Some(Placeholder::new(
				PlaceholderKind::Reg(Register::from_op_size(Register::RAX, return_size)), 
				return_size
			));
		}
	}

	fn gen_sys_v_abi_x86_64_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo)
	{
		let function = &self.ir.functions[function_call_info.index as usize];

		if function.parameters_stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), OP_QWORD), 
				&Placeholder::new(PlaceholderKind::Constant(function.parameters_stack_size as u64), OP_QWORD)
			);
		}

		// rdi, rsi, rdx, rcx, r8, r9,
		let mut allocated_registers: Vec<Register> = Vec::with_capacity(6);

		let mut integer_arguments: u8 = 0;
		let mut stack_position: u8 = 0;
		for (i, argument) in function_call_info.arguments.iter().enumerate()
		{
			let placeholder;
			let argument = self.gen_expression(argument, locals);
			let arg_data = function.locals[i];
			if arg_data.data_type.is_integer() && integer_arguments < 6
			{
				let register = Self::int_argument_2_register_sys_v_abi_x86_64(integer_arguments, arg_data.data_type.size());
				self.reg_alloc_allocate_forced(register);
				allocated_registers.push(register);
				placeholder = Placeholder::new(PlaceholderKind::Reg(register), arg_data.data_type.size());
				integer_arguments += 1;
			} else
			// When adding floats in the future, add em here
			{
				placeholder = Placeholder::new(
					PlaceholderKind::Location(LocationExpr::new(Register::RSP, None, stack_position as isize)), 
					arg_data.data_type.size()
				);
				stack_position += arg_data.data_type.size();
			}

			self.instr_mov(&placeholder, &argument);
		}

		self.instr_call(&function.identifier);

		for register in allocated_registers
		{
			self.reg_alloc_free(register);
		}

		if function.parameters_stack_size != 0
		{
			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), OP_QWORD), 
				&Placeholder::new(PlaceholderKind::Constant(stack_position as u64), OP_QWORD)
			);
		}
	}

	fn store_parameters_sys_v_abi_x86_64(&mut self, function: &Function)
	{
		let mut integer_arguments: u8 = 0;
		for parameter in &function.locals
		{
			if parameter.attributes & attribute::FUNCTION_PARAMETER == 0
			{
				break;
			}

			if parameter.data_type.is_integer() && integer_arguments < 6
			{
				let register = Self::int_argument_2_register_sys_v_abi_x86_64(integer_arguments, parameter.data_type.size());
				let source = Placeholder::new(PlaceholderKind::Reg(register), parameter.data_type.size());
				let destination = Placeholder::new(
					PlaceholderKind::Location(LocationExpr::new(Register::RBP, None, parameter.location)), 
					parameter.data_type.size()
				);
				self.instr_mov(&destination, &source);
				integer_arguments += 1;
			}
		}
	}

	fn int_argument_2_register_sys_v_abi_x86_64(argument: u8, size: OpSize) -> Register
	{
		return Register::from_op_size(
			match argument {
				0 => Register::RDI,
				1 => Register::RSI,
				2 => Register::RDX,
				3 => Register::RCX,
				4 => Register::R8,
				5 => Register::R9,
				_ => panic!("Rust stfu"),		/* Unreachable because of the if statement */
			},
			size
		);
	}
}