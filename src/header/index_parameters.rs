use std::{convert::TryInto, io::Write};

use nom::{IResult, error::ParseError, multi::length_count, number::streaming::{le_u16, le_u32}};

use crate::span::Span;


#[derive(Debug, PartialEq)]
pub struct IndexSpecifier {
    pub stream_number: u16,
    pub index_type: u16,
}

#[derive(Debug, PartialEq)]
pub struct IndexParametersData {
    pub index_entry_time_interval: u32,
    pub index_specifiers: Vec<IndexSpecifier>,
}

impl IndexSpecifier {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, stream_number) = le_u16(input)?;
        let (input, index_type) = le_u16(input)?;
        Ok((input, Self{
            stream_number,
            index_type,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.stream_number.to_le_bytes())?;
        w.write_all(&self.index_type.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 2
    }
}

impl IndexParametersData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, index_entry_time_interval) = le_u32(input)?;
        let (input, index_specifiers) = length_count(le_u16, IndexSpecifier::parse)(input)?;
        Ok((input, Self{
            index_entry_time_interval,
            index_specifiers,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let index_specifiers_len: u16 = self.index_specifiers.len().try_into()?;
        w.write_all(&self.index_entry_time_interval.to_le_bytes())?;
        w.write_all(&index_specifiers_len.to_le_bytes())?;
        for index_specifier in self.index_specifiers.iter() {
            index_specifier.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + 2 + self.index_specifiers.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
