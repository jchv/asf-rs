use std::{convert::TryInto, io::Write};

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

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let name_len: u16 = self.name.size_of().try_into()?;
        let data_len: u16 = self.data.len().try_into()?;
        w.write_all(&self.reserved.to_le_bytes())?;
        w.write_all(&self.stream_number.to_le_bytes())?;
        w.write_all(&name_len.to_le_bytes())?;
        w.write_all(&self.data_type.to_le_bytes())?;
        w.write_all(&data_len.to_le_bytes())?;
        self.name.write(w)?;
        w.write_all(self.data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 2 + 2 + 2 + 4 + self.name.size_of() + self.data.len()
    }
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

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let description_records_len: u16 = self.description_records.len().try_into()?;
        w.write_all(&description_records_len.to_le_bytes())?;
        for description_record in self.description_records.iter() {
            description_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.description_records.iter().map(|x| x.size_of()).sum::<usize>()
    }
}