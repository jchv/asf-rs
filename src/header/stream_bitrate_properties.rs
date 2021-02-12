use std::{convert::TryInto, io::Write};

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

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.flags.to_le_bytes())?;
        w.write_all(&self.average_bitrate.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 4
    }
}

impl StreamBitratePropertiesData {
    named!(pub parse<Self>,
        do_parse!(
            bitrate_records: length_count!(le_u16, BitrateRecord::parse) >>
            (Self{bitrate_records})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let bitrate_records_len: u16 = self.bitrate_records.len().try_into()?;
        w.write_all(&bitrate_records_len.to_le_bytes())?;
        for bitrate_record in self.bitrate_records.iter() {
            bitrate_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.bitrate_records.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
