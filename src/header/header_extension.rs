use std::{convert::TryInto, io::Write};

use uuid::Uuid;
use nom::{IResult, bytes::streaming::take, combinator::complete, error::ParseError, multi::many0, number::streaming::{le_u16, le_u32}};

use crate::{guid::*, object::*, span::Span};

use super::advanced_content_encryption::AdvancedContentEncryptionData;
use super::advanced_mutual_exclusion::AdvancedMutualExclusionData;
use super::bandwidth_sharing::BandwidthSharingData;
use super::extended_stream_properties::ExtendedStreamPropertiesData;
use super::group_mutual_exclusion::GroupMutualExclusionData;
use super::language_list::LanguageListData;
use super::stream_prioritization::StreamPrioritizationData;
use super::metadata::MetadataData;
use super::metadata_library::MetadataLibraryData;
use super::index_parameters::IndexParametersData;
use super::media_object_index_parameters::MediaObjectIndexParametersData;
use super::timecode_index_parameters::TimecodeIndexParametersData;
use super::compatibility::CompatibilityData;

#[derive(Debug, PartialEq)]
pub enum ExtensionHeaderObject<'a> {
    ExtendedStreamProperties(ExtendedStreamPropertiesData<'a>),
    AdvancedMutualExclusion(AdvancedMutualExclusionData),
    GroupMutualExclusion(GroupMutualExclusionData),
    StreamPrioritization(StreamPrioritizationData),
    BandwidthSharing(BandwidthSharingData),
    LanguageList(LanguageListData),
    Metadata(MetadataData<'a>),
    MetadataLibrary(MetadataLibraryData<'a>),
    IndexParameters(IndexParametersData),
    MediaObjectIndexParameters(MediaObjectIndexParametersData),
    TimecodeIndexParameters(TimecodeIndexParametersData),
    Compatibility(CompatibilityData),
    AdvancedContentEncryption(AdvancedContentEncryptionData<'a>),
    Unknown(Object<'a>)
}

impl<'a> ExtensionHeaderObject<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, obj) = object(input)?;
        Ok((input, match obj {
            Object{guid: EXTENDED_STREAM_PROPERTIES_OBJECT, data} =>
                (Self::ExtendedStreamProperties(ExtendedStreamPropertiesData::parse(data)?.1)),
            Object{guid: ADVANCED_MUTUAL_EXCLUSION_OBJECT, data} =>
                (Self::AdvancedMutualExclusion(AdvancedMutualExclusionData::parse(data)?.1)),
            Object{guid: GROUP_MUTUAL_EXCLUSION_OBJECT, data} =>
                (Self::GroupMutualExclusion(GroupMutualExclusionData::parse(data)?.1)),
            Object{guid: STREAM_PRIORITIZATION_OBJECT, data} =>
                (Self::StreamPrioritization(StreamPrioritizationData::parse(data)?.1)),
            Object{guid: BANDWIDTH_SHARING_OBJECT, data} =>
                (Self::BandwidthSharing(BandwidthSharingData::parse(data)?.1)),
            Object{guid: LANGUAGE_LIST_OBJECT, data} =>
                (Self::LanguageList(LanguageListData::parse(data)?.1)),
            Object{guid: METADATA_OBJECT, data} =>
                (Self::Metadata(MetadataData::parse(data)?.1)),
            Object{guid: METADATA_LIBRARY_OBJECT, data} =>
                (Self::MetadataLibrary(MetadataLibraryData::parse(data)?.1)),
            Object{guid: INDEX_PARAMETERS_OBJECT, data} =>
                (Self::IndexParameters(IndexParametersData::parse(data)?.1)),
            Object{guid: MEDIA_OBJECT_INDEX_PARAMETERS_OBJECT, data} =>
                (Self::MediaObjectIndexParameters(MediaObjectIndexParametersData::parse(data)?.1)),
            Object{guid: TIMECODE_INDEX_PARAMETERS_OBJECT, data} =>
                (Self::TimecodeIndexParameters(TimecodeIndexParametersData::parse(data)?.1)),
            Object{guid: COMPATIBILITY_OBJECT, data} =>
                (Self::Compatibility(CompatibilityData::parse(data)?.1)),
            Object{guid: ADVANCED_CONTENT_ENCRYPTION_OBJECT, data} =>
                (Self::AdvancedContentEncryption(AdvancedContentEncryptionData::parse(data)?.1)),
            unknown => Self::Unknown(unknown),
        }))
    }

    pub fn parse_many<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Vec<Self>, E> {
        many0(complete(Self::parse))(input)
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            ExtensionHeaderObject::ExtendedStreamProperties(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&EXTENDED_STREAM_PROPERTIES_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::AdvancedMutualExclusion(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&ADVANCED_MUTUAL_EXCLUSION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::GroupMutualExclusion(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&GROUP_MUTUAL_EXCLUSION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::StreamPrioritization(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&STREAM_PRIORITIZATION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::BandwidthSharing(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&BANDWIDTH_SHARING_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::LanguageList(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&LANGUAGE_LIST_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::Metadata(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&METADATA_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::MetadataLibrary(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&METADATA_LIBRARY_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::IndexParameters(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&INDEX_PARAMETERS_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::MediaObjectIndexParameters(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&MEDIA_OBJECT_INDEX_PARAMETERS_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::TimecodeIndexParameters(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&TIMECODE_INDEX_PARAMETERS_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::Compatibility(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&COMPATIBILITY_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::AdvancedContentEncryption(data) => {
                let data_len: u64 = data.size_of().try_into()?;
                w.write_all(&ADVANCED_CONTENT_ENCRYPTION_OBJECT.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                data.write(w)?;
            },
            ExtensionHeaderObject::Unknown(unk) => {
                let data_len: u64 = unk.data.len().try_into()?;
                w.write_all(&unk.guid.as_bytes_ms())?;
                w.write_all(&data_len.to_le_bytes())?;
                w.write_all(&unk.data)?;
            }
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 8 +
        match self {
            ExtensionHeaderObject::ExtendedStreamProperties(data) => data.size_of(),
            ExtensionHeaderObject::AdvancedMutualExclusion(data) => data.size_of(),
            ExtensionHeaderObject::GroupMutualExclusion(data) => data.size_of(),
            ExtensionHeaderObject::StreamPrioritization(data) => data.size_of(),
            ExtensionHeaderObject::BandwidthSharing(data) => data.size_of(),
            ExtensionHeaderObject::LanguageList(data) => data.size_of(),
            ExtensionHeaderObject::Metadata(data) => data.size_of(),
            ExtensionHeaderObject::MetadataLibrary(data) => data.size_of(),
            ExtensionHeaderObject::IndexParameters(data) => data.size_of(),
            ExtensionHeaderObject::MediaObjectIndexParameters(data) => data.size_of(),
            ExtensionHeaderObject::TimecodeIndexParameters(data) => data.size_of(),
            ExtensionHeaderObject::Compatibility(data) => data.size_of(),
            ExtensionHeaderObject::AdvancedContentEncryption(data) => data.size_of(),
            ExtensionHeaderObject::Unknown(unk) => unk.data.len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HeaderExtensionData<'a> {
    reserved_1: Uuid,
    reserved_2: u16,
    extension_objects: Vec<ExtensionHeaderObject<'a>>,
}

impl<'a> HeaderExtensionData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, reserved_1) = guid(input)?;
        let (input, reserved_2) = le_u16(input)?;
        let (input, extension_data_size) = le_u32(input)?;
        let (input, extension_data) = take(extension_data_size)(input)?;
        Ok((input, HeaderExtensionData{
            reserved_1,
            reserved_2,
            extension_objects: ExtensionHeaderObject::parse_many(extension_data)?.1,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let extension_data_size: u32 = self.extension_objects.iter().map(|x| x.size_of()).sum::<usize>().try_into()?;
        w.write_all(&self.reserved_1.as_bytes_ms())?;
        w.write_all(&self.reserved_2.to_le_bytes())?;
        w.write_all(&extension_data_size.to_le_bytes())?;
        for extension_object in self.extension_objects.iter() {
            extension_object.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        16 + 2 + 4 + self.extension_objects.iter().map(|x| x.size_of()).sum::<usize>()
    }
}
