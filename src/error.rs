pub enum CompileErrors
{
	Usage,
}

// Prints a formatted error message to stderr.
// First argument must be of CompileErrors, then arguments for eprintln!()
#[macro_export]
macro_rules! print_err 
{
	( $err_code:expr, $( $print_data:tt )* ) => 
	{
		use std::io::Write;
		eprint!("\x1b[97;1mslowc\x1b[0m: \x1b[31;1merror\x1b[0m - "); 	
		match $err_code
		{
			CompileErrors::Usage => eprint!("Incorrect usage.\n\t")
		}
		std::io::stdout().flush().unwrap();
		eprintln!($($print_data)*);
		std::process::exit($err_code as i32 + 1);
	}
}


// Print "slow: error - " while "slow" is white bold and "error" is in red bold