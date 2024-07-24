use super::{error::CompileError, print_errln, create_keyword};

const COMMAND_START_OP: char = '#';
const COMMAND_INTEXT: 	&str = create_keyword!("הכנסטקסט", "intext");

pub struct Preprocessor
{
	source: String,
}

impl Preprocessor
{
	pub fn new(source: String) -> Self
	{
		return Self {
			source,
		};
	}

	pub fn preprocess(mut self) -> String
	{
		let mut itr = self.source.chars().enumerate();
		while let Some((i, ch)) = itr.next()
		{
			if ch != COMMAND_START_OP
			{
				continue;
			}	

			let mut command_end_index: usize = 0;
			while let Some((i, ch)) = itr.next()
			{
				if !ch.is_alphabetic()
				{
					command_end_index = i;
					break;
				}
			}
			let command = &self.source[i+1..command_end_index];
			match command {
				COMMAND_INTEXT => self.handle_intext(command_end_index+1),

				_ => { print_errln!(CompileError::InvalidPreprocessorCommand(&command[..]), &self.source, i, ""); }
			}

			// Reinitialize the iterator, because the command handlers will change self.source
			itr = self.source.chars().enumerate();

			// Doing this check so in the next iteration of the loop the iterator will point at the first character that was added 
			if i != 0		
			{
				itr.nth(i - 1);
			}
		}

		return self.source;
	}
	
	fn handle_intext(&mut self, index: usize)
	{
		let mut itr = self.source.chars().enumerate();
		itr.nth(index - 2);

		//  Because preprocessor commands start with '#', a 0 index is an invalid value. Hence use that for "uninitialized" variables
		let mut filepath_start_index = 0;
		let mut filepath_end_index = 0;
		while let Some((i, ch)) = itr.next()
		{
			if ch.is_whitespace() && filepath_start_index != 0
			{
				filepath_end_index = i;
				break;
			} 
			if !ch.is_whitespace() && filepath_start_index == 0
			{
				filepath_start_index = i;
			}
		}
		let filepath = &self.source[filepath_start_index..filepath_end_index];
		let file_contents = std::fs::read_to_string(filepath).unwrap_or_else(|err| {
			print_errln!(CompileError::NoSuchFile(filepath), &self.source, index, "{err}");
		});

		let source_before_command = &self.source[..index - (COMMAND_INTEXT.len() + 2)]; 	/* +1 for the whitespace, and another one for the '#' */
		let source_after_command = &self.source[filepath_end_index..];

		let mut new_source = String::with_capacity(self.source.len() + file_contents.len());

		new_source.push_str(source_before_command);
		new_source.push_str(&file_contents);
		new_source.push_str(source_after_command);
		self.source = new_source;
	}
}