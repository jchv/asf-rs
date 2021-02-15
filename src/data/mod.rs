pub mod packets;

use nom::{ErrorConvert, IResult, bytes::streaming::{tag, take}, combinator::complete, error::ParseError, multi::count, number::streaming::{le_u16, le_u64}};
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
    pub fn parse<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], DataObject, E2> {
        let (input, _data_object_guid) = tag(DATA_OBJECT.as_bytes_ms())(input)?;
        let (input, size) = le_u64(input)?;
        let (input, file_id) = guid(input)?;
        let (input, total_data_packets) = le_u64(input)?;
        let (input, reserved) = le_u16(input)?;
        let (input, data) = take(size - 50)(input)?;
        Ok((input, DataObject{
            file_id, total_data_packets, reserved, packets: complete(count(DataPacket::parse::<E1, E2>, total_data_packets as usize))(data)?.1
        }))
    }
}
