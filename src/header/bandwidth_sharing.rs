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
}
