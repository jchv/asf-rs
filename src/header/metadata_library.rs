use nom::number::streaming::{le_u16, le_u32};

use crate::widestr::*;


#[derive(Debug, PartialEq)]
pub struct DescriptionRecord<'a> {
    language_list_index: u16,
    stream_number: u16,
    data_type: u16,
    name: WideStr,
    data: &'a [u8],
}

named!(pub description_record<DescriptionRecord>,
    do_parse!(
        language_list_index: le_u16 >>
        stream_number: le_u16 >>
        name_length: le_u16 >>
        data_type: le_u16 >>
        data_length: le_u32 >>
        name: take!(name_length) >>
        data: take!(data_length) >>
        (DescriptionRecord{
            language_list_index,
            stream_number,
            data_type,
            name: wchar_str(name)?.1,
            data,
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct MetadataLibraryData<'a> {
    description_records: Vec<DescriptionRecord<'a>>,
}

named!(pub metadata_library_data<MetadataLibraryData>,
    do_parse!(
        description_records: length_count!(le_u16, description_record) >>
        (MetadataLibraryData{description_records})
    )
);
