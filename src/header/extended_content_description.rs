use nom::number::streaming::le_u16;

use crate::widestr::*;


#[derive(Debug, PartialEq)]
pub struct ContentDescriptor<'a> {
    pub name: WideStr,
    pub value_type: u16,
    pub value: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct ExtendedContentDescriptionData<'a> {
    pub descriptors: Vec<ContentDescriptor<'a>>,
}

impl<'a> ContentDescriptor<'a> {
    named!(pub parse<ContentDescriptor>,
        do_parse!(
            name: len16_prefixed_widestr >>
            value_type: le_u16 >>
            value_len: le_u16 >>
            value: take!(value_len) >>
            (ContentDescriptor{name, value_type, value})
        )
    );
}

impl<'a> ExtendedContentDescriptionData<'a> {
    named!(pub parse<ExtendedContentDescriptionData>,
        do_parse!(
            descriptors: length_count!(le_u16, ContentDescriptor::parse) >>
            (ExtendedContentDescriptionData{descriptors})
        )
    );
}
