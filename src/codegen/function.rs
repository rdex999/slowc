use super::*;

impl<'a> CodeGen<'a>
{
	pub fn gen_function_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo) -> Option<Placeholder>
	{
		let function = &self.ir.functions[function_call_info.index as usize];
		
		self.reg_alloc_save_used();

		if function.attributes & attribute::CDECL != 0
		{
			self.gen_cdecl_call(locals, function_call_info);
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

	fn gen_cdecl_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo)
	{
		let function = &self.ir.functions[function_call_info.index as usize];

		if function.parameters_stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), 8), 
				&Placeholder::new(PlaceholderKind::U64(function.parameters_stack_size as u64), 8)
			);
		}

		let mut current_location_in_stack: usize = function.parameters_stack_size;
		for argument in function_call_info.arguments.iter().rev()
		{
			let expr = self.gen_expression(argument, locals);
			
			current_location_in_stack -= expr.size as usize;

			let destination = Placeholder::new(
				PlaceholderKind::Location(LocationExpr::new(Register::RSP, None, current_location_in_stack as isize)), 
				expr.size
			);

			self.instr_mov(&destination, &expr);
		}

		self.instr_call(&function.identifier);

		if function.parameters_stack_size != 0
		{
			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), 8), 
				&Placeholder::new(PlaceholderKind::U64(function.parameters_stack_size as u64), 8)
			);
		}
	}
}