use super::*;

impl<'a> CodeGen<'a>
{
	pub fn write_attribute_segment(&mut self, data: &str)
	{
		self.attribute_segment.push_str(data);
	}

	pub fn write_data_segment(&mut self, data: &str)
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

	pub fn write_lable(&mut self, lable: Lable)
	{
		match lable.kind
		{
			LableKind::DataSeg => self.write_data_segment(&format!("\n\t{lable}:")),
			LableKind::TextSeg => self.write_text_segment(&format!("\n{lable}:")),
		}
	}

	pub fn decl_var_data_seg(&mut self, value: &Value) -> Lable
	{
		let lable = self.generate_data_seg_lable();
		match value
		{
			Value::I8(number) => self.write_data_segment(&format!("\n\t{lable}: db {number}")),
			Value::U8(number) => self.write_data_segment(&format!("\n\t{lable}: db {number}")),
			Value::I16(number) => self.write_data_segment(&format!("\n\t{lable}: dw {number}")),
			Value::U16(number) => self.write_data_segment(&format!("\n\t{lable}: dw {number}")),
			Value::I32(number) => self.write_data_segment(&format!("\n\t{lable}: dd {number}")),
			Value::U32(number) => self.write_data_segment(&format!("\n\t{lable}: dd {number}")),
			Value::I64(number) => self.write_data_segment(&format!("\n\t{lable}: dq {number}")),
			Value::U64(number) => self.write_data_segment(&format!("\n\t{lable}: dq {number}")),
			Value::F32(number) => self.write_data_segment(&format!("\n\t{lable}: dd {:?}", number)),
			Value::F64(number) => self.write_data_segment(&format!("\n\t{lable}: dq {:?}", number)),
			_ => panic!("Dev error! decl_var_data_seg called with a value that is not constant."),
		}

		return lable;
	}

	pub fn generate_data_seg_lable(&mut self) -> Lable
	{
		let index = self.data_seg_var_index;
		self.data_seg_var_index += 1;
		return Lable::new(index, LableKind::DataSeg);
	}

	pub fn generate_text_seg_lable(&mut self) -> Lable
	{
		let index = self.text_seg_var_index;
		self.text_seg_var_index += 1;
		return Lable::new(index, LableKind::TextSeg);
	}

	// Returns the size of a value, in bytes
	pub fn value_type(&self, value: &Value, locals: &Vec<Variable>) -> Type
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
			Value::F32(_)								=> Type::F32,
			Value::F64(_)								=> Type::F64,
			Value::Var(index) 						=> locals[*index as usize].data_type,
			Value::FuncCall(info)	=> self.ir.functions[info.index as usize].return_type,
		}
	}
}