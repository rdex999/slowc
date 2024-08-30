use crate::{ast::*, print_err, CompileError};

pub struct LocalVariables
{
	index: u8,
	scopes: Vec<usize>,
	next_scope_idx: u8,
	variables: HashMap<String, Vec<Variable>>,
	variables_arr: Vec<Variable>,
	function_attributes: AttributeType,

	stack_var_position: isize, 				// Location counter for local variables, also used for determining the functions stack size
	stack_parameter_position: usize,		// Location counter for parameters that were passed on the stack 
	integer_parameters: u8,					// Count parameters that were passed in rdi, rsi, rdx, rcx, r8, r9
	float_parameters: u8, 					// Count parameters that were passed in xmm1-15
}

pub struct LocalVariablesInfo
{
	pub vars: Vec<Variable>,
	
	// The amount of bytes to subtract from the RSP register
	pub stack_size: usize,

	pub parameters_stack_size: usize,
}

impl LocalVariablesInfo
{
	pub fn new(vars: Vec<Variable>, stack_size: usize, parameters_stack_size: usize) -> Self
	{
		return Self {
			vars,
			stack_size,
			parameters_stack_size,
		};
	}
}

impl LocalVariables
{
	pub fn new(function_attributes: AttributeType) -> Self
	{
		let mut scopes: Vec<usize> = Vec::with_capacity(10);
		scopes.push(0);
		return Self {
			index: 0,
			scopes,
			next_scope_idx: 0,
			variables: HashMap::new(),
			variables_arr: Vec::new(),
			function_attributes,

			stack_var_position: 0,
			stack_parameter_position: 8 + 8,		/* Return address(8), base pointer(8) */
			integer_parameters: 0,
			float_parameters: 0,
		};
	}

	pub fn add_variable(&mut self, identifier: String, attributes: AttributeType, data_type: Type) -> Variable
	{
		let scope;
		if attributes & attribute::FUNCTION_PARAMETER != 0
		{
			scope = 0;
		} else
		{
			scope = self.current_scope();
		}
		
		let mut variable = Variable::new(data_type, attributes, self.index, scope);
		
		if self.function_attributes & attribute::SYS_V_ABI_X86_64 != 0
		{
			self.update_var_info_sys_v_abi_x86_64(&mut variable);
		}else
		{
			panic!("Unimplemented calling convenction used.");
		}
		
		if let Some(vars) = self.variables.get_mut(&identifier)
		{
			if vars[vars.len() - 1].scope == scope
			{
				print_err!(CompileError::Syntax, "Variable \"{identifier}\" was already declared in the current scope.");
			}
			vars.push(variable);
		} else
		{
			self.variables.insert(identifier, Vec::from([variable]));
		}
		self.index += 1;
		self.variables_arr.push(variable);
		*self.scopes.last_mut().unwrap() += variable.data_type.size() as usize;
		return variable;
	}

	pub fn get_variable(&self, identifier: &str) -> Option<&Variable>
	{
		if let Some(vars) = self.variables.get(identifier)
		{
			return Some(&vars[vars.len() - 1]);
		}
		return None;
	}

	pub fn start_scope(&mut self)
	{
		if self.next_scope_idx != 0
		{
			self.scopes.push(0);
		}
		self.advance_scope();
	}

	// End the scope and returns its stack size.
	pub fn end_scope(&mut self) -> usize
	{
		for (identifier, vars) in self.variables.clone().into_iter()
		{
			for (i, variable) in vars.iter().enumerate()
			{
				if variable.scope >= self.current_scope()
				{
					if i == 0
					{
						self.variables.remove(&identifier);
					} else
					{
						let vars = self.variables.get_mut(&identifier).unwrap();
						vars.drain(i..vars.len());
						break;
					}
				}
			}
		}
		let stack_size = if self.scopes.len() == 1 { self.scopes[0] as isize } else { self.scopes.pop().unwrap() as isize };
		self.stack_var_position += stack_size;
		self.next_scope_idx -= 1;
		return stack_size as usize;
	}

	pub fn get_variable_by_index(&self, index: u8) -> Option<&Variable>
	{
		let variable = &self.variables_arr[index as usize];
		if variable.scope > self.current_scope()
		{
			return None;
		}
		return Some(variable);
	}

	pub fn get_variable_count(&self) -> u8
	{
		return self.index;
	}

	pub fn get_variables_info(self) -> LocalVariablesInfo
	{
		return LocalVariablesInfo::new(
			self.variables_arr, 
			self.scopes[0],
			self.stack_parameter_position - 8 - 8
		);
	}

	fn current_scope(&self) -> u8
	{
		return self.next_scope_idx - 1;
	}

	fn advance_scope(&mut self)
	{
		self.next_scope_idx += 1;
	}

	fn update_var_info_sys_v_abi_x86_64(&mut self, variable: &mut Variable)
	{
		// If the variable is not a function parameter
		if variable.attributes & attribute::FUNCTION_PARAMETER == 0
		{
			self.stack_var_position -= variable.data_type.size() as isize;
			variable.location = self.stack_var_position;
			return;
		}

		if variable.data_type.is_integer()	
		{
			if self.integer_parameters < 6	/* rdi, rsi, rdx, rcx, r8, r9 (6 registers)*/
			{
				self.stack_var_position -= variable.data_type.size() as isize;
				variable.location = self.stack_var_position;
				self.integer_parameters += 1;
			} else
			{
				variable.location = self.stack_parameter_position as isize;
				self.stack_parameter_position += variable.data_type.size() as usize;
			}
		} else
		{
			if self.float_parameters < 15		/* XMM1-15 (15 registers) */
			{
				self.stack_var_position -= variable.data_type.size() as isize;
				variable.location = self.stack_var_position;
				self.float_parameters += 1;
			} else
			{
				variable.location = self.stack_parameter_position as isize;
				self.stack_parameter_position += variable.data_type.size() as usize;
			}
		}
	}
}