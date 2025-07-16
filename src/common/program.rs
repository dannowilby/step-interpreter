use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::bytecode::ByteCodes;

/// The compiled program representing the bytecode and the current step of execution.
pub struct Program {
    pub bytecode: Vec<ByteCodes>,
    pub step: usize,

    pub stack: HashMap<usize, Rc<dyn Any>>,
}

impl Program {
    pub fn new(bytecode: Vec<ByteCodes>) -> Self {
        Program {
            bytecode,
            step: 0,
            stack: HashMap::new(),
        }
    }
}
