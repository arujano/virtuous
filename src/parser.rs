use crate::instructions::*;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take, take_while},
    combinator::{map, value},
    multi::{count, many0},
    sequence::{pair, preceded},
};

pub enum ParseError {
    Generic(String),
}

// Type-related instructions
pub fn parse_type(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        value(DataType::Void, tag(&[TYPE_VOID][..])),
        value(DataType::Byte, tag(&[TYPE_BYTE][..])),
        value(DataType::Bool, tag(&[TYPE_BOOL][..])),
        value(DataType::Int, tag(&[TYPE_INT][..])),
        value(DataType::Float, tag(&[TYPE_FLOAT][..])),
        value(DataType::String, tag(&[TYPE_STRING][..])),
        value(DataType::UID, tag(&[TYPE_UID][..])),
        value(DataType::Func, tag(&[TYPE_FUNC][..])),
        value(DataType::Table, tag(&[TYPE_TABLE][..])),
        value(DataType::Struct, tag(&[TYPE_STRUCT][..])),
    ))
    .parse(input)
}

pub fn parse_int(input: &[u8]) -> IResult<&[u8], ByteSequence> {
    map(take(4usize), |data: &[u8]| data.to_vec()).parse(input)
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], ByteSequence> {
    let (input, data) = take_while(|byte| byte != b'\0').parse(input)?;
    let (input, _) = take(1usize).parse(input)?; // Consume b'\0'

    Ok((input, data.to_vec()))
}

// Data accessing-related instructions
pub fn parse_literal(input: &[u8]) -> IResult<&[u8], Literal> {
    alt((
        map(preceded(tag(&[TYPE_INT][..]), parse_int), |data| Literal {
            ty: DataType::Int,
            data,
        }),
        map(preceded(tag(&[TYPE_STRING][..]), parse_string), |data| {
            Literal {
                ty: DataType::String,
                data,
            }
        }),
    ))
    .parse(input)
}

pub fn parse_access_literal(input: &[u8]) -> IResult<&[u8], AccessLiteral> {
    map(
        preceded(tag(&[ACCESS_LITERAL][..]), parse_literal),
        |literal| AccessLiteral(literal),
    )
    .parse(input)
}

pub fn parse_access_memory(input: &[u8]) -> IResult<&[u8], AccessMemory> {
    map(
        preceded(tag(&[ACCESS_MEMORY][..]), parse_string),
        |location| AccessMemory(location),
    )
    .parse(input)
}

pub fn parse_access_data(input: &[u8]) -> IResult<&[u8], AccessData> {
    alt((
        map(parse_access_literal, |access| AccessData::Literal(access)),
        map(parse_access_memory, |access| AccessData::Memory(access)),
    ))
    .parse(input)
}

// Data storage-related instructions
pub fn parse_store_memory(input: &[u8]) -> IResult<&[u8], StoreMemory> {
    let (input, _) = tag(&[STORE_MEMORY][..]).parse(input)?;
    let (input, location) = parse_string(input)?;
    let (input, data) = parse_access_data(input)?;

    Ok((input, StoreMemory { location, data }))
}

pub fn parse_free_memory(input: &[u8]) -> IResult<&[u8], FreeMemory> {
    let (input, _) = tag(&[FREE_MEMORY][..]).parse(input)?;
    let (input, location) = parse_string(input)?;

    Ok((input, FreeMemory(location)))
}

// Function-related instructions
pub fn parse_function_argument(input: &[u8]) -> IResult<&[u8], FunctionArgument> {
    map(pair(parse_type, parse_string), |(ty, location)| {
        FunctionArgument { ty, location }
    })
    .parse(input)
}

pub fn parse_declare_function(input: &[u8]) -> IResult<&[u8], DeclareFunction> {
    let (input, _) = tag(&[DECLARE_FUNCTION][..]).parse(input)?;
    let (input, location) = parse_string(input)?;
    let (input, arg_count) = map(take(1usize), |bytes: &[u8]| bytes[0]).parse(input)?;
    let (input, args) = count(parse_function_argument, arg_count as usize).parse(input)?;
    let (input, return_ty) = parse_type(input)?;

    let statements = Vec::new(); // TODO: Add statement parsing

    Ok((
        input,
        DeclareFunction {
            location,
            arg_count,
            args,
            return_ty,
            statements,
        },
    ))
}

pub fn parse_call_function(input: &[u8]) -> IResult<&[u8], CallFunction> {
    let (input, _) = tag(&[CALL_FUNCTION][..]).parse(input)?;
    let (input, location) = parse_string(input)?;
    let (input, arg_count) = map(take(1usize), |bytes: &[u8]| bytes[0]).parse(input)?;
    let (input, args) = count(parse_access_data, arg_count as usize).parse(input)?;
    let (input, result_location) = parse_string(input)?;

    Ok((
        input,
        CallFunction {
            location,
            arg_count,
            args,
            result_location,
        },
    ))
}

// Master
pub fn parse_action(input: &[u8]) -> IResult<&[u8], Action> {
    alt((
        map(parse_store_memory, |x| Action::StoreMemory(x)),
        map(parse_free_memory, |x| Action::FreeMemory(x)),
        map(parse_declare_function, |x| Action::DeclareFunction(x)),
        map(parse_call_function, |x| Action::CallFunction(x)),
    ))
    .parse(input)
}

pub fn parse_script(input: &[u8]) -> IResult<&[u8], Vec<Action>> {
    many0(parse_action).parse(input)
}
