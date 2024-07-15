mod error;
mod lexer;
use error::CompileErrors;
use lexer::TokenKind;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2
    {
        print_err!(CompileErrors::Usage, "Correct usage: slowc <FILE.slw>");
    }
    
    slowc_compile_file(&argv[1]);
    // let ch: char = ' ';
    // println!("alnum: {}", lexer::Lexer::is_op_start(ch));
}

fn slowc_compile_file(filepath: &str)
{
    let source = std::fs::read_to_string(filepath)
        .unwrap_or_else(|err| {print_err!(CompileErrors::NoSuchFile(filepath), "Error: {err}");});
    
    print_msg!("Compiling file: \"{filepath}\"");

    let mut lexer = lexer::Lexer::new(&source);

    let mut token = lexer.next_token();
    while token.kind != TokenKind::Eof
    {
        print_msg!("{:?}", token);
        token = lexer.next_token();
    }

}