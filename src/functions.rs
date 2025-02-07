use std::fmt::Debug;

use crate::{instructions::StoredData, runtime::RuntimeError};

pub trait NativeFunction: Debug {
    fn execute(&self, args: &Vec<StoredData>) -> Result<StoredData, RuntimeError>;
}

#[derive(Debug)]
pub enum DefinedFunction {
    Bytecode,
    Native(Box<dyn NativeFunction>),
}
