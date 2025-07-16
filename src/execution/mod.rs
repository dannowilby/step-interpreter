use std::rc::Rc;

use crate::common::bytecode::ByteCodes;
use crate::common::program::Program;

pub fn step_execute(program: &mut Program) {
    let current_step = program.step;
    let exp = &program.bytecode[current_step];

    let stack = &mut program.stack;

    match exp {
        ByteCodes::Store(index, data) => {
            program.stack.insert(*index, data.clone());
        }
        ByteCodes::Add(store, read1, read2) => {
            let op1 = stack.get(read1).expect("").downcast_ref::<u32>().expect("");
            let op2 = stack.get(read2).expect("").downcast_ref::<u32>().expect("");

            stack.insert(*store, Rc::new(op1 + op2));
        }
        ByteCodes::Mult(store, read1, read2) => {
            let op1 = stack.get(read1).expect("").downcast_ref::<u32>().expect("");
            let op2 = stack.get(read2).expect("").downcast_ref::<u32>().expect("");

            stack.insert(*store, Rc::new(op1 * op2));
        }
    }

    program.step += 1;
}
