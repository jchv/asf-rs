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
use std::{convert::TryInto, io::Write};

use nom::{IResult, bytes::streaming::{tag, take}, combinator::complete, error::ParseError, multi::many0, number::streaming::{le_u32, le_u64, le_u8}};

use crate::{guid::*, object::*};

use self::bitrate_mutual_exclusion::BitrateMutualExclusionData;
use self::codec_list::CodecListData;
use self::content_branding::ContentBrandingData;
use self::content_description::ContentDescriptionData;
use self::content_encryption::ContentEncryptionData;
use self::digital_signature::DigitalSignatureData;
use self::error_correction::ErrorCorrectionData;
use self::extended_content_description::ExtendedContentDescriptionData;
use self::extended_content_encryption::ExtendedContentEncryptionData;
use self::file_properties::FilePropertiesData;
use self::header_extension::HeaderExtensionData;
use self::marker::MarkerData;
use self::script_command::ScriptCommandData;
use self::stream_bitrate_properties::StreamBitratePropertiesData;
use self::stream_properties::StreamPropertiesData;

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
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, obj) = object(input)?;
        Ok((input, match obj {
            Object{guid: FILE_PROPERTIES_OBJECT, data} =>
                (Self::FileProperties(FilePropertiesData::parse(data)?.1)),
            Object{guid: STREAM_PROPERTIES_OBJECT, data} =>
                (Self::StreamProperties(StreamPropertiesData::parse(data)?.1)),
            Object{guid: HEADER_EXTENSION_OBJECT, data} =>
                (Self::HeaderExtension(HeaderExtensionData::parse(data)?.1)),
            Object{guid: CODEC_LIST_OBJECT, data} =>
                (Self::CodecList(CodecListData::parse(data)?.1)),
            Object{guid: SCRIPT_COMMAND_OBJECT, data} =>
                (Self::ScriptCommand(ScriptCommandData::parse(data)?.1)),
            Object{guid: MARKER_OBJECT, data} =>
                (Self::Marker(MarkerData::parse(data)?.1)),
            Object{guid: BITRATE_MUTUAL_EXCLUSION_OBJECT, data} =>
                (Self::BitrateMutualExclusion(BitrateMutualExclusionData::parse(data)?.1)),
            Object{guid: ERROR_CORRECTION_OBJECT, data} =>
                (Self::ErrorCorrection(ErrorCorrectionData::parse(data)?.1)),
            Object{guid: CONTENT_DESCRIPTION_OBJECT, data} =>
                (Self::ContentDescription(ContentDescriptionData::parse(data)?.1)),
            Object{guid: EXTENDED_CONTENT_DESCRIPTION_OBJECT, data} =>
                (Self::ExtendedContentDescription(ExtendedContentDescriptionData::parse(data)?.1)),
            Object{guid: STREAM_BITRATE_PROPERTIES_OBJECT, data} =>
                (Self::StreamBitrateProperties(StreamBitratePropertiesData::parse(data)?.1)),
            Object{guid: CONTENT_BRANDING_OBJECT, data} =>
                (Self::ContentBranding(ContentBrandingData::parse(data)?.1)),
            Object{guid: CONTENT_ENCRYPTION_OBJECT, data} =>
                (Self::ContentEncryption(ContentEncryptionData::parse(data)?.1)),
            Object{guid: EXTENDED_CONTENT_ENCRYPTION_OBJECT, data} =>
                (Self::ExtendedContentEncryption(ExtendedContentEncryptionData::parse(data)?.1)),
            Object{guid: DIGITAL_SIGNATURE_OBJECT, data} =>
                (Self::DigitalSignature(DigitalSignatureData::parse(data)?.1)),
            Object{guid: PADDING_OBJECT, data} =>
                (Self::Padding(data.len())),
            unknown => Self::Unknown(unknown)
        }))
    }

    pub fn parse_many<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Vec<Self>, E> {
        many0(complete(Self::parse))(input)
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let data_len: u64 = self.size_of().try_into()?;
        match self {
            HeaderObject::FileProperties(data) => {
                w.write_all(&FILE_PROPERTIES_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::StreamProperties(data) => {
                w.write_all(&STREAM_PROPERTIES_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::HeaderExtension(data) => {
                w.write_all(&HEADER_EXTENSION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::CodecList(data) => {
                w.write_all(&CODEC_LIST_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ScriptCommand(data) => {
                w.write_all(&SCRIPT_COMMAND_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::Marker(data) => {
                w.write_all(&MARKER_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::BitrateMutualExclusion(data) => {
                w.write_all(&BITRATE_MUTUAL_EXCLUSION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ErrorCorrection(data) => {
                w.write_all(&ERROR_CORRECTION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ContentDescription(data) => {
                w.write_all(&CONTENT_DESCRIPTION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ExtendedContentDescription(data) => {
                w.write_all(&EXTENDED_CONTENT_DESCRIPTION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::StreamBitrateProperties(data) => {
                w.write_all(&STREAM_BITRATE_PROPERTIES_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ContentBranding(data) => {
                w.write_all(&CONTENT_BRANDING_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ContentEncryption(data) => {
                w.write_all(&CONTENT_ENCRYPTION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::ExtendedContentEncryption(data) => {
                w.write_all(&EXTENDED_CONTENT_ENCRYPTION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::DigitalSignature(data) => {
                w.write_all(&DIGITAL_SIGNATURE_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            }
            HeaderObject::Padding(size) => {
                w.write_all(&PADDING_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                w.write_all(&vec![0u8; *size])?;
            }
            HeaderObject::Unknown(unk) => {
                w.write_all(&unk.guid.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                w.write_all(unk.data)?;
            }
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 8 +
        match self {
            HeaderObject::FileProperties(data) => data.size_of(),
            HeaderObject::StreamProperties(data) => data.size_of(),
            HeaderObject::HeaderExtension(data) => data.size_of(),
            HeaderObject::CodecList(data) => data.size_of(),
            HeaderObject::ScriptCommand(data) => data.size_of(),
            HeaderObject::Marker(data) => data.size_of(),
            HeaderObject::BitrateMutualExclusion(data) => data.size_of(),
            HeaderObject::ErrorCorrection(data) => data.size_of(),
            HeaderObject::ContentDescription(data) => data.size_of(),
            HeaderObject::ExtendedContentDescription(data) => data.size_of(),
            HeaderObject::StreamBitrateProperties(data) => data.size_of(),
            HeaderObject::ContentBranding(data) => data.size_of(),
            HeaderObject::ContentEncryption(data) => data.size_of(),
            HeaderObject::ExtendedContentEncryption(data) => data.size_of(),
            HeaderObject::DigitalSignature(data) => data.size_of(),
            HeaderObject::Padding(size) => *size,
            HeaderObject::Unknown(unk) => unk.data.len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HeaderObjects<'a> {
    pub reserved1: u8,
    pub reserved2: u8,
    pub objects: Vec<HeaderObject<'a>>
}

impl<'a> HeaderObjects<'a> {
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, _guid) = tag(HEADER_OBJECT.as_bytes_ms())(input)?;
        let (input, size) = le_u64(input)?;
        let (input, _num_header_objs) = le_u32(input)?;
        let (input, reserved1) = le_u8(input)?;
        let (input, reserved2) = le_u8(input)?;
        let (input, data) = take(size - 30)(input)?;
        Ok((input, Self{reserved1, reserved2, objects: HeaderObject::parse_many(data)?.1}))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let size: u64 = self.size_of().try_into()?;
        let num_header_objs: u32 = self.objects.len().try_into()?;
        w.write_all(&HEADER_OBJECT.as_bytes_ms())?;
        w.write_all(&size.to_le_bytes())?;
        w.write_all(&num_header_objs.to_le_bytes())?;
        w.write_all(&self.reserved1.to_le_bytes())?;
        w.write_all(&self.reserved2.to_le_bytes())?;
        for object in self.objects.iter() {
            object.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 8 + 4 + 1 + 1 + self.objects.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
