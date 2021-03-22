use crate::{guid::*, span::Span, widestr::*};
use nom::{
    error::ParseError,
    multi::count,
    number::streaming::{le_u16, le_u32},
    IResult,
};
use std::{convert::TryInto, io::Write};
use uuid::Uuid;

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
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, presentation_time) = le_u32(input)?;
        let (input, type_index) = le_u16(input)?;
        let (input, command_name) = WideStr::parse_count16(input)?;
        Ok((
            input,
            Self {
                presentation_time,
                type_index,
                command_name,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        w.write_all(&self.presentation_time.to_le_bytes())?;
        w.write_all(&self.type_index.to_le_bytes())?;
        self.command_name.write_count16(w)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 4;
        len += 2;
        len += self.command_name.size_of_count16();
        len
    }
}

impl ScriptCommandData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, reserved) = guid(input)?;
        let (input, commands_count) = le_u16(input)?;
        let (input, command_types_count) = le_u16(input)?;
        let (input, command_types) =
            count(WideStr::parse_count16, command_types_count.into())(input)?;
        let (input, commands) = count(Command::parse, commands_count.into())(input)?;
        Ok((
            input,
            Self {
                reserved,
                command_types,
                commands,
            },
        ))
    }

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
        let mut len = 0;
        len += 16;
        len += 2;
        len += 2;
        len += 2;
        for command_type in self.command_types.iter() {
            len += command_type.size_of_count16();
        }
        for command in self.commands.iter() {
            len += command.size_of();
        }
        len
    }
}
