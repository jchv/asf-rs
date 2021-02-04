use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct ExtendedContentEncryptionData<'a> {
    pub data: &'a [u8],
}

impl<'a> ExtendedContentEncryptionData<'a> {
    named!(pub parse<ExtendedContentEncryptionData>,
        do_parse!(
            data_size: le_u32 >>
            data: take!(data_size) >>
            (ExtendedContentEncryptionData{
                data,
            })
        )
    );
}
