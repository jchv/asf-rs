use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::{guid::*, widestr::*};


#[derive(Debug, PartialEq)]
pub struct CodecEntry<'a> {
    pub codec_type: u16,
    pub codec_name: WideStr,
    pub codec_description: WideStr,
    pub codec_information: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct CodecListData<'a> {
    pub reserved: Uuid,
    pub codec_entries: Vec<CodecEntry<'a>>,
}

impl<'a> CodecEntry<'a> {
    named!(pub parse<CodecEntry>,
        do_parse!(
            codec_type: le_u16 >>
            codec_name: len16_prefixed_widestr >>
            codec_description: len16_prefixed_widestr >>
            codec_information_len: le_u16 >>
            codec_information: take!(codec_information_len) >>
            (CodecEntry{codec_type, codec_name, codec_description, codec_information})
        )
    );
}

impl<'a> CodecListData<'a> {
    named!(pub parse<CodecListData>,
        do_parse!(
            reserved: guid >>
            codec_entries: length_count!(le_u32, CodecEntry::parse) >>
            (CodecListData{reserved, codec_entries})
        )
    );
}
