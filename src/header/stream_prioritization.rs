use std::{convert::TryInto, io::Write};

use nom::{IResult, error::ParseError, multi::length_count, number::streaming::{le_u16}};

use crate::span::Span;


#[derive(Debug, PartialEq)]
pub struct PriorityRecord {
    pub stream_number: u16,
    pub priority_flags: u16,
}

#[derive(Debug, PartialEq)]
pub struct StreamPrioritizationData {
    pub priority_records: Vec<PriorityRecord>,
}

impl PriorityRecord {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, stream_number) = le_u16(input)?;
        let (input, priority_flags) = le_u16(input)?;
        Ok((input, Self{
            stream_number,
            priority_flags,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.stream_number.to_le_bytes())?;
        w.write_all(&self.priority_flags.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 2
    }
}

impl StreamPrioritizationData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, priority_records) = length_count(le_u16, PriorityRecord::parse)(input)?;
        Ok((input, Self{priority_records}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let priority_records_len: u16 = self.priority_records.len().try_into()?;
        w.write_all(&priority_records_len.to_le_bytes())?;
        for priority_record in self.priority_records.iter() {
            priority_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.priority_records.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
