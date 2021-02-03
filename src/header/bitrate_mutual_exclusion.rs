use uuid::Uuid;
use nom::number::streaming::{le_u16};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct BitrateMutualExclusionData {
    exclusion_type: Uuid,
    stream_numbers: Vec<u16>,
}

named!(pub bitrate_mutual_exclusion_data<BitrateMutualExclusionData>,
    do_parse!(
        exclusion_type: guid >>
        stream_numbers: length_count!(le_u16, le_u16) >>
        (BitrateMutualExclusionData{exclusion_type, stream_numbers})
    )
);
