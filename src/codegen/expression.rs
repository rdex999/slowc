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

	fn gen_bin_expr(&mut self, bin_expr: &BinExpr, locals: &Vec<Variable>) -> Placeholder
	{
		match &bin_expr.root
		{
			BinExprPart::Val(value) => return self.gen_value(value, locals),
			BinExprPart::Operation(op) => return self.gen_bin_expr_recurse(op, locals)
		}
	}

	fn gen_bin_operation(&mut self, operator: BinExprOperator, lhs: &Placeholder, rhs: &Placeholder) -> Placeholder 
	{
		// TODO: Make an is_writable function in Placeholder, and check if lhs is a writable, so no need to move to RAX and stuff
		let dst_register = Register::from_op_size(Register::default_for_type(lhs.data_type), lhs.data_type.size());
		let destination = Placeholder::new(PlaceholderKind::Reg(dst_register), lhs.data_type);
		self.instr_mov(&destination, lhs);

		match operator {
			BinExprOperator::Add 	=> self.instr_add(&destination, rhs),
			BinExprOperator::Sub 	=> self.instr_sub(&destination, rhs),
			BinExprOperator::Mul 	=> self.instr_mul(&destination, rhs),
			BinExprOperator::Div 	=> self.instr_div(&destination, rhs),
			BinExprOperator::BoolEq => { self.instr_cmp(&destination, rhs); self.instr_sete(&destination); },
		}
		return destination;
	}

	fn gen_bin_expr_recurse(&mut self, operation: &Box<BinExprOperation>, locals: &Vec<Variable>) -> Placeholder
	{
		match &operation.lhs
		{
			BinExprPart::Val(lhs) =>
			{
				match &operation.rhs
				{
					BinExprPart::Val(rhs) =>
					{
						let lhs = self.gen_value(lhs, locals);
						let rhs = self.gen_value(rhs, locals);
						return self.gen_bin_operation(operation.operator, &lhs, &rhs);
					},

					BinExprPart::Operation(op) =>
					{
						let rhs = self.gen_bin_expr_recurse(&op, locals);
						let register = self.reg_alloc_allocate(rhs.data_type).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(register), rhs.data_type);
						self.instr_mov(
							&rhs_placeholder, 
							&rhs
						);
						
						let lhs = self.gen_value(lhs, locals);

						let result = self.gen_bin_operation(operation.operator, &lhs, &rhs_placeholder);
						self.reg_alloc_free(register);
						return result;
					}
				}
			},

			BinExprPart::Operation(op) =>
			{
				let lhs = self.gen_bin_expr_recurse(&op, locals);
				let register = self.reg_alloc_allocate(lhs.data_type).unwrap();
				let lhs_placeholder = &Placeholder::new(PlaceholderKind::Reg(register), lhs.data_type);
				let result;
				self.instr_mov(
					&lhs_placeholder, 
					&lhs
				);

				match &operation.rhs
				{
					BinExprPart::Val(value) => 
					{
						let rhs = self.gen_value(value, locals);
						let rhs_reg = self.reg_alloc_allocate(rhs.data_type).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(rhs_reg), rhs.data_type);
						self.instr_mov(&rhs_placeholder, &rhs);

						result = self.gen_bin_operation(operation.operator, &lhs_placeholder, &rhs_placeholder);
						self.reg_alloc_free(rhs_reg);
					},

					BinExprPart::Operation(rhs_op) => 
					{
						let rhs = self.gen_bin_expr_recurse(rhs_op, locals);
						let rhs_reg = self.reg_alloc_allocate(rhs.data_type).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(rhs_reg), rhs.data_type);
						self.instr_mov(&rhs_placeholder, &rhs);
						result = self.gen_bin_operation(operation.operator, &lhs_placeholder, &rhs_placeholder);
						self.reg_alloc_free(rhs_reg);
					}
				}
				self.reg_alloc_free(register);
				return result;
			}
		}
	}
}