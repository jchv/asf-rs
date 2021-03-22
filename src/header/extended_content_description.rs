use crate::{span::Span, widestr::*};
use nom::{
    bytes::streaming::take, error::ParseError, multi::length_count, number::streaming::le_u16,
    IResult,
};
use std::{convert::TryInto, io::Write};

#[derive(Debug, PartialEq)]
pub struct ContentDescriptor<'a> {
    pub name: WideStr,
    pub value_type: u16,
    pub value: Span<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ExtendedContentDescriptionData<'a> {
    pub descriptors: Vec<ContentDescriptor<'a>>,
}

impl<'a> ContentDescriptor<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, name) = WideStr::parse_count16(input)?;
        let (input, value_type) = le_u16(input)?;
        let (input, value_len) = le_u16(input)?;
        let (input, value) = take(value_len)(input)?;
        Ok((
            input,
            Self {
                name,
                value_type,
                value,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let value_len: u16 = self.value.len().try_into()?;
        self.name.write_count16(w)?;
        w.write_all(&self.value_type.to_le_bytes())?;
        w.write_all(&value_len.to_le_bytes())?;
        w.write_all(&self.value)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += self.name.size_of_count16();
        len += 2;
        len += 2;
        len += self.value.len();
        len
    }
}

impl<'a> ExtendedContentDescriptionData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, descriptors) = length_count(le_u16, ContentDescriptor::parse)(input)?;
        Ok((input, Self { descriptors }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let descriptors_len: u16 = self.descriptors.len().try_into()?;
        w.write_all(&descriptors_len.to_le_bytes())?;
        for descriptor in self.descriptors.iter() {
            descriptor.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 2;
        for descriptor in self.descriptors.iter() {
            len += descriptor.size_of();
        }
        len
    }
}
