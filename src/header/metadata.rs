use crate::{span::Span, widestr::*};
use nom::{
    bytes::streaming::take,
    error::ParseError,
    multi::length_count,
    number::streaming::{le_u16, le_u32},
    IResult,
};
use std::{convert::TryInto, io::Write};

#[derive(Debug, PartialEq)]
pub struct DescriptionRecord<'a> {
    pub reserved: u16,
    pub stream_number: u16,
    pub data_type: u16,
    pub name: WideStr,
    pub data: Span<'a>,
}

#[derive(Debug, PartialEq)]
pub struct MetadataData<'a> {
    pub description_records: Vec<DescriptionRecord<'a>>,
}

impl<'a> DescriptionRecord<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, reserved) = le_u16(input)?;
        let (input, stream_number) = le_u16(input)?;
        let (input, name_length) = le_u16(input)?;
        let (input, data_type) = le_u16(input)?;
        let (input, data_length) = le_u32(input)?;
        let (input, name) = take(name_length)(input)?;
        let (input, data) = take(data_length)(input)?;
        Ok((
            input,
            Self {
                reserved,
                stream_number,
                data_type,
                name: WideStr::parse(name)?.1,
                data,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let name_len: u16 = self.name.size_of().try_into()?;
        let data_len: u16 = self.data.len().try_into()?;
        w.write_all(&self.reserved.to_le_bytes())?;
        w.write_all(&self.stream_number.to_le_bytes())?;
        w.write_all(&name_len.to_le_bytes())?;
        w.write_all(&self.data_type.to_le_bytes())?;
        w.write_all(&data_len.to_le_bytes())?;
        self.name.write(w)?;
        w.write_all(&self.data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 2;
        len += 2;
        len += 2;
        len += 2;
        len += 4;
        len += self.name.size_of();
        len += self.data.len();
        len
    }
}

impl<'a> MetadataData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, description_records) = length_count(le_u16, DescriptionRecord::parse)(input)?;
        Ok((
            input,
            MetadataData {
                description_records,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let description_records_len: u16 = self.description_records.len().try_into()?;
        w.write_all(&description_records_len.to_le_bytes())?;
        for description_record in self.description_records.iter() {
            description_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 2;
        for description_record in self.description_records.iter() {
            len += description_record.size_of();
        }
        len
    }
}
