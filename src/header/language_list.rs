use nom::number::streaming::{le_u8, le_u16};

use crate::widestr::*;


#[derive(Debug, PartialEq)]
pub struct LanguageListData {
    pub language_id_records: Vec<WideStr>,
}

impl LanguageListData {
    named!(parse_id<WideStr>,
        do_parse!(
            length: le_u8 >>
            data: take!(length) >>
            (wchar_str(data)?.1)
        )
    );

    named!(pub parse<Self>,
        do_parse!(
            language_id_records: length_count!(le_u16, Self::parse_id) >>
            (Self{
                language_id_records,
            })
        )
    );
}
