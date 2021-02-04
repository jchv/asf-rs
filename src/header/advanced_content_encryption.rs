use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct EncryptedObjectRecord<'a> {
    object_type: u16,
    data: &'a [u8],
}

named!(pub encrypted_object_record<EncryptedObjectRecord>,
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

#[derive(Debug, PartialEq)]
pub struct ContentEncryptionRecord<'a> {
    system_id: Uuid,
    system_version: u32,
    encrypted_object_records: Vec<EncryptedObjectRecord<'a>>,
    data: &'a [u8],
}

named!(pub content_encryption_record<ContentEncryptionRecord>,
    do_parse!(
        system_id: guid >>
        system_version: le_u32 >>
        encrypted_object_records: length_count!(le_u16, encrypted_object_record) >>
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

#[derive(Debug, PartialEq)]
pub struct AdvancedContentEncryptionData<'a> {
    content_encryption_records: Vec<ContentEncryptionRecord<'a>>,
}

named!(pub advanced_content_encryption_data<AdvancedContentEncryptionData>,
    do_parse!(
        content_encryption_records: length_count!(le_u16, content_encryption_record) >>
        (AdvancedContentEncryptionData{
            content_encryption_records,
        })
    )
);
