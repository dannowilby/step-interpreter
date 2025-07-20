use crate::{common::bytecode::ByteCodes, compilation::token::{Token, TokenSpan}};

pub struct AstNode {
    right: Option<usize>,
    left: Option<usize>,

    data: Option<u32>,
    op: Option<char>,
}

pub fn build_ast(mut tokens: Vec<TokenSpan>) -> Vec<AstNode> {
    let mut output = Vec::<AstNode>::new();

    output
}

pub fn compile_ast(ast: Vec<AstNode>) -> Vec<ByteCodes> {
    vec![]
}
