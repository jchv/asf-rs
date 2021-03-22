use crate::{guid::*, span::Span};
use nom::{error::ParseError, multi::length_count, number::streaming::le_u16, IResult};
use std::{convert::TryInto, io::Write};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct BitrateMutualExclusionData {
    pub exclusion_type: Uuid,
    pub stream_numbers: Vec<u16>,
}

impl BitrateMutualExclusionData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, exclusion_type) = guid(input)?;
        let (input, stream_numbers) = length_count(le_u16, le_u16)(input)?;
        Ok((
            input,
            Self {
                exclusion_type,
                stream_numbers,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let stream_numbers_len: u16 = self.stream_numbers.len().try_into()?;
        w.write_all(&self.exclusion_type.as_bytes_ms())?;
        w.write_all(&stream_numbers_len.to_le_bytes())?;
        for stream_number in self.stream_numbers.iter() {
            w.write_all(&stream_number.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 16;
        len += 2;
        len += self.stream_numbers.len() * 2;
        len
    }
}
