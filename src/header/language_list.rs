use nom::number::streaming::{le_u8, le_u16};

use crate::widestr::*;


named!(pub language_id_record<WideStr>,
    do_parse!(
        length: le_u8 >>
        data: take!(length) >>
        (wchar_str(data)?.1)
    )
);

#[derive(Debug, PartialEq)]
pub struct LanguageListData {
    language_id_records: Vec<WideStr>,
}

named!(pub language_list_data<LanguageListData>,
    do_parse!(
        language_id_records: length_count!(le_u16, language_id_record) >>
        (LanguageListData{
            language_id_records,
        })
    )
);
