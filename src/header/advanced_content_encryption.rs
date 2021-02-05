use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

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
    named!(parse<EncryptedObjectRecord>,
        do_parse!(
            object_type: le_u16 >>
            length: le_u16 >>
            data: take!(length) >>
            (EncryptedObjectRecord{
                object_type,
                data,
            })
        )
    );

    fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let data_size: u32 = self.data.len().try_into()?;
        w.write_all(&self.object_type.to_le_bytes())?;
        w.write_all(&data_size.to_le_bytes())?;
        w.write_all(self.data)?;
        Ok(())
    }
}

impl<'a> ContentEncryptionRecord<'a> {
    named!(parse<ContentEncryptionRecord>,
        do_parse!(
            system_id: guid >>
            system_version: le_u32 >>
            encrypted_object_records: length_count!(le_u16, EncryptedObjectRecord::parse) >>
            data_size: le_u32 >>
            data: take!(data_size) >>
            (ContentEncryptionRecord{
                system_id,
                system_version,
                encrypted_object_records,
                data,
            })
        )
    );

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
}

impl<'a> AdvancedContentEncryptionData<'a> {
    named!(pub parse<AdvancedContentEncryptionData>,
        do_parse!(
            content_encryption_records: length_count!(le_u16, ContentEncryptionRecord::parse) >>
            (AdvancedContentEncryptionData{
                content_encryption_records,
            })
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let content_encryption_records_len: u16 = self.content_encryption_records.len().try_into()?;
        w.write_all(&content_encryption_records_len.to_le_bytes())?;
        for record in self.content_encryption_records.iter() {
            record.write(w)?
        }
        Ok(())
    }
}
