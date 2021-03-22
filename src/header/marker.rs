use crate::{guid::*, span::Span, widestr::*};
use nom::{
    error::ParseError,
    multi::count,
    number::streaming::{le_u16, le_u32, le_u64},
    IResult,
};
use std::{convert::TryInto, io::Write};
use uuid::Uuid;

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
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, offset) = le_u64(input)?;
        let (input, presentation_time) = le_u64(input)?;
        let (input, entry_length) = le_u16(input)?;
        let (input, send_time) = le_u32(input)?;
        let (input, flags) = le_u32(input)?;
        let (input, marker_description) = WideStr::parse_count32(input)?;
        Ok((
            input,
            Self {
                offset,
                presentation_time,
                entry_length,
                send_time,
                flags,
                marker_description,
            },
        ))
    }

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
        let mut len = 0;
        len += 8;
        len += 8;
        len += 2;
        len += 4;
        len += 4;
        len += self.marker_description.size_of_count32();
        len
    }
}

impl MarkerData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, reserved_1) = guid(input)?;
        let (input, markers_count) = le_u32(input)?;
        let (input, reserved_2) = le_u16(input)?;
        let (input, name) = WideStr::parse_count16(input)?;
        let (input, markers) = count(Marker::parse, markers_count as _)(input)?;
        Ok((
            input,
            Self {
                reserved_1,
                reserved_2,
                name,
                markers,
            },
        ))
    }

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
        let mut len = 0;
        len += 16;
        len += 4;
        len += 2;
        len += self.name.size_of_count16();
        for marker in self.markers.iter() {
            len += 1;
            len += marker.size_of();
        }
        len
    }
}
