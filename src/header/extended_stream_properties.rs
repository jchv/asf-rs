use nom::number::streaming::{le_u16, le_u32, le_u64};
use uuid::Uuid;

use crate::{guid::*, object::*, widestr::*};

use super::stream_properties::*;

#[derive(Debug, PartialEq)]
pub struct StreamName {
    pub language_id_index: u16,
    pub stream_name: WideStr,
}

#[derive(Debug, PartialEq)]
pub struct PayloadExtensionSystem<'a> {
    pub id: Uuid,
    pub data_size: u16,
    pub info: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct ExtendedStreamPropertiesData<'a> {
    pub start_time: u64,
    pub end_time: u64,
    pub data_bitrate: u32,
    pub buffer_size: u32,
    pub initial_buffer_fullness: u32,
    pub alternate_data_bitrate: u32,
    pub alternate_buffer_size: u32,
    pub alternate_initial_buffer_fullness: u32,
    pub maximum_object_size: u32,
    pub flags: u32,
    pub stream_number: u16,
    pub stream_language_id_index: u16,
    pub average_time_per_frame: u64,
    pub stream_names: Vec<StreamName>,
    pub payload_extension_systems: Vec<PayloadExtensionSystem<'a>>,
    pub stream_properties_object: Option<StreamPropertiesData<'a>>,
}

impl StreamName {
    named!(pub parse<Self>,
        do_parse!(
            language_id_index: le_u16 >>
            stream_name_length: le_u16 >>
            stream_name: take!(stream_name_length) >>
            (Self{
                language_id_index,
                stream_name: wchar_str(stream_name)?.1,
            })
        )
    );
}

impl<'a> PayloadExtensionSystem<'a> {
    named!(pub parse<PayloadExtensionSystem>,
        do_parse!(
            id: guid >>
            data_size: le_u16 >>
            info_length: le_u32 >>
            info: take!(info_length) >>
            (PayloadExtensionSystem{
                id,
                data_size,
                info,
            })
        )
    );
}

impl<'a> ExtendedStreamPropertiesData<'a> {
    named!(pub parse<ExtendedStreamPropertiesData>,
        do_parse!(
            start_time: le_u64 >>
            end_time: le_u64 >>
            data_bitrate: le_u32 >>
            buffer_size: le_u32 >>
            initial_buffer_fullness: le_u32 >>
            alternate_data_bitrate: le_u32 >>
            alternate_buffer_size: le_u32 >>
            alternate_initial_buffer_fullness: le_u32 >>
            maximum_object_size: le_u32 >>
            flags: le_u32 >>
            stream_number: le_u16 >>
            stream_language_id_index: le_u16 >>
            average_time_per_frame: le_u64 >>
            stream_name_count: le_u16 >>
            payload_extension_system_count: le_u16 >>
            stream_names: count!(StreamName::parse, stream_name_count.into()) >>
            payload_extension_systems: count!(PayloadExtensionSystem::parse, payload_extension_system_count.into()) >>
            stream_properties_object: opt!(object) >>
            (ExtendedStreamPropertiesData{
                start_time,
                end_time,
                data_bitrate,
                buffer_size,
                initial_buffer_fullness,
                alternate_data_bitrate,
                alternate_buffer_size,
                alternate_initial_buffer_fullness,
                maximum_object_size,
                flags,
                stream_number,
                stream_language_id_index,
                average_time_per_frame,
                stream_names,
                payload_extension_systems,
                stream_properties_object: stream_properties_object
                    .map(|x| StreamPropertiesData::parse(x.data))
                    .map_or(Ok(None), |r| r.map(Some))?
                    .map(|x| x.1),
            })
        )
    );
}
