use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::number::streaming::le_u32;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct ErrorCorrectionData<'a> {
    pub error_correction_type: Uuid,
    pub error_correction_data: &'a [u8],
}

impl<'a> ErrorCorrectionData<'a> {
    named!(pub parse<ErrorCorrectionData>,
        do_parse!(
            error_correction_type: guid >>
            error_correction_data_length: le_u32 >>
            error_correction_data: take!(error_correction_data_length) >>
            (ErrorCorrectionData{error_correction_type, error_correction_data})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let error_correction_data_len: u32 = self.error_correction_data.len().try_into()?;
        w.write_all(&self.error_correction_type.as_bytes_ms())?;
        w.write_all(&error_correction_data_len.to_le_bytes())?;
        w.write_all(self.error_correction_data)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 4 + self.error_correction_data.len()
    }
}
