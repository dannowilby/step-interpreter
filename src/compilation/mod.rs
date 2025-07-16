use crate::common::{bytecode::ByteCodes, program::Program};

pub fn compile(source: String) -> Program {
    let ast = build_ast(&source);
    let bytecode = compile_ast(ast);

    Program::new(bytecode)
}

fn build_ast(src: &str) -> () {}

fn compile_ast(ast: ()) -> Vec<ByteCodes> {
    vec![]
}
