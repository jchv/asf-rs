use nom::number::streaming::{le_u16, le_u32};
use uuid::Uuid;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct BandwidthSharingData {
    sharing_type: Uuid,
    data_bitrate: u32,
    buffer_size: u32,
    stream_numbers: Vec<u16>,
}

named!(pub bandwidth_sharing_data<BandwidthSharingData>,
    do_parse!(
        sharing_type: guid >>
        data_bitrate: le_u32 >>
        buffer_size: le_u32 >>
        stream_numbers: length_count!(le_u16, le_u16) >>
        (BandwidthSharingData{
            sharing_type,
            data_bitrate,
            buffer_size,
            stream_numbers,
        })
    )
);
