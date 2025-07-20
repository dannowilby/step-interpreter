use crate::{
    common::program::Program,
    compilation::{
        ast::{build_ast, compile_ast},
        token::tokenize,
    },
};

pub mod ast;
pub mod token;

pub fn compile(source: String) -> Program {
    let tokens = tokenize(source);

    let ast = build_ast(tokens);
    let bytecode = compile_ast(ast);

    Program::new(bytecode)
}
