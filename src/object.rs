
use nom::{IResult, bytes::streaming::take, error::ParseError, number::streaming::le_u64};
use uuid::Uuid;

use crate::guid::*;

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub guid: Uuid,
    pub size: u64,
}

#[derive(Debug, PartialEq)]
pub struct Object<'a> {
    pub guid: Uuid,
    pub data: &'a [u8],
}

pub fn object_header<'a, E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], ObjectHeader, E> {
    let (input, guid) = guid(input)?;
    let (input, size) = le_u64(input)?;
    Ok((input, ObjectHeader{guid, size}))
}

pub fn object<'a, E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Object, E> {
    let (input, header) = object_header(input)?;
    let (input, data) = take(header.size - 24)(input)?;
    Ok((input, Object{guid: header.guid, data}))
}
