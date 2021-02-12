use std::{convert::TryInto, io::Write};

use nom::number::streaming::{le_u16, le_u32};


#[derive(Debug, PartialEq)]
pub struct IndexSpecifier {
    pub stream_number: u16,
    pub index_type: u16,
}

#[derive(Debug, PartialEq)]
pub struct TimecodeIndexParametersData {
    pub index_entry_time_interval: u32,
    pub index_specifiers: Vec<IndexSpecifier>,
}

impl IndexSpecifier {
    named!(pub parse<IndexSpecifier>,
        do_parse!(
            stream_number: le_u16 >>
            index_type: le_u16 >>
            (IndexSpecifier{
                stream_number,
                index_type,
            })
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.stream_number.to_le_bytes())?;
        w.write_all(&self.index_type.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 2
    }
}

impl TimecodeIndexParametersData {
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

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let index_specifiers_len: u16 = self.index_specifiers.len().try_into()?;
        w.write_all(&self.index_entry_time_interval.to_le_bytes())?;
        w.write_all(&index_specifiers_len.to_le_bytes())?;
        for index_specifier in self.index_specifiers.iter() {
            index_specifier.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + 2 + self.index_specifiers.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
