use crate::ast::*;

pub struct LocalVariables
{
	index: usize,
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

	pub fn add_variable(&mut self, identifier: String, data_type: Type) -> Variable
	{
		let var = Variable::new(data_type, self.index);	
		self.index += 1;
		self.variables.insert(identifier, var.clone());
		return var;
	}

	pub fn get_variable(&self, identifier: &str) -> Option<&Variable>
	{
		return self.variables.get(identifier);
	}

	pub fn into_var_array(self) -> Vec<Variable>
	{
		let mut array: Vec<Variable> = self.variables.values().cloned().collect();
		array.sort_by_cached_key(|var| var.index);
		return array;
	}
}