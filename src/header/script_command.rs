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
            command_name: len16_prefixed_widestr >>
            (Self{presentation_time, type_index, command_name})
        )
    );
}

impl ScriptCommandData {
    named!(pub parse<Self>,
        do_parse!(
            reserved: guid >>
            commands_count: le_u16 >>
            command_types_count: le_u16 >>
            command_types: count!(len16_prefixed_widestr, command_types_count.into()) >>
            commands: count!(Command::parse, commands_count.into()) >>
            (Self{reserved, command_types, commands})
        )
    );
}
