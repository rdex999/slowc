use super::*;

impl<'a> CodeGen<'a>
{
	pub fn gen_function_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo) -> Option<Placeholder>
	{
		let function = &self.ir.functions[function_call_info.index as usize];
		
		self.reg_alloc_save_used();

		if function.attributes & attribute::SYS_V_ABI != 0
		{
			self.gen_sys_v_abi_call(locals, function_call_info);
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

	fn gen_sys_v_abi_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo)
	{
		let function = &self.ir.functions[function_call_info.index as usize];

		if function.parameters_stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), OP_QWORD), 
				&Placeholder::new(PlaceholderKind::Constant(function.parameters_stack_size as u64), OP_QWORD)
			);
		}


		if function.parameters_stack_size != 0
		{
			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), OP_QWORD), 
				&Placeholder::new(PlaceholderKind::Constant(function.parameters_stack_size as u64), OP_QWORD)
			);
		}
	}
}