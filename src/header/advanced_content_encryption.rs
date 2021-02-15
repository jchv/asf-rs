use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::{IResult, bytes::streaming::take, error::ParseError, multi::length_count, number::streaming::{le_u16, le_u32}};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct EncryptedObjectRecord<'a> {
    pub object_type: u16,
    pub data: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct ContentEncryptionRecord<'a> {
    pub system_id: Uuid,
    pub system_version: u32,
    pub encrypted_object_records: Vec<EncryptedObjectRecord<'a>>,
    pub data: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct AdvancedContentEncryptionData<'a> {
    pub content_encryption_records: Vec<ContentEncryptionRecord<'a>>,
}

impl<'a> EncryptedObjectRecord<'a> {
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, object_type) = le_u16(input)?;
        let (input, length) = le_u16(input)?;
        let (input, data) = take(length)(input)?;
        Ok((input, Self{
            object_type,
            data,
        }))
    }

    fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let data_size: u32 = self.data.len().try_into()?;
        w.write_all(&self.object_type.to_le_bytes())?;
        w.write_all(&data_size.to_le_bytes())?;
        w.write_all(self.data)?;
        Ok(())
    }

    fn size_of(&self) -> usize {
        2 + 2 + self.data.len()
    }
}

impl<'a> ContentEncryptionRecord<'a> {
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, system_id) = guid(input)?;
        let (input, system_version) = le_u32(input)?;
        let (input, encrypted_object_records) = length_count(le_u16, EncryptedObjectRecord::parse)(input)?;
        let (input, data_size) = le_u32(input)?;
        let (input, data) = take(data_size)(input)?;
        Ok((input, Self{
            system_id,
            system_version,
            encrypted_object_records,
            data,
        }))
    }

    fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_object_records_len: u16 = self.encrypted_object_records.len().try_into()?;
        let data_size: u32 = self.data.len().try_into()?;
        w.write_all(&self.system_id.as_bytes_ms())?;
        w.write_all(&self.system_version.to_le_bytes())?;
        w.write_all(&encrypted_object_records_len.to_le_bytes())?;
        for record in self.encrypted_object_records.iter() {
            record.write(w)?;
        }
        w.write_all(&data_size.to_le_bytes())?;
        w.write_all(self.data)?;
        Ok(())
    }

    fn size_of(&self) -> usize {
        16 + 4 + 2 + self.encrypted_object_records.iter().map(|x| x.size_of()).sum::<usize>() + 4 + self.data.len()
    }
}

impl<'a> AdvancedContentEncryptionData<'a> {
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, content_encryption_records) = length_count(le_u16, ContentEncryptionRecord::parse)(input)?;
        Ok((input, Self{
            content_encryption_records,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let content_encryption_records_len: u16 = self.content_encryption_records.len().try_into()?;
        w.write_all(&content_encryption_records_len.to_le_bytes())?;
        for record in self.content_encryption_records.iter() {
            record.write(w)?
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.content_encryption_records.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
