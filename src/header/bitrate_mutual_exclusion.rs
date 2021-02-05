use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::number::streaming::{le_u16};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct BitrateMutualExclusionData {
    pub exclusion_type: Uuid,
    pub stream_numbers: Vec<u16>,
}

impl BitrateMutualExclusionData {
    named!(pub parse<Self>,
        do_parse!(
            exclusion_type: guid >>
            stream_numbers: length_count!(le_u16, le_u16) >>
            (Self{exclusion_type, stream_numbers})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let stream_numbers_len: u16 = self.stream_numbers.len().try_into()?;
        w.write_all(&self.exclusion_type.as_bytes_ms())?;
        w.write_all(&stream_numbers_len.to_le_bytes())?;
        for stream_number in self.stream_numbers.iter() {
            w.write_all(&stream_number.to_le_bytes())?;
        }
        Ok(())
    }
}
