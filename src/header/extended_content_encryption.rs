use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct ExtendedContentEncryptionData<'a> {
    data: &'a [u8],
}

named!(pub extended_content_encryption_data<ExtendedContentEncryptionData>,
    do_parse!(
        data_size: le_u32 >>
        data: take!(data_size) >>
        (ExtendedContentEncryptionData{
            data,
        })
    )
);
