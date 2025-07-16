use std::{any::Any, rc::Rc};

/// The instruction set that all programs are compiled to and executed from
pub enum ByteCodes {
    /// store addr, read 1 addr, read 2 addr
    Add(usize, usize, usize),
    Mult(usize, usize, usize),

    /// addr, obj
    Store(usize, Rc<dyn Any>),
}
