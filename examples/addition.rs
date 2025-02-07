use std::io;

use virtuous::{
    instructions::{DataType, StoredData},
    prelude::*,
};

fn main() -> io::Result<()> {
    let mut script = ScriptBuilder::new();

    script.add_instruction(instructions::CALL_FUNCTION); // We want to call a function
    script.add_bytes(b"add\0"); // Its name is "add"
    script.add_bytes(&2u8.to_be_bytes()); // It needs 2 arguments
    script.add_instruction(instructions::ACCESS_LITERAL); // The first will be a literal
    script.add_instruction(instructions::TYPE_INT); // The literal will be an int
    script.add_bytes(&1i32.to_be_bytes()); // With the value "1"
    script.add_instruction(instructions::ACCESS_LITERAL); // The second will also be a literal
    script.add_instruction(instructions::TYPE_INT); // The literal will be an int
    script.add_bytes(&3i32.to_be_bytes()); // With the value "3"
    script.add_bytes(b"result_1\0"); // The location in "result_1"

    script.add_instruction(instructions::CALL_FUNCTION); // Now we want to call another function
    script.add_bytes(b"print\0"); // Its name is "print"
    script.add_bytes(&1u8.to_be_bytes()); // It needs 1 argument
    script.add_instruction(instructions::ACCESS_MEMORY); // The argument will be the value of a variable
    script.add_bytes(b"result_1\0"); // The location of the variable is "result_1"
    script.add_bytes(b"\0"); // The storage location is an empty string, which means we'll discard the result

    let mut runtime = Runtime::new(script.get_binary()).unwrap();

    runtime.add_native_function(b"add".to_vec(), AddFunction);
    runtime.add_native_function(b"print".to_vec(), PrintFunction);

    runtime.execute().unwrap();

    Ok(())
}

#[derive(Debug)]
struct AddFunction;
impl NativeFunction for AddFunction {
    fn execute(
        &self,
        args: &Vec<instructions::StoredData>,
    ) -> Result<instructions::StoredData, RuntimeError> {
        if args.len() != 2 {
            return Err(RuntimeError::InvalidArgumentCount(2, args.len() as u8));
        }

        if args[0].ty != DataType::Int {
            return Err(RuntimeError::InvalidArgument(
                DataType::Int,
                args[0].ty.clone(),
            ));
        }
        if args[1].ty != DataType::Int {
            return Err(RuntimeError::InvalidArgument(
                DataType::Int,
                args[1].ty.clone(),
            ));
        }

        let a = i32::from_be_bytes([
            args[0].data[0],
            args[0].data[1],
            args[0].data[2],
            args[0].data[3],
        ]);
        let b = i32::from_be_bytes([
            args[1].data[0],
            args[1].data[1],
            args[1].data[2],
            args[1].data[3],
        ]);

        Ok(StoredData {
            ty: DataType::Int,
            data: (a + b).to_be_bytes().to_vec(),
        })
    }
}

#[derive(Debug)]
struct PrintFunction;
impl NativeFunction for PrintFunction {
    fn execute(&self, args: &Vec<StoredData>) -> Result<StoredData, RuntimeError> {
        for arg in args {
            let string = match arg.ty {
                DataType::Void => {
                    return Err(RuntimeError::InvalidArgument(
                        DataType::String,
                        arg.ty.clone(),
                    ));
                }
                DataType::Byte => arg.data[0].to_string(),
                DataType::Bool => {
                    if arg.data[0] == 0 {
                        "false".to_string()
                    } else {
                        "true".to_string()
                    }
                }
                DataType::Int => i32::from_be_bytes([
                    args[0].data[0],
                    args[0].data[1],
                    args[0].data[2],
                    args[0].data[3],
                ])
                .to_string(),
                DataType::Float => f32::from_be_bytes([
                    args[0].data[0],
                    args[0].data[1],
                    args[0].data[2],
                    args[0].data[3],
                ])
                .to_string(),
                DataType::String => String::from_utf8_lossy(&arg.data).to_string(),
                DataType::UID => todo!("Implement UID to string"),
                DataType::Func => {
                    return Err(RuntimeError::InvalidArgument(
                        DataType::String,
                        arg.ty.clone(),
                    ));
                }
                DataType::Table => todo!("Implement table to string"),
                DataType::Struct => {
                    return Err(RuntimeError::InvalidArgument(
                        DataType::String,
                        arg.ty.clone(),
                    ));
                }
            };

            println!("{string}");
        }

        Ok(StoredData::NULL)
    }
}
