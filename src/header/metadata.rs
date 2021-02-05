use nom::number::streaming::{le_u16, le_u32};

use crate::widestr::*;


#[derive(Debug, PartialEq)]
pub struct DescriptionRecord<'a> {
    pub reserved: u16,
    pub stream_number: u16,
    pub data_type: u16,
    pub name: WideStr,
    pub data: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct MetadataData<'a> {
    pub description_records: Vec<DescriptionRecord<'a>>,
}

impl<'a> DescriptionRecord<'a> {
    named!(pub parse<DescriptionRecord>,
        do_parse!(
            reserved: le_u16 >>
            stream_number: le_u16 >>
            name_length: le_u16 >>
            data_type: le_u16 >>
            data_length: le_u32 >>
            name: take!(name_length) >>
            data: take!(data_length) >>
            (DescriptionRecord{
                reserved,
                stream_number,
                data_type,
                name: WideStr::parse(name)?.1,
                data,
            })
        )
    );
}

impl<'a> MetadataData<'a> {
    named!(pub parse<MetadataData>,
        do_parse!(
            description_records: length_count!(le_u16, DescriptionRecord::parse) >>
            (MetadataData{
                description_records,
            })
        )
    );
}