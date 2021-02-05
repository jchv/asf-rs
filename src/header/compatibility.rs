use std::io::Write;

use nom::number::streaming::le_u8;


#[derive(Debug, PartialEq)]
pub struct CompatibilityData {
    pub profile: u8,
    pub mode: u8,
}

impl CompatibilityData {
    named!(pub parse<Self>,
        do_parse!(
            profile: le_u8 >>
            mode: le_u8 >>
            (Self{
                profile,
                mode,
            })
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.profile.to_le_bytes())?;
        w.write_all(&self.mode.to_le_bytes())?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + 2
    }
}
