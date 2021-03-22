use nom::{bytes::streaming::take, error::ParseError, number::streaming::le_u64, IResult};
use uuid::Uuid;

use crate::{guid::*, span::Span};

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub guid: Uuid,
    pub size: u64,
}

#[derive(Debug, PartialEq)]
pub struct Object<'a> {
    pub guid: Uuid,
    pub data: Span<'a>,
}

pub fn object_header<'a, E: ParseError<Span<'a>>>(
    input: Span<'a>,
) -> IResult<Span<'a>, ObjectHeader, E> {
    let (input, guid) = guid(input)?;
    let (input, size) = le_u64(input)?;
    Ok((input, ObjectHeader { guid, size }))
}

pub fn object<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Object, E> {
    let (input, header) = object_header(input)?;
    let (input, data) = take(header.size - 24)(input)?;
    Ok((
        input,
        Object {
            guid: header.guid,
            data,
        },
    ))
}
