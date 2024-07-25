use crate::ast::*;

pub struct LocalVariables
{
	index: u8,
	variables: HashMap<String, Variable>
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

	pub fn into_var_array(self) -> Vec<Variable>
	{
		let mut array: Vec<Variable> = self.variables.into_values().collect();
		array.sort_by_cached_key(|var| var.index);
		return array;
	}
}