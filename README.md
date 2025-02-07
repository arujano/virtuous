Virtuous is my attempt at building a bytecode language and virtual machine from scratch just for the fun of it. Feel free to take a look!

The general design is based off of traditional heap machines and is meant to look and feel close to a traditional programming language. For example, the general structure for calling a function is `LOCATION ARG_COUNT ARGS RESULT_LOCATION`, which could look like `add:2(3, 4) => result_1` in a more traditional language. Translated to a human-readable format, this would be written as such:

```
CALL_FUNCTION
add
2
ACCESS_LITERAL TYPE_INT 3
ACCESS_LITERAL TYPE_INT 4
result_1
```

One feature I wanted to have more than many others, though, is easy FFI between the runtime and the host language (in this case, Rust). That way, the user can add custom functions to the runtime, as well as directly calling functions present in the bytecode. This is observable in the `addition` example:

```Rust
struct AddFunction;
impl NativeFunction for AddFunction {
    fn execute(
        &self,
        args: &Vec<StoredData>,
    ) -> Result<StoredData, RuntimeError> {
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

        let a = i32::from_be_bytes([args[0].data[0], args[0].data[1], args[0].data[2], args[0].data[3]]);
        let b = i32::from_be_bytes([args[1].data[0], args[1].data[1], args[1].data[2], args[1].data[3]]);

        Ok(StoredData {
            ty: DataType::Int,
            data: (a + b).to_be_bytes().to_vec(),
        })
    }
}
```

A bit verbose, eh? Maybe I'll sort that out one day, but I don't think it's that practical to make this less verbose. Well, at least adding the function to the runtime's registry is easy:

```Rust
let mut runtime = Runtime::new(script.get_binary())?;
runtime.add_native_function(b"add".to_vec(), AddFunction);
```

There's still a lot missing (like an actual standard library and support for more complex types), but this was a project I had fun working on so far.