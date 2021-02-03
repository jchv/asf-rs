use nom::number::streaming::le_u16;

use crate::widestr::*;


#[derive(Debug, PartialEq)]
pub struct ContentDescriptor<'a> {
    name: WideStr,
    value_type: u16,
    value: &'a [u8],
}

named!(pub content_descriptor<ContentDescriptor>,
    do_parse!(
        name: len16_prefixed_widestr >>
        value_type: le_u16 >>
        value_len: le_u16 >>
        value: take!(value_len) >>
        (ContentDescriptor{name, value_type, value})
    )
);

#[derive(Debug, PartialEq)]
pub struct ExtendedContentDescriptionData<'a> {
    descriptors: Vec<ContentDescriptor<'a>>,
}

named!(pub extended_content_description_data<ExtendedContentDescriptionData>,
    do_parse!(
        descriptors: length_count!(le_u16, content_descriptor) >>
        (ExtendedContentDescriptionData{descriptors})
    )
);
