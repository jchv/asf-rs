pub mod packets;

use nom::{combinator::complete, multi::count, number::streaming::{le_u16, le_u64}};
use packets::DataPacket;
use uuid::Uuid;

use crate::guid::*;

#[derive(Debug, PartialEq)]
pub struct DataObject<'a> {
    file_id: Uuid,
    total_data_packets: u64,
    reserved: u16,
    packets: Vec<DataPacket<'a>>,
}

impl<'a> DataObject<'a> {
    named!(pub parse<DataObject>,
        do_parse!(
            data_object_guid: tag!(DATA_OBJECT.as_bytes_ms()) >>
            size: le_u64 >>
            file_id: guid >>
            total_data_packets: le_u64 >>
            reserved: le_u16 >>
            data: take!(size - 50) >>
            (DataObject{file_id, total_data_packets, reserved, packets: complete(count(DataPacket::parse, total_data_packets as usize))(data)?.1})
        )
    );
}
