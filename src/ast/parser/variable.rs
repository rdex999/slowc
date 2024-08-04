use crate::ast::*;

pub struct LocalVariables
{
	index: u8,
	variables: HashMap<String, Variable>,
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
	pub fn new() -> Self
	{
		return Self {
			index: 0,
			variables: HashMap::new(),
		};
	}

	pub fn add_variable(&mut self, identifier: String, attributes: AttributeType, data_type: Type) -> Variable
	{
		let var = Variable::new(data_type, attributes, self.index);	
		self.index += 1;
		self.variables.insert(identifier, var.clone());
		return var;
	}

	pub fn get_variable(&self, identifier: &str) -> Option<&Variable>
	{
		return self.variables.get(identifier);
	}

	pub fn get_variable_by_index(&self, index: u8) -> Option<&Variable>
	{
		return self.variables.values().find(|var| var.index == index);
	}

	pub fn get_variable_count(&self) -> u8
	{
		return self.index;
	}

	pub fn get_variables_info(self, function_attributes: AttributeType) -> LocalVariablesInfo
	{
		let mut array: Vec<Variable> = self.variables.into_values().collect();
		array.sort_by_cached_key(|var| var.index);

		if function_attributes & attribute::SYS_V_ABI != 0
		{
			return Self::update_var_info_sys_v_abi_x86_64(array);
		} else
		{
			todo!("Unsupported calling convenction.");
		}
	}

	fn update_var_info_sys_v_abi_x86_64(mut variables: Vec<Variable>) -> LocalVariablesInfo
	{
		// Location counter for local variables, also used for determining the functions stack size
		let mut stack_var_position: isize = 0;

		// Location counter for parameters that were passed on the stack 
		let mut stack_parameter_position: usize = 8 + 8;		/* Return address(8), base pointer(8) */

		// Count parameters that were passed in rdi, rsi, rdx, rcx, r8, r9
		let mut integer_parameters: u8 = 0;

		for variable in variables.iter_mut()
		{
			// If the current variable is not a function parameter
			if variable.attributes & attribute::FUNCTION_PARAMETER == 0
			{
				stack_var_position -= variable.data_type.size() as isize;	
				variable.location = stack_var_position;
				continue;
			}

			if variable.data_type.is_integer()	
			{
				if integer_parameters < 6	/* rdi, rsi, rdx, rcx, r8, r9 (6 registers)*/
				{
					stack_var_position -= variable.data_type.size() as isize;
					variable.location = stack_var_position;
					integer_parameters += 1;
				} else
				{
					variable.location = stack_parameter_position as isize;
					stack_parameter_position += variable.data_type.size() as usize;
				}
			} else
			{
				todo!("Add floating point shit");
			}
		}

		return LocalVariablesInfo::new(
			variables, 
			(stack_var_position * -1) as usize, 
			stack_parameter_position - 8 - 8
		);
	}
}