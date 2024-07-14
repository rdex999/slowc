mod error;
use error::CompileErrors;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2
    {
        print_err!(CompileErrors::Usage, "Correct usage: slowc <FILE.slw>");
    }

    let filename: &String = &argv[1];
    print_msg!("Compiling file \"{filename}\"");
}