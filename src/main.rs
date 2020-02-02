mod ast;
mod eval;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use crate::eval::eval;
use std::io;
use std::io::Write;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope.
lrlex_mod!("calc.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope.
lrpar_mod!("calc.y");

fn compile<'input>(input: &'input str) -> Result<ast::Expr, ()> {
    let lexerdef = calc_l::lexerdef();
    let lexer = lexerdef.lexer(input);
    let (expr, err) = calc_y::parse(&lexer);
    let res = expr.unwrap();
    res.map_err(|_| ())
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                let expr = compile(input.as_str());
                match expr {
                    Ok(e) => println!("Result: {}", eval(&e)),
                    Err(_) => println!("Parsing error")
                }
            }
            Err(_) => { println!("Input closed"); break }
        }
    }
}
