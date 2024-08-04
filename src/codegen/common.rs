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
	pub fn _sizeof_value(&self, value: &Value, locals: &Vec<Variable>) -> OpSize
	{
		return match value
		{
			Value::I8(_)  /* | Value::U8(_) */ 			=> OP_BYTE,
			Value::I16(_) | Value::U16(_) 				=> OP_WORD,
			Value::I32(_) | Value::U32(_) 				=> OP_DWORD,
			Value::I64(_) | Value::U64(_)				=> OP_QWORD,
			Value::Var(index) 						=> locals[*index as usize].data_type.size(),
			Value::FuncCall(info)	=> self.ir.functions[info.index as usize].return_type.size()
		}
	}
}