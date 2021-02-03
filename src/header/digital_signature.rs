use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct DigitalSignatureData<'a> {
    signature_type: u32,
    signature_data: &'a [u8],
}

named!(pub digital_signature_data<DigitalSignatureData>,
    do_parse!(
        signature_type: le_u32 >>
        signature_data_size: le_u32 >>
        signature_data: take!(signature_data_size) >>
        (DigitalSignatureData{
            signature_type,
            signature_data,
        })
    )
);
