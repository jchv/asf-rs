use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct HeaderExtensionData<'a> {
    reserved_1: Uuid,
    reserved_2: u16,
    extension_data: &'a [u8],
}

named!(pub header_extension_data<HeaderExtensionData>,
    do_parse!(
        reserved_1: guid >>
        reserved_2: le_u16 >>
        extension_data_size: le_u32 >>
        extension_data: take!(extension_data_size) >>
        (HeaderExtensionData{reserved_1, reserved_2, extension_data})
    )
);
