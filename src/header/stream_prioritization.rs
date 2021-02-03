use nom::number::streaming::{le_u16};


#[derive(Debug, PartialEq)]
pub struct PriorityRecord {
    stream_number: u16,
    priority_flags: u16,
}

named!(pub priority_record<PriorityRecord>,
    do_parse!(
        stream_number: le_u16 >>
        priority_flags: le_u16 >>
        (PriorityRecord{
            stream_number,
            priority_flags,
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct StreamPrioritizationData {
    priority_records: Vec<PriorityRecord>,
}

named!(pub stream_prioritization_data<StreamPrioritizationData>,
    do_parse!(
        priority_records: length_count!(le_u16, priority_record) >>
        (StreamPrioritizationData{priority_records})
    )
);
