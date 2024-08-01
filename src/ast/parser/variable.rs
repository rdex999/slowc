use crate::ast::*;

pub struct LocalVariables
{
	index: u8,
	variables: HashMap<String, Variable>
}

pub struct LocalVariablesInfo
{
	pub vars: Vec<Variable>,
	
	// The amount of bytes to subtract from the RSP register
	pub stack_size: usize,			
}

impl LocalVariablesInfo
{
	pub fn new(vars: Vec<Variable>, stack_size: usize) -> Self
	{
		return Self {
			vars,
			stack_size
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

	pub fn get_variables_info(self) -> LocalVariablesInfo
	{
		let mut array: Vec<Variable> = self.variables.into_values().collect();
		let mut total_size: usize = 0;
		array.sort_by_cached_key(|var| {
			if var.attributes & attribute::FUNCTION_PARAMETER == 0
			{
				total_size += var.data_type.size() as usize;
			}
			return var.index;
		});

		let mut current_location = total_size as isize * -1;
		let mut parameters_location: isize = 8 + 8;		/* First thing on the stack is the base pointer, then the return address, so skip them */
		for variable in &mut array
		{
			if variable.attributes & attribute::FUNCTION_PARAMETER != 0
			{
				variable.location = parameters_location;
				parameters_location += variable.data_type.size() as isize;
				continue;
			}
			variable.location = current_location;
			current_location += variable.data_type.size() as isize;
		}
		return LocalVariablesInfo::new(
			array, 
			total_size
		);
	}
}