use uuid::Uuid;
use nom::number::streaming::{le_u32, le_u64};

use crate::guid::*;

#[derive(Debug, PartialEq)]
pub struct FilePropertiesData {
    file_id: Uuid,
    file_size: u64,
    creation_date: u64,
    data_packets_count: u64,
    play_duration: u64,
    send_duration: u64,
    preroll: u64,
    flags: u32,
    minimum_data_packet_size: u32,
    maximum_data_packet_size: u32,
    maximum_bitrate: u32,
}

named!(pub file_properties_data<FilePropertiesData>,
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

        (FilePropertiesData{
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
