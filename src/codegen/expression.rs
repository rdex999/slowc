use super::*;

impl<'a> CodeGen<'a>
{
	pub fn gen_expression(&mut self, expression: &ExprType, locals: &Vec<Variable>) -> Placeholder
	{
		match expression {
			ExprType::BinExprT(bin_expr) => return self.gen_bin_expr(bin_expr, locals),
		}
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
					PlaceholderKind::Location(LocationExpr::new(Register::RBP, None, variable.location)), 
					OpSize::from_size(variable.data_type.size())
				);
			}, 
			_ => panic!("Dev error! gen_value_access() called with none-writable value. {:#?}", value),
		}
	}

	pub fn gen_function_call(&mut self, locals: &Vec<Variable>, function_call_info: &FunctionCallInfo) -> ()//Option<Placeholder>
	{
		
	}

	fn gen_bin_expr(&mut self, bin_expr: &BinExpr, locals: &Vec<Variable>) -> Placeholder
	{
		match &bin_expr.root
		{
			BinExprPart::Val(value) => return self.gen_value(value, locals),
			BinExprPart::Operation(op) => return self.gen_bin_expr_recurse(op, locals, bin_expr.signed)
		}
	}

	fn gen_value(&mut self, value: &Value, locals: &Vec<Variable>) -> Placeholder 
	{
		match value
		{
			Value::I32(number) => return Placeholder::new(PlaceholderKind::I32(*number), OpSize::Dword),
			Value::Var(_) => return self.gen_value_access(locals, value),
			_ => todo!(),
		}	
	}


	fn gen_bin_operation(&mut self, operator: BinExprOperator, lhs: &Placeholder, rhs: &Placeholder, signed: bool) -> Placeholder 
	{
		// TODO: Make an is_writable function in Placeholder, and check if lhs is a writable, so no need to move to RAX and stuff
		let rax = Register::from_op_size(Register::RAX, lhs.size);
		let destination = Placeholder::new(PlaceholderKind::Reg(rax), lhs.size);
		self.instr_mov(&destination, lhs);

		match operator {
			BinExprOperator::Add => self.instr_add(&destination, rhs),
			BinExprOperator::Sub => self.instr_sub(&destination, rhs),
			BinExprOperator::Mul =>
			{
				if signed
				{
					self.instr_imul(&destination, rhs);
				} else
				{
					todo!();
				}
			},
			BinExprOperator::Div =>
			{
				if signed
				{
					self.instr_idiv(rhs);
				} else
				{
					todo!();
				}
			},
		}
		return Placeholder::new(PlaceholderKind::Reg(rax), lhs.size);
	}

	fn gen_bin_expr_recurse(&mut self, operation: &Box<BinExprOperation>, locals: &Vec<Variable>, signed: bool) -> Placeholder
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
						return self.gen_bin_operation(operation.operator, &lhs, &rhs, signed);
					},

					BinExprPart::Operation(op) =>
					{
						let rhs = self.gen_bin_expr_recurse(&op, locals, signed);
						let register = self.reg_alloc_allocate(rhs.size.bytes()).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(register), rhs.size);
						self.instr_mov(
							&rhs_placeholder, 
							&rhs
						);
						
						let lhs = self.gen_value(lhs, locals);

						let result = self.gen_bin_operation(operation.operator, &lhs, &rhs_placeholder, signed);
						self.reg_alloc_free(register);
						return result;
					}
				}
			},

			BinExprPart::Operation(op) =>
			{
				let lhs = self.gen_bin_expr_recurse(&op, locals, signed);
				let register = self.reg_alloc_allocate(lhs.size.bytes()).unwrap();
				let lhs_placeholder = &Placeholder::new(PlaceholderKind::Reg(register), lhs.size);
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
						result = self.gen_bin_operation(operation.operator, &lhs_placeholder, &rhs, signed);
					},

					BinExprPart::Operation(rhs_op) => 
					{
						let rhs = self.gen_bin_expr_recurse(rhs_op, locals, signed);
						let rhs_reg = self.reg_alloc_allocate(rhs.size.bytes()).unwrap();
						let rhs_placeholder = Placeholder::new(PlaceholderKind::Reg(rhs_reg), rhs.size);
						self.instr_mov(&rhs_placeholder, &rhs);
						result = self.gen_bin_operation(operation.operator, &lhs_placeholder, &rhs_placeholder, signed);
						self.reg_alloc_free(rhs_reg);
					}
				}
				self.reg_alloc_free(register);
				return result;
			}
		}
	}
}