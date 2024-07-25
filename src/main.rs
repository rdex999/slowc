mod error;
mod preprocessor;
mod lexer;
mod ast;
use error::CompileError;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2
    {
        print_err!(CompileError::Usage, "Correct usage: slowc <FILE.slw>");
    }
    
    slowc_compile_file(&argv[1]);
}

fn slowc_compile_file(filepath: &str)
{
    let mut source = std::fs::read_to_string(filepath)
        .unwrap_or_else(|err| {print_err!(CompileError::NoSuchFile(filepath), "Error: {err}");});
    
    print_msg!("Compiling file: \"{filepath}\"");

    let preprocessor = preprocessor::Preprocessor::new(source);
    source = preprocessor.preprocess();

    let lexer = lexer::Lexer::new(&source);
    
    let parser = ast::parser::Parser::new(lexer);

    let ir = parser.generate_ir();

    print_msg!("IR:\n\t{:#?}", ir);
    // while let Some(token) = lexer.next()
    // {
    //     print_msg!("{:?}", token);
    // }
}