use super::*;

impl<'a> CodeGen<'a>
{
	pub fn gen_function(&mut self, function: &Function)
	{
		self.decl_attribute(&function.identifier, function.attributes);
		
		if function.code_block.statements.len() == 0
		{
			return;
		}

		self.write_lable_text_seg(&function.identifier);

		// Save stack frame
		self.instr_push(&Placeholder::new(PlaceholderKind::Reg(Register::RBP), Type::U64));
		self.instr_mov(
			&Placeholder::new(PlaceholderKind::Reg(Register::RBP), Type::U64),
			&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::U64)
		);
		if function.code_block.stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::U64),
				&Placeholder::new(PlaceholderKind::Integer(function.code_block.stack_size as u64), Type::U64), 
			);
		}

		self.instr_add_spacing();

		// Store parameters according to the functions calling convenction.
		if function.attributes & attribute::SYS_V_ABI_X86_64 != 0
		{
			self.store_parameters_sys_v_abi_x86_64(function);
		}

		self.gen_scope(&function.code_block, &function.locals);
	
		self.gen_function_return();
	}

	pub fn gen_function_return(&mut self)
	{
		self.instr_mov(
			&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::U64),
			&Placeholder::new(PlaceholderKind::Reg(Register::RBP), Type::U64)
		);
		self.instr_pop(&Placeholder::new(PlaceholderKind::Reg(Register::RBP), Type::U64));
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
		
		if function.return_type == Type::Void 
		{
			return None;
		} else 
		{ 
			return Some(Placeholder::new(
				PlaceholderKind::Reg(Register::from_op_size(Register::default_for_type(function.return_type), function.return_type.size())), 
				function.return_type	
			));
		}
	}

	fn gen_sys_v_abi_x86_64_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo)
	{
		let function = &self.ir.functions[function_call_info.index as usize];

		if function.parameters_stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::U64), 
				&Placeholder::new(PlaceholderKind::Integer(function.parameters_stack_size as u64), Type::U64)
			);
		}

		// rdi, rsi, rdx, rcx, r8, r9,
		let mut allocated_registers: Vec<Register> = Vec::with_capacity(6);
		let mut integer_arguments: u8 = 0;
		let mut float_arguments: u8 = 0;
		let mut stack_position: u8 = 0;

		for (i, argument) in function_call_info.arguments.iter().enumerate()
		{
			let placeholder;
			let argument = self.gen_expression(argument, locals);
			let arg_data = function.locals[i];
			// println!("{}\n\n", self.text_segment);

			if arg_data.data_type.is_integer() && integer_arguments < 6
			{
				let register = Self::int_argument_2_register_sys_v_abi_x86_64(integer_arguments, arg_data.data_type.size());
				self.reg_alloc_allocate_forced(register);
				allocated_registers.push(register);
				placeholder = Placeholder::new(PlaceholderKind::Reg(register), arg_data.data_type);
				integer_arguments += 1;
			} else if !arg_data.data_type.is_integer() && float_arguments < 15
			{
				let register = Self::float_argument_2_register_sys_v_abi_x86_64(float_arguments);
				self.reg_alloc_allocate_forced(register);
				allocated_registers.push(register);
				placeholder = Placeholder::new(PlaceholderKind::Reg(register), arg_data.data_type);
				float_arguments += 1;
			} else
			{
				placeholder = Placeholder::new(
					PlaceholderKind::Location(
						LocationExpr::new(
							LocationExprPart::Reg(Register::RSP),
							LocationExprPart::Offset(stack_position as isize), 
							None
						)
					), 
					arg_data.data_type
				);
				stack_position += arg_data.data_type.size();
			}

			self.instr_mov(&placeholder, &argument);
			// println!("{}\n\n", self.text_segment);
		}

		self.instr_call(&function.identifier);

		for register in allocated_registers
		{
			self.reg_alloc_free(register);
		}

		if function.parameters_stack_size != 0
		{
			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::U64), 
				&Placeholder::new(PlaceholderKind::Integer(stack_position as u64), Type::U64)
			);
		}
	}

	fn store_parameters_sys_v_abi_x86_64(&mut self, function: &Function)
	{
		let mut integer_parameters: u8 = 0;
		let mut float_parameters: u8 = 0;

		for parameter in &function.locals
		{
			if parameter.attributes & attribute::FUNCTION_PARAMETER == 0
			{
				break;
			}

			let source;
			let destination = Placeholder::new(
				PlaceholderKind::Location(
					LocationExpr::new(
						LocationExprPart::Reg(Register::RBP), 
						LocationExprPart::Offset(parameter.location),
						None, 
					)
				), 
				parameter.data_type
			);
			
			if parameter.data_type.is_integer() && integer_parameters < 6
			{
				let register = Self::int_argument_2_register_sys_v_abi_x86_64(integer_parameters, parameter.data_type.size());
				source = Placeholder::new(PlaceholderKind::Reg(register), parameter.data_type);
				integer_parameters += 1;
			} else if !parameter.data_type.is_integer() && float_parameters < 15
			{
				let register = Self::float_argument_2_register_sys_v_abi_x86_64(float_parameters);
				source = Placeholder::new(PlaceholderKind::Reg(register), parameter.data_type);
				float_parameters += 1;
			} else
			{
				break;
			}
			self.instr_mov(&destination, &source);
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

	fn float_argument_2_register_sys_v_abi_x86_64(argument: u8) -> Register
	{
		return Register::try_from(Register::XMM1 as u8 + argument).unwrap();
	}
}