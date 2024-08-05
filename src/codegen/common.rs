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
			self.write_attribute_segment(&format!("\nglobal {identifier}\n"));
		} else if attr & attribute::EXTERN != 0
		{
			self.write_attribute_segment(&format!("\nextern {identifier}\n"));
		}
		
	}

	pub fn write_lable_text_seg(&mut self, lable: &str)
	{
		self.write_text_segment(&format!("\n{lable}:"));
	}

	// Returns the size of a value, in bytes
	pub fn _value_type(&self, value: &Value, locals: &Vec<Variable>) -> Type
	{
		return match value
		{
			Value::I8(_)   								=> Type::I8,
			Value::U8(_)	 							=> Type::U8,
			Value::I16(_) 								=> Type::I16,
			Value::U16(_) 								=> Type::U16,
			Value::I32(_) 								=> Type::I32,
			Value::U32(_) 								=> Type::U32,
			Value::I64(_) 								=> Type::I64,
			Value::U64(_)								=> Type::U64,
			Value::Var(index) 						=> locals[*index as usize].data_type,
			Value::FuncCall(info)	=> self.ir.functions[info.index as usize].return_type,
		}
	}
}