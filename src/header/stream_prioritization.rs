use nom::number::streaming::{le_u16};


#[derive(Debug, PartialEq)]
pub struct PriorityRecord {
    pub stream_number: u16,
    pub priority_flags: u16,
}

#[derive(Debug, PartialEq)]
pub struct StreamPrioritizationData {
    pub priority_records: Vec<PriorityRecord>,
}

impl PriorityRecord {
    named!(pub parse<Self>,
        do_parse!(
            stream_number: le_u16 >>
            priority_flags: le_u16 >>
            (Self{
                stream_number,
                priority_flags,
            })
        )
    );
}

impl StreamPrioritizationData {
    named!(pub parse<Self>,
        do_parse!(
            priority_records: length_count!(le_u16, PriorityRecord::parse) >>
            (Self{priority_records})
        )
    );
}
