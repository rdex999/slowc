use super::*;

impl<'a> CodeGen<'a>
{
	pub fn write_attribute_segment(&mut self, data: &str)
	{
		self.attribute_segment.push_str(data);
	}

	pub fn _write_data_segment(&mut self, data: &str)
	{
		self.data_segment.push_str(data);
	}

	pub fn write_text_segment(&mut self, data: &str)
	{
		self.text_segment.push_str(data);
	}

	pub fn decl_attribute(&mut self, identifier: &str, attr: attribute::AttributeType)
	{
		if attr & attribute::GLOBAL != 0
		{
			self.write_attribute_segment(&format!("global {identifier}\n"));
		}
	}

	pub fn write_lable_text_seg(&mut self, lable: &str)
	{
		self.write_text_segment(&format!("\n{lable}:"));
	}

	// Returns the size of a value, in bytes
	pub fn _sizeof_value(&self, value: &Value, locals: &Vec<Variable>) -> u16
	{
		match value
		{
			Value::I32(_) => return 4,
			Value::Var(index) => return locals[*index as usize].data_type.size(),
			Value::FuncCall(info) => return self.ir.functions[info.index as usize].return_type.size()
		}
	}
}