use nom::number::streaming::{le_u16, le_u32};


#[derive(Debug, PartialEq)]
pub struct BitrateRecord {
    pub flags: u16,
    pub average_bitrate: u32,
}

#[derive(Debug, PartialEq)]
pub struct StreamBitratePropertiesData {
    pub bitrate_records: Vec<BitrateRecord>
}

impl BitrateRecord {
    named!(pub parse<Self>,
        do_parse!(
            flags: le_u16 >>
            average_bitrate: le_u32 >>
            (Self{flags, average_bitrate})
        )
    );
}

impl StreamBitratePropertiesData {
    named!(pub parse<Self>,
        do_parse!(
            bitrate_records: length_count!(le_u16, BitrateRecord::parse) >>
            (Self{bitrate_records})
        )
    );
}
