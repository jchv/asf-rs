use std::{convert::TryInto, io::Write};

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
            (WideStr::parse(data)?.1)
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

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let language_id_records_len: u16 = self.language_id_records.len().try_into()?;
        w.write_all(&language_id_records_len.to_le_bytes())?;
        for language_id_record in self.language_id_records.iter() {
            let language_id_record_len: u8 = language_id_record.size_of().try_into()?;
            w.write_all(&language_id_record_len.to_le_bytes())?;
            language_id_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.language_id_records.iter().map(|x| 1 + x.size_of()).sum::<usize>()
    }
}
