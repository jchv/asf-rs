use uuid::Uuid;
use nom::number::streaming::le_u32;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct ErrorCorrectionData<'a> {
    error_correction_type: Uuid,
    error_correction_data: &'a [u8],
}

named!(pub error_correction_data<ErrorCorrectionData>,
    do_parse!(
        error_correction_type: guid >>
        error_correction_data_length: le_u32 >>
        error_correction_data: take!(error_correction_data_length) >>
        (ErrorCorrectionData{error_correction_type, error_correction_data})
    )
);
