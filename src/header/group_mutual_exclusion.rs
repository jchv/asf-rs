use uuid::Uuid;
use nom::number::streaming::{le_u16};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct GroupMutualExclusionData {
    pub exclusion_type: Uuid,
    pub records: Vec<Vec<u16>>,
}

impl GroupMutualExclusionData {
    named!(pub parse<Self>,
        do_parse!(
            exclusion_type: guid >>
            records: length_count!(le_u16, length_count!(le_u16, le_u16)) >>
            (Self{exclusion_type, records})
        )
    );
}
