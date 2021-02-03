use nom::number::streaming::{le_u16, le_u32};


#[derive(Debug, PartialEq)]
pub struct IndexSpecifier {
    stream_number: u16,
    index_type: u16,
}

named!(pub index_specifier<IndexSpecifier>,
    do_parse!(
        stream_number: le_u16 >>
        index_type: le_u16 >>
        (IndexSpecifier{
            stream_number,
            index_type,
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct IndexParametersData {
    index_entry_time_interval: u32,
    index_specifiers: Vec<IndexSpecifier>,
}

named!(pub index_parameters_data<IndexParametersData>,
    do_parse!(
        index_entry_time_interval: le_u32 >>
        index_specifiers: length_count!(le_u16, index_specifier) >>
        (IndexParametersData{
            index_entry_time_interval,
            index_specifiers,
        })
    )
);
