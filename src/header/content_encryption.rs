use std::{convert::TryInto, io::Write};

use nom::{IResult, bytes::streaming::take, error::ParseError, number::streaming::le_u32};

use crate::span::Span;


#[derive(Debug, PartialEq)]
pub struct ContentEncryptionData<'a> {
    pub secret_data: Span<'a>,
    pub protection_type: Span<'a>,
    pub key_id: Span<'a>,
    pub license_url: Span<'a>,
}

impl<'a> ContentEncryptionData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, secret_data_length) = le_u32(input)?;
        let (input, secret_data) = take(secret_data_length)(input)?;
        let (input, protection_type_length) = le_u32(input)?;
        let (input, protection_type) = take(protection_type_length)(input)?;
        let (input, key_id_length) = le_u32(input)?;
        let (input, key_id) = take(key_id_length)(input)?;
        let (input, license_url_length) = le_u32(input)?;
        let (input, license_url) = take(license_url_length)(input)?;

        Ok((input, Self{
            secret_data,
            protection_type,
            key_id,
            license_url,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let secret_data_len: u32 = self.secret_data.len().try_into()?;
        let protection_type_len: u32 = self.protection_type.len().try_into()?;
        let key_id_len: u32 = self.key_id.len().try_into()?;
        let license_url_len: u32 = self.license_url.len().try_into()?;

        w.write_all(&secret_data_len.to_le_bytes())?;
        w.write_all(&self.secret_data)?;
        w.write_all(&protection_type_len.to_le_bytes())?;
        w.write_all(&self.protection_type)?;
        w.write_all(&key_id_len.to_le_bytes())?;
        w.write_all(&self.key_id)?;
        w.write_all(&license_url_len.to_le_bytes())?;
        w.write_all(&self.license_url)?;

        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + self.secret_data.len() +
        4 + self.protection_type.len() +
        4 + self.key_id.len() +
        4 + self.license_url.len()
    }
}

#[cfg(test)]
mod tests {
    use nom::{AsBytes, Slice, error::VerboseError};

    use crate::header::*;

    use super::*;

    const BASIC_CONTENT_ENCRYPTION_BYTES: &'static [u8] = &[
        0xFB, 0xB3, 0x11, 0x22, 0x23, 0xBD, 0xD2, 0x11, 0xB4, 0xB7, 0x00, 0xA0,
        0xC9, 0x55, 0xFC, 0x6E, 0xBC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0xB8, 0xE8, 0x9C, 0xBB, 0x79, 0x31, 0x80, 0x5C,
        0x3D, 0x7F, 0xDD, 0x00, 0xC5, 0x5C, 0xE9, 0xBA, 0x80, 0x3B, 0x1A, 0x5C,
        0xFB, 0x81, 0xDA, 0xF9, 0x04, 0x00, 0x00, 0x00, 0x44, 0x52, 0x4D, 0x00,
        0x19, 0x00, 0x00, 0x00, 0x4C, 0x35, 0x33, 0x6C, 0x51, 0x67, 0x74, 0x71,
        0x53, 0x41, 0x45, 0x63, 0x46, 0x36, 0x30, 0x35, 0x43, 0x54, 0x4F, 0x74,
        0x37, 0x59, 0x55, 0x6A, 0x00, 0x5F, 0x00, 0x00, 0x00, 0x68, 0x74, 0x74,
        0x70, 0x3A, 0x2F, 0x2F, 0x67, 0x6F, 0x2E, 0x6D, 0x69, 0x63, 0x72, 0x6F,
        0x73, 0x6F, 0x66, 0x74, 0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x66, 0x77, 0x6C,
        0x69, 0x6E, 0x6B, 0x2F, 0x3F, 0x70, 0x72, 0x64, 0x3D, 0x38, 0x31, 0x36,
        0x26, 0x70, 0x76, 0x65, 0x72, 0x3D, 0x37, 0x2E, 0x31, 0x26, 0x73, 0x62,
        0x70, 0x3D, 0x44, 0x52, 0x4D, 0x26, 0x70, 0x6C, 0x63, 0x69, 0x64, 0x3D,
        0x30, 0x78, 0x34, 0x30, 0x39, 0x26, 0x63, 0x6C, 0x63, 0x69, 0x64, 0x3D,
        0x30, 0x78, 0x34, 0x30, 0x39, 0x26, 0x61, 0x72, 0x3D, 0x50, 0x65, 0x72,
        0x73, 0x6F, 0x6E, 0x61, 0x6C, 0x56, 0x32, 0x00,
    ];

    fn basic_content_encryption_data() -> ContentEncryptionData<'static> {
        let span = Span::new(BASIC_CONTENT_ENCRYPTION_BYTES);
        ContentEncryptionData {
            secret_data: span.slice(28..52),
            protection_type: span.slice(56..60),
            key_id: span.slice(64..89),
            license_url: span.slice(93..)
        }
    }

    #[test]
    fn parse_basic_content_encryption() {
        assert_eq!(
            HeaderObject::parse::<VerboseError<_>>(Span::new(BASIC_CONTENT_ENCRYPTION_BYTES)).expect("parser error").1,
            HeaderObject::ContentEncryption(basic_content_encryption_data())
        )
    }

    #[test]
    fn write_basic_content_encryption() {
        let mut buf = Vec::new();

        HeaderObject::ContentEncryption(basic_content_encryption_data()).write(&mut buf).expect("write to succeed");

        assert_eq!(buf.as_bytes(), &BASIC_CONTENT_ENCRYPTION_BYTES[..])
    }

    #[test]
    fn size_of_basic_content_encryption() {
        assert_eq!(
            HeaderObject::ContentEncryption(basic_content_encryption_data()).size_of(),
            BASIC_CONTENT_ENCRYPTION_BYTES.len()
        )
    }
}
