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
}