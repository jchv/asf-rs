use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::{guid::*, object::*};

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
    named!(pub parse<ExtensionHeaderObject>,
        switch!(object,
            Object{guid: EXTENDED_STREAM_PROPERTIES_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::ExtendedStreamProperties(ExtendedStreamPropertiesData::parse(data)?.1))
            ) |
            Object{guid: ADVANCED_MUTUAL_EXCLUSION_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::AdvancedMutualExclusion(AdvancedMutualExclusionData::parse(data)?.1))
            ) |
            Object{guid: GROUP_MUTUAL_EXCLUSION_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::GroupMutualExclusion(GroupMutualExclusionData::parse(data)?.1))
            ) |
            Object{guid: STREAM_PRIORITIZATION_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::StreamPrioritization(StreamPrioritizationData::parse(data)?.1))
            ) |
            Object{guid: BANDWIDTH_SHARING_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::BandwidthSharing(BandwidthSharingData::parse(data)?.1))
            ) |
            Object{guid: LANGUAGE_LIST_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::LanguageList(LanguageListData::parse(data)?.1))
            ) |
            Object{guid: METADATA_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::Metadata(MetadataData::parse(data)?.1))
            ) |
            Object{guid: METADATA_LIBRARY_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::MetadataLibrary(MetadataLibraryData::parse(data)?.1))
            ) |
            Object{guid: INDEX_PARAMETERS_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::IndexParameters(IndexParametersData::parse(data)?.1))
            ) |
            Object{guid: MEDIA_OBJECT_INDEX_PARAMETERS_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::MediaObjectIndexParameters(MediaObjectIndexParametersData::parse(data)?.1))
            ) |
            Object{guid: TIMECODE_INDEX_PARAMETERS_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::TimecodeIndexParameters(TimecodeIndexParametersData::parse(data)?.1))
            ) |
            Object{guid: COMPATIBILITY_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::Compatibility(CompatibilityData::parse(data)?.1))
            ) |
            Object{guid: ADVANCED_CONTENT_ENCRYPTION_OBJECT, data} => do_parse!(
                (ExtensionHeaderObject::AdvancedContentEncryption(AdvancedContentEncryptionData::parse(data)?.1))
            ) |
            unknown => do_parse!((ExtensionHeaderObject::Unknown(unknown)))
        )
    );

    named!(parse_many<Vec<ExtensionHeaderObject>>, many0!(complete!(Self::parse)));
}

#[derive(Debug, PartialEq)]
pub struct HeaderExtensionData<'a> {
    reserved_1: Uuid,
    reserved_2: u16,
    extension_objects: Vec<ExtensionHeaderObject<'a>>,
}

impl<'a> HeaderExtensionData<'a> {
    named!(pub parse<HeaderExtensionData>,
        do_parse!(
            reserved_1: guid >>
            reserved_2: le_u16 >>
            extension_data_size: le_u32 >>
            extension_data: take!(extension_data_size) >>
            (HeaderExtensionData{
                reserved_1,
                reserved_2,
                extension_objects: ExtensionHeaderObject::parse_many(extension_data)?.1,
            })
        )
    );
}
