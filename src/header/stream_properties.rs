use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32, le_u64};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct StreamPropertiesData<'a> {
    pub stream_type: Uuid,
    pub error_correction_type: Uuid,
    pub time_offset: u64,
    pub flags: u16,
    pub reserved: u32,
    pub type_specific_data: &'a [u8],
    pub error_correction_data: &'a [u8],
}

impl<'a> StreamPropertiesData<'a> {
    named!(pub parse<StreamPropertiesData>,
        do_parse!(
            stream_type: guid >>
            error_correction_type: guid >>
            time_offset: le_u64 >>
            type_specific_data_len: le_u32 >>
            error_correction_data_len: le_u32 >>
            flags: le_u16 >>
            reserved: le_u32 >>
            type_specific_data: take!(type_specific_data_len) >>
            error_correction_data: take!(error_correction_data_len) >>

            (StreamPropertiesData{
                stream_type,
                error_correction_type,
                time_offset,
                flags,
                reserved,
                type_specific_data,
                error_correction_data,
            })
        )
    );

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
        w.write_all(self.type_specific_data)?;
        w.write_all(self.error_correction_data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 16 + 8 + 4 + 4 + 2 + 4 + self.type_specific_data.len() + self.error_correction_data.len()
    }
}

#[cfg(test)]
mod tests {
    use nom::AsBytes;

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

    const BASIC_STREAM_PROPERTIES_DATA: StreamPropertiesData = StreamPropertiesData{
        stream_type: AUDIO_MEDIA,
        error_correction_type: AUDIO_SPREAD,
        time_offset: 0,
        flags: 1,
        reserved: 7811952,
        type_specific_data: &[
            0x61, 0x01, 0x01, 0x00, 0x80, 0x3E, 0x00, 0x00,
            0xD0, 0x07, 0x00, 0x00, 0x80, 0x02, 0x10, 0x00,
            0x0A, 0x00, 0x00, 0x22, 0x00, 0x00, 0x0E, 0x00,
            0x80, 0x07, 0x00, 0x00,
        ],
        error_correction_data: &[
            0x01, 0x80, 0x02, 0x80, 0x02, 0x01, 0x00, 0x00,
        ],
    };

    #[test]
    fn parse_basic_stream_properties() {
        assert_eq!(
            HeaderObject::parse(BASIC_STREAM_PROPERTIES_BYTES),
            Ok((&b""[..], HeaderObject::StreamProperties(BASIC_STREAM_PROPERTIES_DATA))),
        );
    }

    #[test]
    fn write_basic_stream_properties() {
        let mut buf = Vec::new();

        HeaderObject::StreamProperties(BASIC_STREAM_PROPERTIES_DATA).write(&mut buf).expect("write to succeed");

        assert_eq!(buf.as_bytes(), &BASIC_STREAM_PROPERTIES_BYTES[..])
    }

    #[test]
    fn size_of_basic_stream_properties() {
        assert_eq!(
            HeaderObject::StreamProperties(BASIC_STREAM_PROPERTIES_DATA).size_of(),
            BASIC_STREAM_PROPERTIES_BYTES.len()
        )
    }
}
