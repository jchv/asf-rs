use std::{convert::TryInto, io::Write};

use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct DigitalSignatureData<'a> {
    pub signature_type: u32,
    pub signature_data: &'a [u8],
}

impl<'a> DigitalSignatureData<'a> {
    named!(pub parse<DigitalSignatureData>,
        do_parse!(
            signature_type: le_u32 >>
            signature_data_size: le_u32 >>
            signature_data: take!(signature_data_size) >>
            (DigitalSignatureData{
                signature_type,
                signature_data,
            })
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let signature_data_len: u32 = self.signature_data.len().try_into()?;
        w.write_all(&self.signature_type.to_le_bytes())?;
        w.write_all(&signature_data_len.to_le_bytes())?;
        w.write_all(self.signature_data)?;

        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + 4 + self.signature_data.len()
    }
}
