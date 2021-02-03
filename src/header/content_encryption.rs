use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct ContentEncryptionData<'a> {
    secret_data: &'a [u8],
    protection_type: &'a [u8],
    key_id: &'a [u8],
    license_url: &'a [u8],
}

named!(pub content_encryption_data<ContentEncryptionData>,
    do_parse!(
        secret_data_length: le_u32 >>
        secret_data: take!(secret_data_length) >>
        protection_type_length: le_u32 >>
        protection_type: take!(protection_type_length) >>
        key_id_length: le_u32 >>
        key_id: take!(key_id_length) >>
        license_url_length: le_u32 >>
        license_url: take!(license_url_length) >>
        (ContentEncryptionData{
            secret_data,
            protection_type,
            key_id,
            license_url,
        })
    )
);
