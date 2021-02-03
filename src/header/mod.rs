pub mod bitrate_mutual_exclusion;
pub mod codec_list;
pub mod content_description;
pub mod error_correction;
pub mod extended_content_description;
pub mod file_properties;
pub mod header_extension;
pub mod marker;
pub mod script_command;
pub mod stream_bitrate_properties;
pub mod stream_properties;
use nom::number::streaming::{le_u32, le_u64, le_u8};

use crate::{guid::*, object::*};

use self::{
    bitrate_mutual_exclusion::*,
    codec_list::*,
    content_description::*,
    error_correction::*,
    extended_content_description::*,
    file_properties::*,
    header_extension::*,
    marker::*,
    script_command::*,
    stream_bitrate_properties::*,
    stream_properties::*
};


#[derive(Debug, PartialEq)]
pub enum HeaderObject<'a> {
    FileProperties(FilePropertiesData),
    StreamProperties(StreamPropertiesData<'a>),
    HeaderExtension(HeaderExtensionData<'a>),
    CodecList(CodecListData<'a>),
    ScriptCommand(ScriptCommandData),
    Marker(MarkerData),
    BitrateMutualExclusion(BitrateMutualExclusionData),
    ErrorCorrection(ErrorCorrectionData<'a>),
    ContentDescription(ContentDescriptionData),
    ExtendedContentDescription(ExtendedContentDescriptionData<'a>),
    StreamBitrateProperties(StreamBitratePropertiesData),
    Unknown(Object<'a>)
}

named!(pub header_object<HeaderObject>,
    switch!(object,
        Object{guid: FILE_PROPERTIES_OBJECT, data} => do_parse!(
            (HeaderObject::FileProperties(file_properties_data(data)?.1))
        ) |
        Object{guid: STREAM_PROPERTIES_OBJECT, data} => do_parse!(
            (HeaderObject::StreamProperties(stream_properties_data(data)?.1))
        ) |
        Object{guid: HEADER_EXTENSION_OBJECT, data} => do_parse!(
            (HeaderObject::HeaderExtension(header_extension_data(data)?.1))
        ) |
        Object{guid: CODEC_LIST_OBJECT, data} => do_parse!(
            (HeaderObject::CodecList(codec_list_data(data)?.1))
        ) |
        Object{guid: SCRIPT_COMMAND_OBJECT, data} => do_parse!(
            (HeaderObject::ScriptCommand(script_command_data(data)?.1))
        ) |
        Object{guid: MARKER_OBJECT, data} => do_parse!(
            (HeaderObject::Marker(marker_data(data)?.1))
        ) |
        Object{guid: BITRATE_MUTUAL_EXCLUSION_OBJECT, data} => do_parse!(
            (HeaderObject::BitrateMutualExclusion(bitrate_mutual_exclusion_data(data)?.1))
        ) |
        Object{guid: ERROR_CORRECTION_OBJECT, data} => do_parse!(
            (HeaderObject::ErrorCorrection(error_correction_data(data)?.1))
        ) |
        Object{guid: CONTENT_DESCRIPTION_OBJECT, data} => do_parse!(
            (HeaderObject::ContentDescription(content_description_data(data)?.1))
        ) |
        Object{guid: EXTENDED_CONTENT_DESCRIPTION_OBJECT, data} => do_parse!(
            (HeaderObject::ExtendedContentDescription(extended_content_description_data(data)?.1))
        ) |
        Object{guid: STREAM_BITRATE_PROPERTIES_OBJECT, data} => do_parse!(
            (HeaderObject::StreamBitrateProperties(stream_bitrate_properties_data(data)?.1))
        ) |
        unknown => do_parse!((HeaderObject::Unknown(unknown)))
    )
);

named!(header_object_vec<Vec<HeaderObject>>, many0!(complete!(header_object)));

#[derive(Debug, PartialEq)]
pub struct HeaderObjects<'a> {
    pub objects: Vec<HeaderObject<'a>>
}

named!(pub header_objects<HeaderObjects>,
    do_parse!(
        guid: tag!(HEADER_OBJECT.as_bytes_ms()) >>
        size: le_u64 >>
        num_header_objs: le_u32 >>
        reserved1: le_u8 >>
        reserved2: le_u8 >>
        data: take!(size - 30) >>
        (HeaderObjects{objects: header_object_vec(data)?.1})
    )
);