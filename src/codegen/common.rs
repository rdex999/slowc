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

	pub fn decl_attribute(&mut self, identifier: &str, attr: attribute::AttributeType)
	{
		if attr & attribute::GLOBAL != 0
		{
			self.write_attribute_segment(&format!("global {identifier}\n"));
		}
	}
}