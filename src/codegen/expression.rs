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
			Value::I8(number) 			=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::I8)),
			Value::U8(number) 			=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::U8)),
			Value::I16(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::I16)),
			Value::U16(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::U16)),
			Value::I32(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::I32)),
			Value::U32(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::U32)),
			Value::I64(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::I64)),
			Value::U64(number) 		=> Placeholder::new(PlaceholderKind::Integer(*number as u64), Type::new(TypeKind::U64)),
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
			Value::Dereference(info)				=> self.gen_pointer_dereference(locals, info),
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
			
			let destination = Placeholder::new(PlaceholderKind::Reg(Register::AL), Type::new(TypeKind::U8));
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
			let dst_register = Register::default_for_type(lhs.data_type);
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

	fn gen_bin_self_operation(&mut self, locals: &Vec<Variable>, operation: &BinExprSelfOperation) -> Placeholder
	{
		let mut expression = self.gen_bin_expr_recurse(locals, &operation.expression);
		if operation.operator != BinExprOperator::AddressOf
		{
			if !expression.is_register()
			{
				let new_placeholder = Placeholder::new(
					PlaceholderKind::Reg(Register::default_for_type(expression.data_type)), 
					expression.data_type
				);
	
				self.instr_mov(&new_placeholder, &expression);
				expression = new_placeholder;
			}
		}

		match operation.operator {
			BinExprOperator::BitwiseNot 	=> self.instr_not(&expression),
			BinExprOperator::BoolNot		=> {self.instr_test(&expression, &expression); self.instr_setz(&expression); },
			BinExprOperator::AddressOf		=>
			{
				let rax = Placeholder::new(PlaceholderKind::Reg(Register::RAX), Type::new(TypeKind::U64));
				self.instr_lea(&rax, &expression);
				return rax;
			}
			_ => panic!("gen_bin_expr_recurse(), BinExprPart::SelfOperation.operator, not a binary operator.\n{:#?}", operation),
		}
		return expression;
	}
	
	fn gen_bin_expr(&mut self, bin_expr: &BinExpr, locals: &Vec<Variable>) -> Placeholder
	{
		return self.gen_bin_expr_recurse(locals, &bin_expr.root)		
	}
	
	fn gen_bin_expr_recurse(&mut self, locals: &Vec<Variable>, expr_part: &BinExprPart) -> Placeholder
	{
		match expr_part {
			BinExprPart::Val(value) => return self.gen_value(value, locals),
			BinExprPart::TypeCast(type_cast_info) => return self.gen_type_cast(locals, type_cast_info),
			BinExprPart::SelfOperation(operation) => return self.gen_bin_self_operation(locals, operation),
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
		}
	}

	fn gen_type_cast(&mut self, locals: &Vec<Variable>, type_cast_info: &TypeCastInfo) -> Placeholder
	{
		let mut expression = self.gen_bin_expr_recurse(locals, &type_cast_info.expression);

		if type_cast_info.from_type.kind == TypeKind::Pointer && type_cast_info.from_type.kind == TypeKind::Pointer
		{
			return expression.of_type(type_cast_info.into_type);
		}

		if type_cast_info.from_type.is_integer() && type_cast_info.into_type.is_integer()
		{
			if !type_cast_info.from_type.is_signed() && !type_cast_info.into_type.is_signed()
			{
				if type_cast_info.into_type.size() < type_cast_info.from_type.size()
				{
					return expression.of_type(type_cast_info.into_type);
				}

				// Here we know that sizeof(into_type) > sizeof(from_type), 
				// so the casting is just a bitwise AND operation (to remove crap that was in the register)
				let rax = Placeholder::new(
					PlaceholderKind::Reg(Register::RAX.of_size(type_cast_info.into_type.size())), 
					type_cast_info.into_type
				);

				match type_cast_info.into_type.kind
				{
					TypeKind::U16 | TypeKind::U32 => self.instr_movzx(&rax, &expression),
					TypeKind::U64 | TypeKind::Pointer =>
					{
						if type_cast_info.from_type == Type::new(TypeKind::U16) || type_cast_info.from_type == Type::new(TypeKind::U8)
						{
							self.instr_movzx(&rax, &expression);
						} else
						{
							// TODO: When switching to the GNU assembler, replace this crap with instr_and();
							if expression.is_register_eq(Register::RAX.of_size(expression.data_type.size()))
							{
								let allocated_reg = self.reg_alloc_allocate(expression.data_type).unwrap();
								let placeholder = Placeholder::new(PlaceholderKind::Reg(allocated_reg), expression.data_type);
								self.instr_mov(
									&placeholder,
									&expression 
								);
								self.reg_alloc_free(allocated_reg);
								expression = placeholder;
							}
							self.instr_xor(&rax, &rax);
							self.instr_mov(&rax.of_type(expression.data_type), &expression);

						} 
					},
					_ => panic!("type_cast_info.into_type cannot be U8 as it must be greater than type_cast_info.from_type"),
				}
				
				return rax;

			// If one is signed and the other is unsigned
			} else
			{
				if type_cast_info.into_type.size() <= type_cast_info.from_type.size()
				{
					return expression.of_type(type_cast_info.into_type);
				}

				// if true, then from_type is unsigned. Also we know that sizeof(into_type) > sizeof(from_type)
				if type_cast_info.into_type.is_signed()
				{
					let rax = Placeholder::new(
						PlaceholderKind::Reg(Register::RAX.of_size(type_cast_info.into_type.size())), 
						type_cast_info.into_type
					);

					match type_cast_info.into_type.kind
					{
						TypeKind::I16 | TypeKind::I32 => self.instr_movzx(&rax, &expression),
						TypeKind::I64 =>
						{
							if type_cast_info.from_type == Type::new(TypeKind::I16) || type_cast_info.from_type == Type::new(TypeKind::I8)
							{
								self.instr_movsx(&rax, &expression);
							} else
							{
								if !expression.is_register_eq(Register::RAX.of_size(expression.data_type.size()))
								{
									self.instr_mov(
										&Placeholder::new(
											PlaceholderKind::Reg(Register::RAX.of_size(expression.data_type.size())), 
											expression.data_type
										), 
										&expression
									);
								}
								self.instr_cdqe();
							} 
						},
						_ => panic!("type_cast_info.into_type cannot be U8 as it must be greater than type_cast_info.from_type"),
					}
					
					return rax;	

				// Means into_type is unsigned and from_type is signed and sizeof(into_type) > sizeof(from_type)
				} else
				{
					let rax = Placeholder::new(
						PlaceholderKind::Reg(Register::RAX.of_size(type_cast_info.into_type.size())), 
						type_cast_info.into_type
					);

					if expression.is_register_eq(Register::RAX.of_size(expression.data_type.size()))
					{
						let allocated_reg = self.reg_alloc_allocate(expression.data_type).unwrap();
						let placeholder = Placeholder::new(PlaceholderKind::Reg(allocated_reg), expression.data_type);
						self.instr_mov(&placeholder, &expression);
						self.reg_alloc_free(allocated_reg);
						expression = placeholder
					}

					self.instr_xor(&rax, &rax);
					self.instr_mov(
						&Placeholder::new(PlaceholderKind::Reg(Register::RAX.of_size(expression.data_type.size())), expression.data_type), 
						&expression
					);

					return rax;
				}
			}

		// Float into integer
		} else if type_cast_info.into_type.is_integer() && !type_cast_info.from_type.is_integer()
		{
			let rax = Placeholder::new(PlaceholderKind::Reg(Register::RAX.of_size(type_cast_info.into_type.size())), type_cast_info.into_type);
			self.instr_cvttsf2si(
					&rax,
				&expression
			);
			return rax;
		// Integer into float
		} else if !type_cast_info.into_type.is_integer() && type_cast_info.from_type.is_integer()
		{
			let xmm0 = Placeholder::new(PlaceholderKind::Reg(Register::XMM0), type_cast_info.into_type);
			self.instr_cvtsi2sf(
				&xmm0, 
				&expression
			);
			return xmm0;

		// F32 into F64 or F64 into F32
		} else
		{
			let xmm0 = Placeholder::new(PlaceholderKind::Reg(Register::XMM0), type_cast_info.into_type);
			self.instr_cvtsf2sf(
				&xmm0,	
				&expression
			);
			return xmm0;
		}
	}

	fn gen_pointer_dereference(&mut self, locals: &Vec<Variable>, dereference_info: &DereferenceInfo) -> Placeholder
	{
		let expression = self.gen_expression(&dereference_info.expression, locals);
		let mut result = Placeholder::new(PlaceholderKind::Reg(Register::RAX), dereference_info.data_type);
		let start_dereference_cnt;
		if expression.is_location()
		{
			start_dereference_cnt = 0;
			self.instr_mov(&result, &expression);
		} else
		{
			start_dereference_cnt = 1;
			self.instr_mov(
				&result, 
				&Placeholder::new(PlaceholderKind::Location(LocationExpr::from_placeholder(&expression)), expression.data_type)
			);
			result = result.of_type(result.data_type.dereference(1));
		}

		for _ in start_dereference_cnt..dereference_info.dereference_count
		{
			self.instr_mov(
				&result, 
				&Placeholder::new(PlaceholderKind::Location(LocationExpr::from_placeholder(&result)), result.data_type)
			);
			result = result.of_type(result.data_type.dereference(1));
		}

		return result;
	}
}