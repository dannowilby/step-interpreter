use std::rc::Rc;

use step_compiler::common::bytecode::ByteCodes;
use step_compiler::common::program::Program;
use step_compiler::execution::step_execute;

#[test]
fn test_program_1() {
    let mut program = Program::new(vec![
        ByteCodes::Store(0, Rc::new(2u32)),
        ByteCodes::Store(1, Rc::new(3u32)),
        ByteCodes::Add(2, 0, 1),
    ]);

    step_execute(&mut program);
    step_execute(&mut program);
    step_execute(&mut program);

    let output_index: usize = 2;
    let output = program
        .stack
        .get(&output_index)
        .expect("")
        .downcast_ref::<u32>()
        .expect("");
    assert_eq!(*output, 5);
}

#[test]
fn test_program_2() {
    let mut program = Program::new(vec![
        ByteCodes::Store(0, Rc::new(2u32)),
        ByteCodes::Store(1, Rc::new(3u32)),
        ByteCodes::Mult(2, 0, 1),
    ]);

    step_execute(&mut program);
    step_execute(&mut program);
    step_execute(&mut program);

    let output_index: usize = 2;
    let output = program
        .stack
        .get(&output_index)
        .expect("")
        .downcast_ref::<u32>()
        .expect("");
    assert_eq!(*output, 6);
}

#[test]
fn test_program_3() {}

#[test]
fn test_program_4() {}

#[test]
fn test_partial_execution_1() {}

#[test]
fn test_partial_execution_2() {}

#[test]
fn test_partial_execution_3() {}

#[test]
fn test_access_uninitialized_memory_1() {}

#[test]
fn test_access_uninitialized_memory_2() {}

#[test]
fn test_access_uninitialized_memory_3() {}

#[test]
fn test_incorrect_stored_type_1() {}

#[test]
fn test_incorrect_stored_type_2() {}

#[test]
fn test_incorrect_stored_type_3() {}

#[test]
fn test_incorrect_stored_type_4() {}
