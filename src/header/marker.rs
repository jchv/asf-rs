use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32, le_u64};

use crate::{guid::*, widestr::*};


#[derive(Debug, PartialEq)]
pub struct Marker {
    pub offset: u64,
    pub presentation_time: u64,
    pub send_time: u32,
    pub flags: u32,
    pub marker_description: WideStr,
}

#[derive(Debug, PartialEq)]
pub struct MarkerData {
    pub reserved_1: Uuid,
    pub reserved_2: u16,
    pub name: WideStr,
    pub markers: Vec<Marker>,
}

impl Marker {
    named!(pub parse<Self>,
        do_parse!(
            offset: le_u64 >>
            presentation_time: le_u64 >>
            entry_length: le_u16 >>
            send_time: le_u32 >>
            flags: le_u32 >>
            marker_description: len32_prefixed_widestr >>
            (Self{offset, presentation_time, send_time, flags, marker_description})
        )
    );
}

impl MarkerData {
    named!(pub parse<Self>,
        do_parse!(
            reserved_1: guid >>
            markers_count: le_u32 >>
            reserved_2: le_u16 >>
            name: len16_prefixed_widestr >>
            markers: count!(Marker::parse, markers_count as _) >>
            (Self{reserved_1, reserved_2, name, markers})
        )
    );
}
