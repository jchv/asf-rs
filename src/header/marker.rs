use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32, le_u64};

use crate::{guid::*, widestr::*};


#[derive(Debug, PartialEq)]
pub struct Marker {
    offset: u64,
    presentation_time: u64,
    send_time: u32,
    flags: u32,
    marker_description: WideStr,
}

named!(pub marker<Marker>,
    do_parse!(
        offset: le_u64 >>
        presentation_time: le_u64 >>
        entry_length: le_u16 >>
        send_time: le_u32 >>
        flags: le_u32 >>
        marker_description: len32_prefixed_widestr >>
        (Marker{offset, presentation_time, send_time, flags, marker_description})
    )
);

#[derive(Debug, PartialEq)]
pub struct MarkerData {
    reserved_1: Uuid,
    reserved_2: u16,
    name: WideStr,
    markers: Vec<Marker>,
}

named!(pub marker_data<MarkerData>,
    do_parse!(
        reserved_1: guid >>
        markers_count: le_u32 >>
        reserved_2: le_u16 >>
        name: len16_prefixed_widestr >>
        markers: count!(marker, markers_count as _) >>
        (MarkerData{reserved_1, reserved_2, name, markers})
    )
);
