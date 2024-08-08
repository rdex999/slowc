mod error;
mod preprocessor;
mod lexer;
mod ast;
mod codegen;
use error::CompileError;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2
    {
        print_err!(CompileError::Usage, "Correct usage: slowc <FILE.slw>");
    }

    let executable_path = "a.out";
    let obj_file = slowc_compile_file(&argv[1]);

    // Linking with the C standard library is temporary. Il create my own in the future
    std::process::Command::new("ld")
        .args(["-o", executable_path])
        .args(["-dynamic-linker", "/lib64/ld-linux-x86-64.so.2"])
        .args(["/usr/lib/crt1.o", "/usr/lib/crti.o", "-lc", "/usr/lib/crtn.o"])
        .arg(obj_file)
        .spawn()
        .expect("Error, failed to link program.");
}

fn slowc_compile_file(filepath: &str) -> &str
{
    let mut source = std::fs::read_to_string(filepath)
        .unwrap_or_else(|err| {print_err!(CompileError::NoSuchFile(filepath), "Error: {err}");});
    
    print_msg!("Compiling file: \"{filepath}\"");

    let preprocessor = preprocessor::Preprocessor::new(source);
    source = preprocessor.preprocess();

    let lexer = lexer::Lexer::new(&source);
    
    let parser = ast::parser::Parser::new(lexer);

    let ir = parser.generate_ir();

    let code_generator = codegen::CodeGen::new(&ir);

    return code_generator.generate();

    // while let Some(token) = lexer.next()
    // {
    //     print_msg!("{:?}", token);
    // }
}