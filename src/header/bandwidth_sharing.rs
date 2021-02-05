use std::{convert::TryInto, io::Write};

use nom::number::streaming::{le_u16, le_u32};
use uuid::Uuid;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct BandwidthSharingData {
    pub sharing_type: Uuid,
    pub data_bitrate: u32,
    pub buffer_size: u32,
    pub stream_numbers: Vec<u16>,
}

impl BandwidthSharingData {
    named!(pub parse<Self>,
        do_parse!(
            sharing_type: guid >>
            data_bitrate: le_u32 >>
            buffer_size: le_u32 >>
            stream_numbers: length_count!(le_u16, le_u16) >>
            (Self{
                sharing_type,
                data_bitrate,
                buffer_size,
                stream_numbers,
            })
        )
    );

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
