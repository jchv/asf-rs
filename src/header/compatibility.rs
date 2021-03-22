use std::io::Write;

use nom::{IResult, error::ParseError, number::streaming::le_u8};

use crate::span::Span;


#[derive(Debug, PartialEq)]
pub struct CompatibilityData {
    pub profile: u8,
    pub mode: u8,
}

impl CompatibilityData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, profile) = le_u8(input)?;
        let (input, mode) = le_u8(input)?;
        Ok((input, Self{
            profile,
            mode,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.profile.to_le_bytes())?;
        w.write_all(&self.mode.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 2
    }
}
