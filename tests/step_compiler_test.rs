use step_compiler::{Program, StepCompiler};

#[test]
fn test_arithmetic_execution() {
    // might add a language server
    // with the language server, might want to add bindings too
    let compiler = StepCompiler::new();
    let mut program = compiler.from_source("5 * (3 + 4)".to_owned());

    compiler.step_execute(&mut program);
}
