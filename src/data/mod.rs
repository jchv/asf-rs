pub mod packets;

use crate::{error::Error, guid::*, span::Span};
use nom::error::context;
use nom::{
    bytes::streaming::{tag, take},
    multi::count,
    number::streaming::{le_u16, le_u64},
    IResult,
};
use packets::DataPacket;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct DataObject<'a> {
    file_id: Uuid,
    total_data_packets: u64,
    reserved: u16,
    packets: Vec<DataPacket<'a>>,
}

impl<'a> DataObject<'a> {
    pub fn parse(input: Span<'a>) -> IResult<Span<'a>, DataObject, Error<Span<'a>>> {
        context("DataObject", move |input: Span<'a>| {
            let (input, _data_object_guid) = tag(DATA_OBJECT.as_bytes_ms())(input)?;
            let (input, size) = le_u64(input)?;
            let (input, file_id) = guid(input)?;
            let (input, total_data_packets) = le_u64(input)?;
            let (input, reserved) = le_u16(input)?;
            let total_packet_len = size - 50;
            let (input, data) = take(total_packet_len)(input)?;

            Ok((
                input,
                DataObject {
                    file_id,
                    total_data_packets,
                    reserved,
                    packets: count(
                        DataPacket::parser(total_data_packets, total_packet_len),
                        total_data_packets as usize,
                    )(data)?
                    .1,
                },
            ))
        })(input)
    }
}
