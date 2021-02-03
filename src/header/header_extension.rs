use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32};

use crate::{guid::*, object::*};

use super::{
    advanced_mutual_exclusion::*,
    bandwidth_sharing::*,
    extended_stream_properties::*,
    group_mutual_exclusion::*,
    language_list::*,
    stream_prioritization::*,
    metadata::{MetadataData, metadata_data},
    metadata_library::{MetadataLibraryData, metadata_library_data},
};

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
    Unknown(Object<'a>)
}

named!(pub extension_header_object<ExtensionHeaderObject>,
    switch!(object,
        Object{guid: EXTENDED_STREAM_PROPERTIES_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::ExtendedStreamProperties(extended_stream_properties_data(data)?.1))
        ) |
        Object{guid: ADVANCED_MUTUAL_EXCLUSION_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::AdvancedMutualExclusion(advanced_mutual_exclusion_data(data)?.1))
        ) |
        Object{guid: GROUP_MUTUAL_EXCLUSION_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::GroupMutualExclusion(group_mutual_exclusion_data(data)?.1))
        ) |
        Object{guid: STREAM_PRIORITIZATION_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::StreamPrioritization(stream_prioritization_data(data)?.1))
        ) |
        Object{guid: BANDWIDTH_SHARING_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::BandwidthSharing(bandwidth_sharing_data(data)?.1))
        ) |
        Object{guid: LANGUAGE_LIST_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::LanguageList(language_list_data(data)?.1))
        ) |
        Object{guid: METADATA_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::Metadata(metadata_data(data)?.1))
        ) |
        Object{guid: METADATA_LIBRARY_OBJECT, data} => do_parse!(
            (ExtensionHeaderObject::MetadataLibrary(metadata_library_data(data)?.1))
        ) |
        unknown => do_parse!((ExtensionHeaderObject::Unknown(unknown)))
    )
);

named!(extension_header_object_vec<Vec<ExtensionHeaderObject>>, many0!(complete!(extension_header_object)));

#[derive(Debug, PartialEq)]
pub struct HeaderExtensionData<'a> {
    reserved_1: Uuid,
    reserved_2: u16,
    extension_objects: Vec<ExtensionHeaderObject<'a>>,
}

named!(pub header_extension_data<HeaderExtensionData>,
    do_parse!(
        reserved_1: guid >>
        reserved_2: le_u16 >>
        extension_data_size: le_u32 >>
        extension_data: take!(extension_data_size) >>
        (HeaderExtensionData{
            reserved_1,
            reserved_2,
            extension_objects: extension_header_object_vec(extension_data)?.1,
        })
    )
);
