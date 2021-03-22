use std::{convert::TryInto, io::Write};

use nom::{IResult, bytes::streaming::take, error::ParseError, number::streaming::le_u32};

use crate::span::Span;


#[derive(Debug, PartialEq)]
pub struct DigitalSignatureData<'a> {
    pub signature_type: u32,
    pub signature_data: Span<'a>,
}

impl<'a> DigitalSignatureData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, signature_type) = le_u32(input)?;
        let (input, signature_data_size) = le_u32(input)?;
        let (input, signature_data) = take(signature_data_size)(input)?;
        Ok((input, DigitalSignatureData{
            signature_type,
            signature_data,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let signature_data_len: u32 = self.signature_data.len().try_into()?;
        w.write_all(&self.signature_type.to_le_bytes())?;
        w.write_all(&signature_data_len.to_le_bytes())?;
        w.write_all(&self.signature_data)?;

        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + 4 + self.signature_data.len()
    }
}
