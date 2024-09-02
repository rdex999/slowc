use super::*;

impl<'a> CodeGen<'a>
{
	pub fn gen_expression(&mut self, expression: &BinExpr, locals: &Vec<Variable>) -> Placeholder
	{
		return self.gen_bin_expr(expression, locals);
	}
	
	// Will return a pointer to the result
	pub fn gen_value_access(&mut self, locals: &Vec<Variable>, value: &Value) -> Placeholder
	{
		match value
		{
			Value::Var(variable_index) =>
			{
				let variable = locals[*variable_index as usize];
				return Placeholder::new(
					PlaceholderKind::Location(LocationExpr::new(
						LocationExprPart::Reg(Register::RBP),
						LocationExprPart::Offset(variable.location), 
						None,
					)), 
					variable.data_type
				);
			}, 
			_ => panic!("Dev error! gen_value_access() called with none-writable value. {:#?}", value),
		}
	}

	fn gen_value(&mut self, value: &Value, locals: &Vec<Variable>) -> Placeholder 
	{
		return match value
		{
			Value::I8(number) 			=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::I8),
			Value::U8(number) 			=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::U8),
			Value::I16(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::I16),
			Value::U16(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::U16),
			Value::I32(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::I32),
			Value::U32(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::U32),
			Value::I64(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::I64),
			Value::U64(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::U64),
			Value::F64(_) | Value::F32(_) 	=> 
			{
				let lable = self.decl_var_data_seg(value);
				Placeholder::new(
					PlaceholderKind::Location(
						LocationExpr::new(
							LocationExprPart::Labl(lable), 
							LocationExprPart::Offset(0), 
							None
						),
					),
					self.value_type(value, locals),
				)
			}
			Value::Var(_) 											=> self.gen_value_access(locals, value),
			Value::FuncCall(function_call_info) 	=> self.gen_function_call(locals, function_call_info).unwrap(),
		}	
	}
	
	fn gen_bin_operation(&mut self, operator: BinExprOperator, lhs: &Placeholder, rhs: &Placeholder) -> Placeholder 
	{
		if operator.is_boolean()
		{
			
			if operator == BinExprOperator::BoolAnd || operator == BinExprOperator::BoolOr
			{
				// TODO: Make an is_writable function in Placeholder, and check if lhs is a writable, so no need to move to RAX and stuff
				let dst_register = Register::from_op_size(Register::RAX, lhs.data_type.size());
				let destination = Placeholder::new(PlaceholderKind::Reg(dst_register), lhs.data_type);
				self.instr_mov(&destination, lhs);
				match operator
				{
					BinExprOperator::BoolAnd => self.instr_and(&destination, rhs),
					BinExprOperator::BoolOr  => self.instr_or(&destination, rhs),
					_ => panic!("Rust doesnt work"),
				}
				return destination;
			}
			
			let destination = Placeholder::new(PlaceholderKind::Reg(Register::AL), Type::U8);
			self.instr_cmp(lhs, rhs); 
			
			match operator
			{
				BinExprOperator::BoolEq 		=> self.instr_sete(&destination),
				BinExprOperator::BoolNotEq		=> self.instr_setne(&destination),
				BinExprOperator::BoolGreater	=> self.instr_setg(&destination),
				BinExprOperator::BoolLess		=> self.instr_setl(&destination),
				BinExprOperator::BoolGreaterEq	=> self.instr_setge(&destination),
				BinExprOperator::BoolLessEq		=> self.instr_setle(&destination),
				_ => panic!("Rust doesnt work"),
			}
			return destination;
		} else
		{
			// TODO: Make an is_writable function in Placeholder, and check if lhs is a writable, so no need to move to RAX and stuff
			let dst_register = Register::from_op_size(Register::default_for_type(lhs.data_type), lhs.data_type.size());
			let destination = Placeholder::new(PlaceholderKind::Reg(dst_register), lhs.data_type);
			self.instr_mov(&destination, lhs);
			
			match operator {
				BinExprOperator::BitwiseOr 			=> self.instr_or(&destination, rhs),
				BinExprOperator::BitwiseXor 		=> self.instr_xor(&destination, rhs),
				BinExprOperator::BitwiseAnd 		=> self.instr_and(&destination, rhs),
				BinExprOperator::BitwiseRightShift 	=> self.instr_shr(&destination, rhs),
				BinExprOperator::BitwiseLeftShift 	=> self.instr_shl(&destination, rhs),
				BinExprOperator::Add 				=> self.instr_add(&destination, rhs),
				BinExprOperator::Sub 				=> self.instr_sub(&destination, rhs),
				BinExprOperator::Mul 				=> self.instr_mul(&destination, rhs),
				BinExprOperator::Div 				=> self.instr_div(&destination, rhs, false),
				BinExprOperator::Modulo 			=> self.instr_div(&destination, rhs, true),
				_ => panic!("Rust doesnt work."),
			}
			
			return destination;
		}
	}
	
	fn gen_bin_expr(&mut self, bin_expr: &BinExpr, locals: &Vec<Variable>) -> Placeholder
	{
		return self.gen_bin_expr_recurse(locals, &bin_expr.root)		
	}
	
	fn gen_bin_expr_recurse(&mut self, locals: &Vec<Variable>, expr_part: &BinExprPart) -> Placeholder
	{
		match expr_part {
			BinExprPart::Val(value) => return self.gen_value(value, locals),
			BinExprPart::Operation(operation) =>
			{
				let mut lhs_allocated_reg = None;
				let mut lhs = self.gen_bin_expr_recurse(locals, &operation.lhs);
				if lhs.is_register()
				{
					lhs_allocated_reg = Some(self.reg_alloc_allocate(lhs.data_type).unwrap());
					let new_lhs = Placeholder::new(PlaceholderKind::Reg(lhs_allocated_reg.unwrap()), lhs.data_type);
					self.instr_mov(&new_lhs, &lhs);
					lhs = new_lhs;
				}

				let mut rhs_allocated_reg = None;
				let mut rhs = self.gen_bin_expr_recurse(locals, &operation.rhs);
				if rhs.is_register()
				{
					rhs_allocated_reg = Some(self.reg_alloc_allocate(rhs.data_type).unwrap());
					let new_rhs = Placeholder::new(PlaceholderKind::Reg(rhs_allocated_reg.unwrap()), rhs.data_type);
					self.instr_mov(&new_rhs, &rhs);
					rhs = new_rhs;
				}	
				
				let result = self.gen_bin_operation(operation.operator, &lhs, &rhs);

				if let Some(allocated_register) = lhs_allocated_reg
				{
					self.reg_alloc_free(allocated_register);
				}

				if let Some(allocated_register) = rhs_allocated_reg
				{
					self.reg_alloc_free(allocated_register);
				}
				return result;
			},
			_ => todo!("Implement type casts."),
		}

	}
}