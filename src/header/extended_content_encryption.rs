use std::{convert::TryInto, io::Write};

use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct ExtendedContentEncryptionData<'a> {
    pub data: &'a [u8],
}

impl<'a> ExtendedContentEncryptionData<'a> {
    named!(pub parse<ExtendedContentEncryptionData>,
        do_parse!(
            data_size: le_u32 >>
            data: take!(data_size) >>
            (ExtendedContentEncryptionData{
                data,
            })
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let data_len: u32 = self.data.len().try_into()?;
        w.write_all(&data_len.to_le_bytes())?;
        w.write_all(self.data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + self.data.len()
    }
}
