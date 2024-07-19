use crate::ast::*;

pub struct LocalVariables
{
	variables: HashMap<String, Variable>
}

impl LocalVariables
{
	pub fn new() -> Self
	{
		return Self {
			variables: HashMap::new()
		};
	}

	pub fn add_variable(&mut self, identifier: String, data_type: Type) -> Variable
	{
		let var = Variable::new(data_type);	
		self.variables.insert(identifier, var.clone());
		return var;
	}

	// pub fn get_variable(&self, identifier: &str) -> Option<&Variable>
	// {
	// 	return self.variables.get(identifier);
	// }

	pub fn into_var_array(&self) -> Vec<Variable>
	{
		return self.variables.values().cloned().collect();
	}

}