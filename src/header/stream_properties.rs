use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::{IResult, bytes::streaming::take, error::ParseError, number::streaming::{le_u16, le_u32, le_u64}};

use crate::{guid::*, span::Span};


#[derive(Debug, PartialEq)]
pub struct StreamPropertiesData<'a> {
    pub stream_type: Uuid,
    pub error_correction_type: Uuid,
    pub time_offset: u64,
    pub flags: u16,
    pub reserved: u32,
    pub type_specific_data: Span<'a>,
    pub error_correction_data: Span<'a>,
}

impl<'a> StreamPropertiesData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, stream_type) = guid(input)?;
        let (input, error_correction_type) = guid(input)?;
        let (input, time_offset) = le_u64(input)?;
        let (input, type_specific_data_len) = le_u32(input)?;
        let (input, error_correction_data_len) = le_u32(input)?;
        let (input, flags) = le_u16(input)?;
        let (input, reserved) = le_u32(input)?;
        let (input, type_specific_data) = take(type_specific_data_len)(input)?;
        let (input, error_correction_data) = take(error_correction_data_len)(input)?;

        Ok((input, Self{
            stream_type,
            error_correction_type,
            time_offset,
            flags,
            reserved,
            type_specific_data,
            error_correction_data,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let type_specific_data_len: u32 = self.type_specific_data.len().try_into()?;
        let error_correction_data_len: u32 = self.error_correction_data.len().try_into()?;
        w.write_all(&self.stream_type.as_bytes_ms())?;
        w.write_all(&self.error_correction_type.as_bytes_ms())?;
        w.write_all(&self.time_offset.to_le_bytes())?;
        w.write_all(&type_specific_data_len.to_le_bytes())?;
        w.write_all(&error_correction_data_len.to_le_bytes())?;
        w.write_all(&self.flags.to_le_bytes())?;
        w.write_all(&self.reserved.to_le_bytes())?;
        w.write_all(&self.type_specific_data)?;
        w.write_all(&self.error_correction_data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 16 + 8 + 4 + 4 + 2 + 4 + self.type_specific_data.len() + self.error_correction_data.len()
    }
}

#[cfg(test)]
mod tests {
    use nom::{AsBytes, Slice, error::VerboseError};

    use crate::header::*;

    use super::*;

    const BASIC_STREAM_PROPERTIES_BYTES: &'static [u8] = &[
        0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6,
        0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65, 0x72, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 
        0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B, 
        0x50, 0xCD, 0xC3, 0xBF, 0x8F, 0x61, 0xCF, 0x11, 0x8B, 0xB2, 
        0x00, 0xAA, 0x00, 0xB4, 0xE2, 0x20, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x08, 0x00, 
        0x00, 0x00, 0x01, 0x00, 0x70, 0x33, 0x77, 0x00, 0x61, 0x01, 
        0x01, 0x00, 0x80, 0x3E, 0x00, 0x00, 0xD0, 0x07, 0x00, 0x00, 
        0x80, 0x02, 0x10, 0x00, 0x0A, 0x00, 0x00, 0x22, 0x00, 0x00, 
        0x0E, 0x00, 0x80, 0x07, 0x00, 0x00, 0x01, 0x80, 0x02, 0x80,
        0x02, 0x01, 0x00, 0x00,
    ];

    fn basic_stream_properties_data() -> StreamPropertiesData<'static> {
        let span = Span::new(BASIC_STREAM_PROPERTIES_BYTES);
        StreamPropertiesData{
            stream_type: AUDIO_MEDIA,
            error_correction_type: AUDIO_SPREAD,
            time_offset: 0,
            flags: 1,
            reserved: 7811952,
            type_specific_data: span.slice(78..106),
            error_correction_data: span.slice(106..),
        }
    }

    #[test]
    fn parse_basic_stream_properties() {
        assert_eq!(
            HeaderObject::parse::<VerboseError<_>>(Span::new(BASIC_STREAM_PROPERTIES_BYTES)).expect("parse error").1,
            HeaderObject::StreamProperties(basic_stream_properties_data()),
        );
    }

    #[test]
    fn write_basic_stream_properties() {
        let mut buf = Vec::new();

        HeaderObject::StreamProperties(basic_stream_properties_data()).write(&mut buf).expect("write to succeed");

        assert_eq!(buf.as_bytes(), &BASIC_STREAM_PROPERTIES_BYTES[..])
    }

    #[test]
    fn size_of_basic_stream_properties() {
        assert_eq!(
            HeaderObject::StreamProperties(basic_stream_properties_data()).size_of(),
            BASIC_STREAM_PROPERTIES_BYTES.len()
        )
    }
}
