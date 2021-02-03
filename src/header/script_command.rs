use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::{guid::*, widestr::*};

#[derive(Debug, PartialEq)]
pub struct Command {
    presentation_time: u32,
    type_index: u16,
    command_name: WideStr,
}

named!(pub command<Command>,
    do_parse!(
        presentation_time: le_u32 >>
        type_index: le_u16 >>
        command_name: len16_prefixed_widestr >>
        (Command{presentation_time, type_index, command_name})
    )
);

#[derive(Debug, PartialEq)]
pub struct ScriptCommandData {
    reserved: Uuid,
    command_types: Vec<WideStr>,
    commands: Vec<Command>,
}

named!(pub script_command_data<ScriptCommandData>,
    do_parse!(
        reserved: guid >>
        commands_count: le_u16 >>
        command_types_count: le_u16 >>
        command_types: count!(len16_prefixed_widestr, command_types_count.into()) >>
        commands: count!(command, commands_count.into()) >>
        (ScriptCommandData{reserved, command_types, commands})
    )
);