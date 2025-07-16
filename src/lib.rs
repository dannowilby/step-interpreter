/// The compiled program representing the bytecode and the current step of execution.
pub struct Program {
    pub bytecode: Vec<()>,
    pub step: u64,
}

impl Program {
    pub fn new(bytecode: Vec<()>) -> Self {
        Program { bytecode, step: 0 }
    }
}

/// The master controller for stepping through program executions. Initialize with the bytecode compiler you want to use (the language), and step through.
pub struct StepCompiler {}

impl StepCompiler {
    pub fn new() -> Self {
        StepCompiler {}
    }

    pub fn step_execute(&self, program: &mut Program) {}

    pub fn from_source(&self, _src: String) -> Program {
        Program::new(vec![])
    }
}
