use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::{guid::*, widestr::*};


#[derive(Debug, PartialEq)]
pub struct CodecEntry<'a> {
    codec_type: u16,
    codec_name: WideStr,
    codec_description: WideStr,
    codec_information: &'a [u8],
}

named!(pub codec_entry<CodecEntry>,
    do_parse!(
        codec_type: le_u16 >>
        codec_name: len16_prefixed_widestr >>
        codec_description: len16_prefixed_widestr >>
        codec_information_len: le_u16 >>
        codec_information: take!(codec_information_len) >>
        (CodecEntry{codec_type, codec_name, codec_description, codec_information})
    )
);

#[derive(Debug, PartialEq)]
pub struct CodecListData<'a> {
    reserved: Uuid,
    codec_entries: Vec<CodecEntry<'a>>,
}

named!(pub codec_list_data<CodecListData>,
    do_parse!(
        reserved: guid >>
        codec_entries: length_count!(le_u32, codec_entry) >>
        (CodecListData{reserved, codec_entries})
    )
);
