pub enum CompileErrors<'a>
{
	Usage,
	NoSuchFile(&'a str),
	UnexpectedEof,
	NoSuchOperator(&'a str),
}

pub enum ExitCodes
{
	Usage,
	NoSuchFile,
	UnexpectedEof,
	NoSuchOperator,
}


// Prints a formatted error message to stderr.
// First argument must be of CompileErrors, then arguments for eprintln!()
#[macro_export]
macro_rules! print_err 
{
	( $compile_error:expr, $( $print_data:tt )* ) => 
	{
		// Can someone please explain to me why the fuck do i need this line
		// I mean i dont need to do it for the CompileErrors enum
		use crate::error::ExitCodes;

		// Print "slow: error - " while "slow" is white bold and "error" is in red bold
		eprint!("\x1b[1mslowc\x1b[0m: \x1b[31;1merror\x1b[0m - "); 	
		let error_code: i32;	
		match $compile_error
		{
			CompileErrors::Usage => 
			{
				eprint!("Incorrect usage.\n\t");
				error_code = ExitCodes::Usage as i32;
			},

			CompileErrors::NoSuchFile(filepath) =>
			{
				eprint!("No such file: \"{filepath}\"\n\t");
				error_code = ExitCodes::NoSuchFile as i32;
			},

			CompileErrors::UnexpectedEof =>
			{
				eprint!("Unexpected Eof.\n\t");
				error_code = ExitCodes::UnexpectedEof as i32;
			},

			CompileErrors::NoSuchOperator(op) =>
			{
				eprint!("No such operator \"{op}\".");
				error_code = ExitCodes::NoSuchOperator as i32;
			},
		}
		eprintln!($($print_data)*);
		std::process::exit(error_code + 1); 	/* +1 because error codes start from 1 and enums start from 0 */
	}
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
