pub mod advanced_content_encryption;
pub mod advanced_mutual_exclusion;
pub mod bandwidth_sharing;
pub mod bitrate_mutual_exclusion;
pub mod codec_list;
pub mod compatibility;
pub mod content_branding;
pub mod content_description;
pub mod content_encryption;
pub mod digital_signature;
pub mod error_correction;
pub mod extended_content_description;
pub mod extended_content_encryption;
pub mod extended_stream_properties;
pub mod file_properties;
pub mod group_mutual_exclusion;
pub mod header_extension;
pub mod index_parameters;
pub mod language_list;
pub mod marker;
pub mod media_object_index_parameters;
pub mod metadata;
pub mod metadata_library;
pub mod script_command;
pub mod stream_bitrate_properties;
pub mod stream_prioritization;
pub mod stream_properties;
pub mod timecode_index_parameters;
use nom::number::streaming::{le_u32, le_u64, le_u8};

use crate::{guid::*, object::*};

use self::{
    bitrate_mutual_exclusion::*,
    codec_list::*,
    content_description::*,
    content_branding::*,
    content_encryption::*,
    digital_signature::*,
    error_correction::*,
    extended_content_description::*,
    extended_content_encryption::*,
    file_properties::*,
    header_extension::*,
    marker::*,
    script_command::*,
    stream_bitrate_properties::*,
    stream_properties::*,
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
    ContentBranding(ContentBrandingData<'a>),
    ContentEncryption(ContentEncryptionData<'a>),
    ExtendedContentEncryption(ExtendedContentEncryptionData<'a>),
    DigitalSignature(DigitalSignatureData<'a>),
    Padding(usize),
    Unknown(Object<'a>)
}

impl<'a> HeaderObject<'a> {
    named!(pub parse<HeaderObject>,
        switch!(object,
            Object{guid: FILE_PROPERTIES_OBJECT, data} => do_parse!(
                (HeaderObject::FileProperties(FilePropertiesData::parse(data)?.1))
            ) |
            Object{guid: STREAM_PROPERTIES_OBJECT, data} => do_parse!(
                (HeaderObject::StreamProperties(StreamPropertiesData::parse(data)?.1))
            ) |
            Object{guid: HEADER_EXTENSION_OBJECT, data} => do_parse!(
                (HeaderObject::HeaderExtension(HeaderExtensionData::parse(data)?.1))
            ) |
            Object{guid: CODEC_LIST_OBJECT, data} => do_parse!(
                (HeaderObject::CodecList(CodecListData::parse(data)?.1))
            ) |
            Object{guid: SCRIPT_COMMAND_OBJECT, data} => do_parse!(
                (HeaderObject::ScriptCommand(ScriptCommandData::parse(data)?.1))
            ) |
            Object{guid: MARKER_OBJECT, data} => do_parse!(
                (HeaderObject::Marker(MarkerData::parse(data)?.1))
            ) |
            Object{guid: BITRATE_MUTUAL_EXCLUSION_OBJECT, data} => do_parse!(
                (HeaderObject::BitrateMutualExclusion(BitrateMutualExclusionData::parse(data)?.1))
            ) |
            Object{guid: ERROR_CORRECTION_OBJECT, data} => do_parse!(
                (HeaderObject::ErrorCorrection(ErrorCorrectionData::parse(data)?.1))
            ) |
            Object{guid: CONTENT_DESCRIPTION_OBJECT, data} => do_parse!(
                (HeaderObject::ContentDescription(ContentDescriptionData::parse(data)?.1))
            ) |
            Object{guid: EXTENDED_CONTENT_DESCRIPTION_OBJECT, data} => do_parse!(
                (HeaderObject::ExtendedContentDescription(ExtendedContentDescriptionData::parse(data)?.1))
            ) |
            Object{guid: STREAM_BITRATE_PROPERTIES_OBJECT, data} => do_parse!(
                (HeaderObject::StreamBitrateProperties(StreamBitratePropertiesData::parse(data)?.1))
            ) |
            Object{guid: CONTENT_BRANDING_OBJECT, data} => do_parse!(
                (HeaderObject::ContentBranding(ContentBrandingData::parse(data)?.1))
            ) |
            Object{guid: CONTENT_ENCRYPTION_OBJECT, data} => do_parse!(
                (HeaderObject::ContentEncryption(ContentEncryptionData::parse(data)?.1))
            ) |
            Object{guid: EXTENDED_CONTENT_ENCRYPTION_OBJECT, data} => do_parse!(
                (HeaderObject::ExtendedContentEncryption(ExtendedContentEncryptionData::parse(data)?.1))
            ) |
            Object{guid: DIGITAL_SIGNATURE_OBJECT, data} => do_parse!(
                (HeaderObject::DigitalSignature(DigitalSignatureData::parse(data)?.1))
            ) |
            Object{guid: PADDING_OBJECT, data} => do_parse!(
                (HeaderObject::Padding(data.len()))
            ) |
            unknown => do_parse!((HeaderObject::Unknown(unknown)))
        )
    );

    named!(parse_many<Vec<HeaderObject>>, many0!(complete!(Self::parse)));
}

#[derive(Debug, PartialEq)]
pub struct HeaderObjects<'a> {
    pub objects: Vec<HeaderObject<'a>>
}

impl<'a> HeaderObjects<'a> {
    named!(pub parse<HeaderObjects>,
        do_parse!(
            guid: tag!(HEADER_OBJECT.as_bytes_ms()) >>
            size: le_u64 >>
            num_header_objs: le_u32 >>
            reserved1: le_u8 >>
            reserved2: le_u8 >>
            data: take!(size - 30) >>
            (HeaderObjects{objects: HeaderObject::parse_many(data)?.1})
        )
    );
}
