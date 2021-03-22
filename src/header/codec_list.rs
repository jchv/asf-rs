use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::{IResult, bytes::streaming::take, error::ParseError, multi::length_count, number::streaming::{le_u16, le_u32}};

use crate::{guid::*, span::Span, widestr::*};


#[derive(Debug, PartialEq)]
pub struct CodecEntry<'a> {
    pub codec_type: u16,
    pub codec_name: WideStr,
    pub codec_description: WideStr,
    pub codec_information: Span<'a>,
}

#[derive(Debug, PartialEq)]
pub struct CodecListData<'a> {
    pub reserved: Uuid,
    pub codec_entries: Vec<CodecEntry<'a>>,
}

impl<'a> CodecEntry<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, codec_type) = le_u16(input)?;
        let (input, codec_name) = WideStr::parse_count16(input)?;
        let (input, codec_description) = WideStr::parse_count16(input)?;
        let (input, codec_information_len) = le_u16(input)?;
        let (input, codec_information) = take(codec_information_len)(input)?;
        Ok((input, Self{codec_type, codec_name, codec_description, codec_information}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let codec_information_len: u16 = self.codec_information.len().try_into()?;
        w.write_all(&self.codec_type.to_le_bytes())?;
        self.codec_name.write_count16(w)?;
        self.codec_description.write_count16(w)?;
        w.write_all(&codec_information_len.to_le_bytes())?;
        w.write_all(&self.codec_information)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.codec_name.size_of_count16() + self.codec_description.size_of_count16() + 2 + self.codec_information.len()
    }
}

impl<'a> CodecListData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, reserved) = guid(input)?;
        let (input, codec_entries) = length_count(le_u32, CodecEntry::parse)(input)?;
        Ok((input, Self{reserved, codec_entries}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let codec_entries_len: u32 = self.codec_entries.len().try_into()?;
        w.write_all(&self.reserved.as_bytes_ms())?;
        w.write_all(&codec_entries_len.to_le_bytes())?;
        for codec_entry in self.codec_entries.iter() {
            codec_entry.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 4 + self.codec_entries.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
