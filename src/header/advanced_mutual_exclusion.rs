use uuid::Uuid;
use nom::number::streaming::le_u16;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct AdvancedMutualExclusionData {
    exclusion_type: Uuid,
    stream_numbers: Vec<u16>,
}

named!(pub advanced_mutual_exclusion_data<AdvancedMutualExclusionData>,
    do_parse!(
        exclusion_type: guid >>
        stream_numbers: length_count!(le_u16, le_u16) >>
        (AdvancedMutualExclusionData{exclusion_type, stream_numbers})
    )
);
