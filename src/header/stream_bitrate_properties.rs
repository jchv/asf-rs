use std::{convert::TryInto, io::Write};

use nom::{IResult, error::ParseError, multi::length_count, number::streaming::{le_u16, le_u32}};

use crate::span::Span;


#[derive(Debug, PartialEq)]
pub struct BitrateRecord {
    pub flags: u16,
    pub average_bitrate: u32,
}

#[derive(Debug, PartialEq)]
pub struct StreamBitratePropertiesData {
    pub bitrate_records: Vec<BitrateRecord>
}

impl BitrateRecord {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, flags) = le_u16(input)?;
        let (input, average_bitrate) = le_u32(input)?;
        Ok((input, Self{flags, average_bitrate}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.flags.to_le_bytes())?;
        w.write_all(&self.average_bitrate.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 4
    }
}

impl StreamBitratePropertiesData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, bitrate_records) = length_count(le_u16, BitrateRecord::parse)(input)?;
        Ok((input, Self{bitrate_records}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let bitrate_records_len: u16 = self.bitrate_records.len().try_into()?;
        w.write_all(&bitrate_records_len.to_le_bytes())?;
        for bitrate_record in self.bitrate_records.iter() {
            bitrate_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.bitrate_records.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
