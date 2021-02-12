use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::{guid::*, widestr::*};


#[derive(Debug, PartialEq)]
pub struct Command {
    pub presentation_time: u32,
    pub type_index: u16,
    pub command_name: WideStr,
}

#[derive(Debug, PartialEq)]
pub struct ScriptCommandData {
    pub reserved: Uuid,
    pub command_types: Vec<WideStr>,
    pub commands: Vec<Command>,
}

impl Command {
    named!(pub parse<Self>,
        do_parse!(
            presentation_time: le_u32 >>
            type_index: le_u16 >>
            command_name: call!(WideStr::parse_count16) >>
            (Self{presentation_time, type_index, command_name})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.presentation_time.to_le_bytes())?;
        w.write_all(&self.type_index.to_le_bytes())?;
        self.command_name.write_count16(w)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + 2 + self.command_name.size_of_count16()
    }
}

impl ScriptCommandData {
    named!(pub parse<Self>,
        do_parse!(
            reserved: guid >>
            commands_count: le_u16 >>
            command_types_count: le_u16 >>
            command_types: count!(WideStr::parse_count16, command_types_count.into()) >>
            commands: count!(Command::parse, commands_count.into()) >>
            (Self{reserved, command_types, commands})
        )
    );

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let command_types_len: u16 = self.command_types.len().try_into()?;
        let commands_len: u16 = self.commands.len().try_into()?;
        w.write_all(&self.reserved.as_bytes_ms())?;
        w.write_all(&commands_len.to_le_bytes())?;
        w.write_all(&command_types_len.to_le_bytes())?;
        for command_type in self.command_types.iter() {
            command_type.write_count16(w)?;
        }
        for command in self.commands.iter() {
            command.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 2 + 2 + 2 +
        self.command_types.iter().map(|x| x.size_of()).sum::<usize>() +
        self.commands.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
