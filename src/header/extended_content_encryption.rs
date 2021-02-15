use std::{convert::TryInto, io::Write};

use nom::{IResult, bytes::streaming::take, error::ParseError, number::streaming::le_u32};


#[derive(Debug, PartialEq)]
pub struct ExtendedContentEncryptionData<'a> {
    pub data: &'a [u8],
}

impl<'a> ExtendedContentEncryptionData<'a> {
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, data_size) = le_u32(input)?;
        let (input, data) = take(data_size)(input)?;
        Ok((input, ExtendedContentEncryptionData{
            data,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let data_len: u32 = self.data.len().try_into()?;
        w.write_all(&data_len.to_le_bytes())?;
        w.write_all(self.data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + self.data.len()
    }
}
