use nom::number::streaming::{le_u16, le_u32};


#[derive(Debug, PartialEq)]
pub struct IndexSpecifier {
    pub stream_number: u16,
    pub index_type: u16,
}

#[derive(Debug, PartialEq)]
pub struct IndexParametersData {
    pub index_entry_time_interval: u32,
    pub index_specifiers: Vec<IndexSpecifier>,
}

impl IndexSpecifier {
    named!(pub parse<Self>,
        do_parse!(
            stream_number: le_u16 >>
            index_type: le_u16 >>
            (Self{
                stream_number,
                index_type,
            })
        )
    );
}

impl IndexParametersData {
    named!(pub parse<Self>,
        do_parse!(
            index_entry_time_interval: le_u32 >>
            index_specifiers: length_count!(le_u16, IndexSpecifier::parse) >>
            (Self{
                index_entry_time_interval,
                index_specifiers,
            })
        )
    );
}
