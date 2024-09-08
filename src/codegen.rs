mod common;
mod instructions;
mod register_allocator;
mod expression;
mod function;

use instructions::*;
use register_allocator::*;
use super::{ast::*, CompileError, print_err};

const OUT_OBJECT_FILE_PATH: &str = "/tmp/slowc_compiled.obj";
const OUT_ASM_FILE_PATH: &str = "/tmp/slowc_compiled.asm";

pub struct CodeGen<'a>
{
	ir: &'a Root,
	registers: [RegisterInfo; ALLOCATABLE_REGS_COUNT],
	attribute_segment: String,
	data_segment: String,
	text_segment: String,

	data_seg_var_index: usize,
	text_seg_var_index: usize,
}

impl<'a> CodeGen<'a>
{

	pub fn new(ir: &'a Root) -> Self
	{
		let attribute_segment = String::from("bits 64");
		let data_segment = String::from("\nsegment .data");
		let text_segment = String::from("\nsegment .text");

		return Self {
			ir,
			registers: Self::reg_alloc_init(),
			attribute_segment,
			data_segment,
			text_segment,
			data_seg_var_index: 0,
			text_seg_var_index: 0,
		};
	}
	
	pub fn generate<'b>(mut self) -> &'b str
	{
		for function in &self.ir.functions
		{
			self.gen_function(&function);
		}

		if cfg!(debug_assertions)
		{
			self.reg_alloc_check_leaks();
		}

		let mut final_asm = String::with_capacity(self.attribute_segment.len() + self.data_segment.len() + self.text_segment.len() + 1);
		final_asm.push_str(&self.attribute_segment);
		final_asm.push_str(&self.data_segment);
		final_asm.push_str(&self.text_segment);
		final_asm.push('\n');

		std::fs::write(OUT_ASM_FILE_PATH, final_asm).unwrap_or_else(|err| {
			print_err!(CompileError::FileWriteError(OUT_ASM_FILE_PATH), "Could not write to temporary assembly file. {err}");
		});

		std::process::Command::new("nasm")
			.args(["-f", "elf64"])
			.arg("-g")
			.args(["-o", OUT_OBJECT_FILE_PATH])
			.arg(OUT_ASM_FILE_PATH)
			.spawn()
			.expect("Dev error! failed to execute nasm.");

		return &OUT_OBJECT_FILE_PATH;
	}

	
	fn gen_scope(&mut self, scope: &Scope, locals: &Vec<Variable>)
	{
		if scope.stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::new(TypeKind::U64)), 
				&Placeholder::new(PlaceholderKind::Integer(scope.stack_size as u64), Type::new(TypeKind::U64))
			);
		}

		for statement in &scope.statements
		{
			self.gen_statement(&statement, locals);
		}

		if scope.stack_size != 0
		{
			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::new(TypeKind::U64)), 
				&Placeholder::new(PlaceholderKind::Integer(scope.stack_size as u64), Type::new(TypeKind::U64))
			);
		}
	}

	fn gen_statement(&mut self, statement: &Statement, locals: &Vec<Variable>)
	{
		match statement
		{
			Statement::Scope(scope)									=> self.gen_scope(scope, locals),
			Statement::Assign(assign_data) 					=> self.gen_assign_stmt(assign_data, locals),
			Statement::FunctionCall(function_call_info) 	=> { self.gen_function_call(locals, function_call_info); } 
			Statement::Return(expression) 				=> self.gen_return_stmt(locals, expression),
			Statement::If(if_info)									=> self.gen_if_stmt(locals, if_info),
			Statement::For(for_info) 							=> self.gen_for_stmt(locals, for_info),
		}
	}

	fn gen_assign_stmt(&mut self, assign_data: &VarUpdateInfo, locals: &Vec<Variable>)
	{
		let mut expression = self.gen_expression(&assign_data.value, locals);
		let mut allocated_reg = None;
		if expression.is_register()
		{
			allocated_reg = Some(self.reg_alloc_allocate(expression.data_type).unwrap());

			let expr_placeholder = Placeholder::new(
				PlaceholderKind::Reg(allocated_reg.unwrap()), 
				expression.data_type
			);
			self.instr_mov(&expr_placeholder, &expression);
			expression = expr_placeholder;
		}

		let destination = self.gen_value_access(locals, &assign_data.destination);

		self.instr_mov(&destination, &expression);

		if let Some(register) = allocated_reg
		{
			self.reg_alloc_free(register);
		}
	}

	fn gen_return_stmt(&mut self, locals: &Vec<Variable>, expression: &Option<BinExpr>)
	{
		let expr;
		if let Some(exp) = expression
		{
			expr = exp;
		} else
		{
			self.gen_function_return();
			return;
		}

		let expr_placeholder = self.gen_expression(expr, locals);
		let return_register = Register::default_for_type(expr_placeholder.data_type);

		// I hate Rust
		if let PlaceholderKind::Reg(reg) = expr_placeholder.kind
		{
			if reg != return_register 
			{
				self.instr_mov(
					&Placeholder::new(PlaceholderKind::Reg(return_register), expr_placeholder.data_type), 
					&expr_placeholder
				);
			}
		} else 
		{
			self.instr_mov(
				&Placeholder::new(PlaceholderKind::Reg(return_register), expr_placeholder.data_type), 
				&expr_placeholder
			);
		}

		self.gen_function_return();
	}

	fn gen_if_stmt(&mut self, locals: &Vec<Variable>, if_info: &IfInfo)
	{
		let false_lable = self.generate_text_seg_lable();
		let end_else = if let Some(_) = if_info.else_block { Some(self.generate_text_seg_lable()) } else { None };

		#[cfg(debug_assertions)]
		self.write_text_segment(&format!("\n\t; Start if statement {}, expression:", false_lable.index));

		let expression = self.gen_expression(&if_info.condition, locals).of_type(Type::new(TypeKind::U8));

		self.instr_test(&expression, &expression);
		self.instr_jz(false_lable);

		#[cfg(debug_assertions)]
		self.write_text_segment(&format!("\n\t; If statement {}, then:", false_lable.index));

		self.gen_statement(&if_info.then_block, locals);

		if let Some(_) = if_info.else_block
		{
			self.instr_jmp(end_else.unwrap());

			#[cfg(debug_assertions)]
			self.write_lable_text_seg(&format!(" \t; Skip else block of if statement {}", false_lable.index));
		}

		self.write_lable(false_lable);
		
		if let Some(statement) = &if_info.else_block
		{
			#[cfg(debug_assertions)]
			self.write_text_segment(&format!(" \t; Else block of if statement {}", false_lable.index));

			self.gen_statement(statement, locals);
			
			self.write_lable(end_else.unwrap());
		}

		#[cfg(debug_assertions)]
		self.write_text_segment(&format!(" \t; End if statement {}", false_lable.index));
	}

	fn gen_for_stmt(&mut self, locals: &Vec<Variable>, for_info: &ForLoopInfo)
	{
		let loop_start = self.generate_text_seg_lable();
		let condition_lable = self.generate_text_seg_lable();

		let mut stack_size = for_info.stack_size;
		if let Statement::Scope(scope) = &*for_info.code_block 	/* Borrow the dereference, sure */
		{
			stack_size += scope.stack_size;
		}

		if stack_size != 0
		{
			self.instr_sub(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::new(TypeKind::U64)), 
				&Placeholder::new(PlaceholderKind::Integer(stack_size as u64), Type::new(TypeKind::U64)),
			);
		}

		if let Some(initializer_stmt) = &for_info.initializer
		{
			#[cfg(debug_assertions)]
			self.write_text_segment(&format!("\n\t; For loop initializer statement"));

			self.gen_statement(initializer_stmt, locals);
		}

		if let Some(_) = for_info.condition
		{
			self.instr_jmp(condition_lable);
		}
		
		self.write_lable(loop_start);

		#[cfg(debug_assertions)]
		self.write_text_segment(&format!("\t; For loop body"));

		if let Statement::Scope(scope) = &*for_info.code_block 	/* Borrow the dereference, sure */
		{
			for statement in &scope.statements
			{
				self.gen_statement(statement, locals);
			}
		} else
		{
			self.gen_statement(&for_info.code_block, locals);
		}

		if let Some(update_stmt) = &for_info.update
		{
			#[cfg(debug_assertions)]
			self.write_text_segment(&format!("\n\t; For loop update statement"));

			self.gen_statement(update_stmt, locals);
		}

		self.write_lable(condition_lable);

		if let Some(condition_expr) = &for_info.condition
		{
			#[cfg(debug_assertions)]
			self.write_text_segment(&format!("\t; For loop condition expression"));

			let result = self.gen_expression(condition_expr, locals);
			self.instr_test(&result, &result);
			self.instr_jnz(loop_start);
		} else
		{
			self.instr_jmp(loop_start);
		}

		if stack_size != 0
		{
			self.instr_add(
				&Placeholder::new(PlaceholderKind::Reg(Register::RSP), Type::new(TypeKind::U64)), 
				&Placeholder::new(PlaceholderKind::Integer(stack_size as u64), Type::new(TypeKind::U64)),
			);		
		}
	}
}