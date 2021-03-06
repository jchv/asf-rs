use crate::{guid::*, span::Span};
use nom::{bytes::streaming::take, error::ParseError, number::streaming::le_u32, IResult};
use std::{convert::TryInto, io::Write};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct ErrorCorrectionData<'a> {
    pub error_correction_type: Uuid,
    pub error_correction_data: Span<'a>,
}

impl<'a> ErrorCorrectionData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, error_correction_type) = guid(input)?;
        let (input, error_correction_data_length) = le_u32(input)?;
        let (input, error_correction_data) = take(error_correction_data_length)(input)?;

        Ok((
            input,
            Self {
                error_correction_type,
                error_correction_data,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let error_correction_data_len: u32 = self.error_correction_data.len().try_into()?;
        w.write_all(&self.error_correction_type.as_bytes_ms())?;
        w.write_all(&error_correction_data_len.to_le_bytes())?;
        w.write_all(&self.error_correction_data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 16;
        len += 4;
        len += self.error_correction_data.len();
        len
    }
}
