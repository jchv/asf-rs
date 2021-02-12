use std::{convert::TryInto, io::Write};

use nom::number::streaming::le_u16;

use crate::widestr::*;


#[derive(Debug, PartialEq)]
pub struct ContentDescriptor<'a> {
    pub name: WideStr,
    pub value_type: u16,
    pub value: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct ExtendedContentDescriptionData<'a> {
    pub descriptors: Vec<ContentDescriptor<'a>>,
}

impl<'a> ContentDescriptor<'a> {
    named!(pub parse<ContentDescriptor>,
        do_parse!(
            name: call!(WideStr::parse_count16) >>
            value_type: le_u16 >>
            value_len: le_u16 >>
            value: take!(value_len) >>
            (ContentDescriptor{name, value_type, value})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let value_len: u16 = self.value.len().try_into()?;
        self.name.write_count16(w)?;
        w.write_all(&self.value_type.to_le_bytes())?;
        w.write_all(&value_len.to_le_bytes())?;
        w.write_all(self.value)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        self.name.size_of_count16() + 2 + 2 + self.value.len()
    }
}

impl<'a> ExtendedContentDescriptionData<'a> {
    named!(pub parse<ExtendedContentDescriptionData>,
        do_parse!(
            descriptors: length_count!(le_u16, ContentDescriptor::parse) >>
            (ExtendedContentDescriptionData{descriptors})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let descriptors_len: u32 = self.descriptors.len().try_into()?;
        w.write_all(&descriptors_len.to_le_bytes())?;
        for descriptor in self.descriptors.iter() {
            descriptor.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        2 + self.descriptors.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
