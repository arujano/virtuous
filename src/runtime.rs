use std::collections::HashMap;

use crate::{
    functions::{DefinedFunction, NativeFunction},
    instructions::*,
    parser::parse_script,
};

#[derive(Debug)]
pub enum RuntimeError {
    Generic(String),
    InvalidLocation(MemoryLocation),
    InvalidArgument(DataType, DataType),
    InvalidArgumentCount(u8, u8),
}

#[derive(Debug)]
pub struct Runtime {
    actions: Vec<Action>,
    memory: HashMap<MemoryLocation, StoredData>,
    definitions: HashMap<MemoryLocation, DefinedFunction>,
}

impl Runtime {
    pub fn new(script: &[u8]) -> Result<Self, ()> {
        let (_, actions) = parse_script(script).unwrap();

        Ok(Self {
            actions,
            memory: HashMap::new(),
            definitions: HashMap::new(),
        })
    }

    pub fn add_native_function<T: 'static + NativeFunction>(
        &mut self,
        location: MemoryLocation,
        function: T,
    ) {
        _ = self
            .definitions
            .insert(location, DefinedFunction::Native(Box::new(function)));
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        for action in &self.actions {
            match action {
                Action::StoreMemory(action) => match &action.data {
                    AccessData::Literal(access) => {
                        _ = self.memory.insert(
                            action.location.clone(),
                            StoredData {
                                ty: access.0.ty.clone(),
                                data: access.0.data.clone(),
                            },
                        )
                    }
                    AccessData::Memory(access) => {
                        if self.memory.contains_key(&access.0) {
                            let data = self.memory[&access.0].clone();
                            self.memory.insert(action.location.clone(), data);
                        } else {
                            return Err(RuntimeError::InvalidLocation(access.0.clone()));
                        }
                    }
                },
                Action::FreeMemory(action) => {
                    if self.memory.contains_key(&action.0) {
                        self.memory.remove(&action.0);
                    } else {
                        return Err(RuntimeError::InvalidLocation(action.0.clone()));
                    }
                }
                Action::DeclareFunction(_) => todo!("Add function declarations"),
                Action::CallFunction(action) => {
                    if !self.definitions.contains_key(&action.location) {
                        return Err(RuntimeError::InvalidLocation(action.location.clone()));
                    }

                    let mut args: Vec<StoredData> = Vec::new();
                    for arg in &action.args {
                        args.push(match arg {
                            AccessData::Literal(access) => StoredData {
                                ty: access.0.ty.clone(),
                                data: access.0.data.clone(),
                            },
                            AccessData::Memory(access) => {
                                if !self.memory.contains_key(&access.0) {
                                    return Err(RuntimeError::InvalidLocation(access.0.clone()));
                                }

                                self.memory[&access.0].clone()
                            }
                        })
                    }

                    let result = match &self.definitions[&action.location] {
                        DefinedFunction::Bytecode => todo!("Implement bytecode-defined functions"),
                        DefinedFunction::Native(function) => function.execute(&args),
                    }?;

                    if !action.result_location.is_empty() {
                        _ = self.memory.insert(action.result_location.clone(), result);
                    }
                }
            }
        }

        Ok(())
    }
}
