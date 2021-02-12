use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32, le_u64};

use crate::{guid::*, widestr::*};


#[derive(Debug, PartialEq)]
pub struct Marker {
    pub offset: u64,
    pub presentation_time: u64,
    pub entry_length: u16,
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
            marker_description: call!(WideStr::parse_count32) >>
            (Self{offset, presentation_time, entry_length, send_time, flags, marker_description})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.offset.to_le_bytes())?;
        w.write_all(&self.presentation_time.to_le_bytes())?;
        w.write_all(&self.entry_length.to_le_bytes())?;
        w.write_all(&self.send_time.to_le_bytes())?;
        w.write_all(&self.flags.to_le_bytes())?;
        self.marker_description.write_count32(w)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        8 + 8 + 2 + 4 + 4 + self.marker_description.size_of_count32()
    }
}

impl MarkerData {
    named!(pub parse<Self>,
        do_parse!(
            reserved_1: guid >>
            markers_count: le_u32 >>
            reserved_2: le_u16 >>
            name: call!(WideStr::parse_count16) >>
            markers: count!(Marker::parse, markers_count as _) >>
            (Self{reserved_1, reserved_2, name, markers})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let markers_len: u32 = self.markers.len().try_into()?;
        w.write_all(&self.reserved_1.as_bytes_ms())?;
        w.write_all(&markers_len.to_le_bytes())?;
        w.write_all(&self.reserved_2.to_le_bytes())?;
        self.name.write_count16(w)?;
        for marker in self.markers.iter() {
            marker.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 4 + 2 + self.name.size_of_count16() + self.markers.iter().map(|x| 1 + x.size_of()).sum::<usize>()
    }
}
