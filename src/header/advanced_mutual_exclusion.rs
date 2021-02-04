use uuid::Uuid;
use nom::number::streaming::le_u16;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct AdvancedMutualExclusionData {
    pub exclusion_type: Uuid,
    pub stream_numbers: Vec<u16>,
}

impl AdvancedMutualExclusionData {
    named!(pub parse<Self>,
        do_parse!(
            exclusion_type: guid >>
            stream_numbers: length_count!(le_u16, le_u16) >>
            (Self{exclusion_type, stream_numbers})
        )
    );
}
