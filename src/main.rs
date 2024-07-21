mod parse;


use parse::Interpreter;

fn main() {
    let src = include_bytes!("../src.bf");

    Interpreter::new(src, 0x1000).execute();

    // println!("{:?}", p.src);
}
