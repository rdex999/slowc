use super::{super::lexer::*, tree};

pub struct Parser<'a>
{
	tokens_itr: Lexer<'a>,
}

impl<'a> Parser<'a>
{
	pub fn new(lexer: Lexer<'a>) -> Self
	{
		return Self{
			tokens_itr: lexer,
		};
	}

	// pub fn generate_ast(&mut self) -> tree::Root
	// {
	// }
}