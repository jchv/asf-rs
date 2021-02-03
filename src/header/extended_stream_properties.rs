use nom::number::streaming::{le_u16, le_u32, le_u64};
use uuid::Uuid;

use crate::{guid::*, object::*, widestr::*};

use super::stream_properties::*;

#[derive(Debug, PartialEq)]
pub struct StreamName {
    language_id_index: u16,
    stream_name: WideStr,
}

named!(pub stream_name<StreamName>,
    do_parse!(
        language_id_index: le_u16 >>
        stream_name_length: le_u16 >>
        stream_name: take!(stream_name_length) >>
        (StreamName{
            language_id_index,
            stream_name: wchar_str(stream_name)?.1,
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct PayloadExtensionSystem<'a> {
    id: Uuid,
    data_size: u16,
    info: &'a [u8],
}

named!(pub payload_extension_system<PayloadExtensionSystem>,
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

#[derive(Debug, PartialEq)]
pub struct ExtendedStreamPropertiesData<'a> {
    start_time: u64,
    end_time: u64,
    data_bitrate: u32,
    buffer_size: u32,
    initial_buffer_fullness: u32,
    alternate_data_bitrate: u32,
    alternate_buffer_size: u32,
    alternate_initial_buffer_fullness: u32,
    maximum_object_size: u32,
    flags: u32,
    stream_number: u16,
    stream_language_id_index: u16,
    average_time_per_frame: u64,
    stream_names: Vec<StreamName>,
    payload_extension_systems: Vec<PayloadExtensionSystem<'a>>,
    stream_properties_object: Option<StreamPropertiesData<'a>>,
}

named!(pub extended_stream_properties_data<ExtendedStreamPropertiesData>,
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
        stream_names: count!(stream_name, stream_name_count.into()) >>
        payload_extension_systems: count!(payload_extension_system, payload_extension_system_count.into()) >>
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
                .map(|x| stream_properties_data(x.data))
                .map_or(Ok(None), |r| r.map(Some))?
                .map(|x| x.1),
        })
    )
);
