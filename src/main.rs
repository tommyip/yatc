/// Yet Another Tiny Compiler (yatc)

mod lexer;

fn main() {
    let src = "let apple = \"a type of fruit\" ; return apple + 1337 ;";

    let token_stream = lexer::lex(src);

    println!("token_stream: {:?}", token_stream);
}
