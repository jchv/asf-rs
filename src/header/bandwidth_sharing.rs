use std::{convert::TryInto, io::Write};

use nom::{IResult, error::ParseError, multi::length_count, number::streaming::{le_u16, le_u32}};
use uuid::Uuid;

use crate::{guid::*, span::Span};


#[derive(Debug, PartialEq)]
pub struct BandwidthSharingData {
    pub sharing_type: Uuid,
    pub data_bitrate: u32,
    pub buffer_size: u32,
    pub stream_numbers: Vec<u16>,
}

impl BandwidthSharingData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, sharing_type) = guid(input)?;
        let (input, data_bitrate) = le_u32(input)?;
        let (input, buffer_size) = le_u32(input)?;
        let (input, stream_numbers) = length_count(le_u16, le_u16)(input)?;
        Ok((input, Self{
            sharing_type,
            data_bitrate,
            buffer_size,
            stream_numbers,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let stream_numbers_len: u16 = self.stream_numbers.len().try_into()?;
        w.write_all(&self.sharing_type.as_bytes_ms())?;
        w.write_all(&self.data_bitrate.to_le_bytes())?;
        w.write_all(&self.buffer_size.to_le_bytes())?;
        w.write_all(&stream_numbers_len.to_le_bytes())?;
        for stream_number in self.stream_numbers.iter() {
            w.write_all(&stream_number.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 4 + 4 + 2 + self.stream_numbers.len() * 2
    }
}
