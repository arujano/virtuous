#![allow(unused)]

use std::fmt::Debug;

pub type ByteSequence = Vec<u8>;
pub type MemoryLocation = ByteSequence;

#[derive(Debug)]
pub enum Action {
    StoreMemory(StoreMemory),
    FreeMemory(FreeMemory),
    DeclareFunction(DeclareFunction),
    CallFunction(CallFunction),
}

// Basic instructions
pub const END: u8 = 0;

// Type-related instructions
pub const TYPE_VOID: u8 = 10;
pub const TYPE_BYTE: u8 = 11;
pub const TYPE_BOOL: u8 = 12;
pub const TYPE_INT: u8 = 13;
pub const TYPE_FLOAT: u8 = 14;
pub const TYPE_STRING: u8 = 15;
pub const TYPE_UID: u8 = 16;
pub const TYPE_FUNC: u8 = 17;
pub const TYPE_TABLE: u8 = 18;
pub const TYPE_STRUCT: u8 = 19;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Void,
    Byte,
    Bool,
    Int,
    Float,
    String,
    UID,
    Func,
    Table,
    Struct,
}

// Data accessing-related instructions
pub const ACCESS_LITERAL: u8 = 20; // TYPE_* + data
pub const ACCESS_MEMORY: u8 = 21; // Location (string)

#[derive(Debug)]
pub struct Literal {
    pub ty: DataType,
    pub data: ByteSequence,
}

#[derive(Debug)]
pub struct AccessLiteral(pub Literal);

#[derive(Debug)]
pub struct AccessMemory(pub MemoryLocation);

#[derive(Debug, Clone)]
pub struct StoredData {
    pub ty: DataType,
    pub data: ByteSequence,
}

impl StoredData {
    pub const NULL: Self = Self {
        ty: DataType::Void,
        data: Vec::new(),
    };
}

#[derive(Debug)]
pub enum AccessData {
    Literal(AccessLiteral),
    Memory(AccessMemory),
}

// Data storage-related instructions
pub const STORE_MEMORY: u8 = 30; // Location (string) + TYPE_* + ACCESS_*
pub const FREE_MEMORY: u8 = 31; // Location (string)

#[derive(Debug)]
pub struct StoreMemory {
    pub location: MemoryLocation,
    pub data: AccessData,
}

#[derive(Debug)]
pub struct FreeMemory(pub MemoryLocation);

// Function-related instructions
pub const DECLARE_FUNCTION: u8 = 40; // Location (string) + arg count (u8) + args (TYPE_* + string) + return (TYPE_*) + statements until END
pub const CALL_FUNCTION: u8 = 41; // Location (string) + arg count (u8) + args (ACCESS_* instructions) + result storage location (string)

#[derive(Debug)]
pub struct FunctionArgument {
    pub ty: DataType,
    pub location: MemoryLocation,
}

#[derive(Debug)]
pub struct DeclareFunction {
    pub location: MemoryLocation,
    pub arg_count: u8,
    pub args: Vec<FunctionArgument>,
    pub return_ty: DataType,
    pub statements: Vec<()>,
}

#[derive(Debug)]
pub struct CallFunction {
    pub location: MemoryLocation,
    pub arg_count: u8,
    pub args: Vec<AccessData>,
    pub result_location: MemoryLocation,
}
