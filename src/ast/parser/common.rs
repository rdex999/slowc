use super::*;

impl<'a> Parser<'a>
{
	pub fn get_text(&self, text_span: &TextSpan) -> &'a str
	{
		return &self.source[text_span.start..text_span.end];
	}
}