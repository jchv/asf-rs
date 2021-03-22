use nom::{IResult, bytes::streaming::take, combinator::complete, error::ParseError, multi::many0};

use crate::{object::*, span::Span};

#[derive(Debug, PartialEq)]
pub struct IndexObject {
}

#[derive(Debug, PartialEq)]
pub struct IndexObjects {
    pub objects: Vec<IndexObject>
}

impl IndexObject {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, header) = object_header(input)?;
        let (input, _data) = take(header.size - 24)(input)?;
        Ok((input, IndexObject{}))
    }
}

impl IndexObjects {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, objects) = many0(complete(IndexObject::parse))(input)?;
        Ok((input, Self{objects}))
    }
}
