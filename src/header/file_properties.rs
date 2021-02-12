use std::io::Write;

use uuid::Uuid;
use nom::number::streaming::{le_u32, le_u64};

use crate::guid::*;

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
    named!(pub parse<Self>,
        do_parse!(
            file_id: guid >>
            file_size: le_u64 >>
            creation_date: le_u64 >>
            data_packets_count: le_u64 >>
            play_duration: le_u64 >>
            send_duration: le_u64 >>
            preroll: le_u64 >>
            flags: le_u32 >>
            minimum_data_packet_size: le_u32 >>
            maximum_data_packet_size: le_u32 >>
            maximum_bitrate: le_u32 >>

            (Self{
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
            })
        )
    );

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
