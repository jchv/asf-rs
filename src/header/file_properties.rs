use std::io::Write;

use uuid::Uuid;
use nom::{IResult, error::ParseError, number::streaming::{le_u32, le_u64}};

use crate::{guid::*, span::Span};

#[derive(Debug, PartialEq)]
pub struct FilePropertiesData {
    pub file_id: Uuid,
    pub file_size: u64,
    pub creation_date: u64,
    pub data_packets_count: u64,
    pub play_duration: u64,
    pub send_duration: u64,
    pub preroll: u64,
    pub flags: u32,
    pub minimum_data_packet_size: u32,
    pub maximum_data_packet_size: u32,
    pub maximum_bitrate: u32,
}

impl FilePropertiesData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, file_id) = guid(input)?;
        let (input, file_size) = le_u64(input)?;
        let (input, creation_date) = le_u64(input)?;
        let (input, data_packets_count) = le_u64(input)?;
        let (input, play_duration) = le_u64(input)?;
        let (input, send_duration) = le_u64(input)?;
        let (input, preroll) = le_u64(input)?;
        let (input, flags) = le_u32(input)?;
        let (input, minimum_data_packet_size) = le_u32(input)?;
        let (input, maximum_data_packet_size) = le_u32(input)?;
        let (input, maximum_bitrate) = le_u32(input)?;

        Ok((input, Self{
            file_id,
            file_size,
            creation_date,
            data_packets_count,
            play_duration,
            send_duration,
            preroll,
            flags,
            minimum_data_packet_size,
            maximum_data_packet_size,
            maximum_bitrate,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.file_id.as_bytes_ms())?;
        w.write_all(&self.file_size.to_le_bytes())?;
        w.write_all(&self.creation_date.to_le_bytes())?;
        w.write_all(&self.data_packets_count.to_le_bytes())?;
        w.write_all(&self.play_duration.to_le_bytes())?;
        w.write_all(&self.send_duration.to_le_bytes())?;
        w.write_all(&self.preroll.to_le_bytes())?;
        w.write_all(&self.flags.to_le_bytes())?;
        w.write_all(&self.minimum_data_packet_size.to_le_bytes())?;
        w.write_all(&self.maximum_data_packet_size.to_le_bytes())?;
        w.write_all(&self.maximum_bitrate.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 8 + 8 + 8 + 8 + 8 + 8 + 4 + 4 + 4 + 4
    }
}
