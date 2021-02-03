use uuid::Uuid;
use nom::number::streaming::{le_u16};

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct GroupMutualExclusionData {
    exclusion_type: Uuid,
    records: Vec<Vec<u16>>,
}

named!(pub group_mutual_exclusion_data<GroupMutualExclusionData>,
    do_parse!(
        exclusion_type: guid >>
        records: length_count!(le_u16, length_count!(le_u16, le_u16)) >>
        (GroupMutualExclusionData{exclusion_type, records})
    )
);
