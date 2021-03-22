use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::{IResult, error::ParseError, multi::length_count, number::streaming::{le_u16}};

use crate::{guid::*, span::Span};


#[derive(Debug, PartialEq)]
pub struct GroupMutualExclusionData {
    pub exclusion_type: Uuid,
    pub records: Vec<Vec<u16>>,
}

impl GroupMutualExclusionData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, exclusion_type) = guid(input)?;
        let (input, records) = length_count(le_u16, length_count(le_u16, le_u16))(input)?;
        Ok((input, Self{exclusion_type, records}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let records_len: u16 = self.records.len().try_into()?;
        w.write_all(&self.exclusion_type.as_bytes_ms())?;
        w.write_all(&records_len.to_le_bytes())?;
        for record in &self.records {
            let record_len: u16 = record.len().try_into()?;
            w.write_all(&record_len.to_le_bytes())?;
            for record in record {
                w.write_all(&record.to_le_bytes())?;
            }
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 2 + self.records.iter().map(|x| 2 + 2 * x.len()).sum::<usize>()
    }
}
