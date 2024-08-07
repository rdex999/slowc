use super::ast::Type;

pub enum CompileError<'a>
{
	Usage,
	NoSuchFile(&'a str),
	UnexpectedEof,
	NoSuchOperator(&'a str),
	Syntax,
	UnknownIdentifier(&'a str),
	TypeError(Type, Type),			/* ExpectedType, GivenType */
	InvalidPreprocessorCommand(&'a str),
	FileWriteError(&'a str),
}

pub enum ExitCodes
{
	Usage,
	NoSuchFile,
	UnexpectedEof,
	NoSuchOperator,
	Syntax,
	UnknownIdentifier,
	TypeError,
	InvalidPreprocessorCommand,
	FileWriteError,
}

pub struct LineInfo
{
	pub line_index: usize,
	pub column: usize,
	pub line_contents: String,
	pub tabs_count: u8,
}

impl LineInfo
{
	pub fn new(line_index: usize, column: usize, line_contents: String, tabs_count: u8) -> Self
	{
		return Self {
			line_index,
			column,
			line_contents,
			tabs_count,
		};
	}
}

// Takes O(n), line, column, line_contents
pub fn get_line_from_index(source: &str, index: usize) -> LineInfo
{
	fn count_leading_char(string: &str, ch: char) -> u8
	{
		let mut count = 0;
		for c in string.chars()
		{
			if !c.is_whitespace()
			{
				break;
			}
			if c == ch
			{
				count += 1;
			}
		}
		return count;
	}

	let mut index_in_source: usize = 0;	

	for (i, line) in source.lines().enumerate()
	{
		if index_in_source + line.len() > index
		{
			let line_contents = line.to_string();
			let tabs = count_leading_char(&line_contents, '\t');
			return LineInfo::new(
				i,
				index - index_in_source,
				line_contents,
				tabs,
			);
		}
		index_in_source += line.len() + 1;
	}

	let lines: Vec<&str> = source.lines().collect();
	let line_contents = lines[lines.len() -  1].to_string();
	let column = line_contents.len();
	let tabs = count_leading_char(&line_contents, '\t');
	return LineInfo::new(
		lines.len() - 1,
		column,
		line_contents,	
		tabs,
	);
}


pub fn get_exit_code(compile_error: CompileError) -> ExitCodes
{
	match compile_error
	{
		CompileError::Usage => 
		{
			eprint!("Incorrect usage.\n\t");
			return ExitCodes::Usage;
		},

		CompileError::NoSuchFile(filepath) =>
		{
			eprint!("No such file: \"{filepath}\"\n\t");
			return ExitCodes::NoSuchFile;
		},

		CompileError::UnexpectedEof =>
		{
			eprint!("Unexpected Eof.\n\t");
			return ExitCodes::UnexpectedEof;
		},

		CompileError::NoSuchOperator(op) =>
		{
			eprint!("No such operator \"{op}\".");
			return ExitCodes::NoSuchOperator;
		},

		CompileError::Syntax =>
		{
			eprint!("Syntax error.\n\t");
			return ExitCodes::Syntax;
		},

		CompileError::UnknownIdentifier(ident) =>
		{
			eprint!("Unknown identifier \"{ident}\".");
			return ExitCodes::UnknownIdentifier;
		},

		CompileError::TypeError(expected, given) =>
		{
			eprint!("Type error. Expected {} but type {} was given. ", expected.to_string(), given.to_string());
			return ExitCodes::TypeError;
		},

		CompileError::InvalidPreprocessorCommand(command) =>
		{
			eprint!("Invalid preprocessor command. \"{command}\"");
			return ExitCodes::InvalidPreprocessorCommand;
		},

		CompileError::FileWriteError(file_path) =>
		{
			eprint!("Could not write to file \"{file_path}\".");
			return ExitCodes::FileWriteError;
		}
	}
}

// Prints a formatted error message to stderr.
// First argument must be of CompileError, then the text span of the error, then arguments for eprintln!()
#[macro_export]
macro_rules! print_err 
{
	( $compile_error:expr, $( $print_data:tt )* ) => 
	{
		// Print "slow: error - " while "slow" is white bold and "error" is in red bold
		eprint!("\x1b[1mslowc\x1b[0m: \x1b[31;1merror\x1b[0m - "); 	
		let exit_code = crate::error::get_exit_code($compile_error);
		eprintln!($($print_data)*);
		std::process::exit(exit_code as i32 + 1); 	/* +1 because error codes start from 1 and enums start from 0 */
	}
}

#[macro_export]
macro_rules! print_errln {
	($compile_error:expr, $source:expr, $source_index:expr, $( $print_data:tt )* ) => {
		// Print "slow: error - " while "slow" is in bold and "error" is in red bold
		eprint!("\x1b[1mslowc\x1b[0m: \x1b[31;1merror\x1b[0m - "); 	
		let exit_code = crate::error::get_exit_code($compile_error);
		let line = crate::error::get_line_from_index($source, $source_index);
		eprintln!($($print_data)*);
		eprintln!("\tOn line {}: {}", line.line_index + 1, line.line_contents);
		eprintln!("\t  {}{}\x1b[1mHere: <---->\x1b[0m", str::repeat(" ", line.column), str::repeat("\t", line.tabs_count as usize));
		std::process::exit(exit_code as i32 + 1); 	/* +1 because error codes start from 1 and enums start from 0 */
	};
}

// Prints a formatted warning message to stdout.
#[macro_export]
macro_rules! print_wrn
{
	( $( $print_data:tt )* ) => 
	{
		// Print "slow: warning - " while "slow" is in bold and "warning" is in yellow bold
		print!("\x1b[1mslowc\x1b[0m: \x1b[93;1mwarning\x1b[0m - ");
		println!($($print_data)*);
	}
}

// Prints a formatted message to stdout.
#[macro_export]
macro_rules! print_msg
{
	( $( $print_data:tt )* ) => 
	{
		// Print "slow: info - " while "slow" is in bold and "info" is in white bold
		print!("\x1b[1mslowc\x1b[0m: \x1b[97;1minfo\x1b[0m - ");
		println!($($print_data)*);
	}
}
