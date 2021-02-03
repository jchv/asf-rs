use nom::number::streaming::{le_u16, le_u32};


#[derive(Debug, PartialEq)]
pub struct BitrateRecord {
    flags: u16,
    average_bitrate: u32,
}

named!(pub bitrate_record<BitrateRecord>,
    do_parse!(
        flags: le_u16 >>
        average_bitrate: le_u32 >>
        (BitrateRecord{flags, average_bitrate})
    )
);

#[derive(Debug, PartialEq)]
pub struct StreamBitratePropertiesData {
    bitrate_records: Vec<BitrateRecord>
}

named!(pub stream_bitrate_properties_data<StreamBitratePropertiesData>,
    do_parse!(
        bitrate_records: length_count!(le_u16, bitrate_record) >>
        (StreamBitratePropertiesData{bitrate_records})
    )
);
