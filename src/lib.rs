pub mod functions;
pub mod instructions;
pub mod parser;
pub mod runtime;
pub mod script_builder;

#[allow(unused)]
pub mod prelude {
    pub use crate::functions::NativeFunction;
    pub use crate::instructions;
    pub use crate::runtime::*;
    pub use crate::script_builder::*;
}
