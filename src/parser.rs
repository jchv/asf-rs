use uuid::Uuid;
use nom::number::streaming::{le_u16, le_u32, le_u64, le_u8};

use crate::guid::*;


named!(pub guid<Uuid>,
    do_parse!(
        b: count!(le_u8, 16) >>
        (Uuid::from_bytes([
            b[3], b[2], b[1], b[0], b[5], b[4], b[7], b[6],
            b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15],
        ]))
    )
);

#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub guid: Uuid,
    pub size: u64,
}

named!(pub object_header<ObjectHeader>,
    do_parse!(guid: guid >> size: le_u64 >> (ObjectHeader{guid, size}))
);

#[derive(Debug, PartialEq)]
pub struct Object<'a> {
    pub guid: Uuid,
    pub data: &'a [u8],
}

named!(pub object<Object>,
    do_parse!(
        header: object_header >>
        data: take!(header.size - 24) >>
        (Object{guid: header.guid, data})
    )
);

named!(pub wchar_str<Vec<u16>>, terminated!(many0!(complete!(le_u16)), eof!()));

#[derive(Debug, PartialEq)]
pub struct StreamPropertiesData<'a> {
    stream_type: Uuid,
    error_correction_type: Uuid,
    time_offset: u64,
    flags: u16,
    reserved: u32,
    type_specific_data: &'a [u8],
    error_correction_data: &'a [u8],
}

named!(pub stream_properties_data<StreamPropertiesData>,
    do_parse!(
        stream_type: guid >>
        error_correction_type: guid >>
        time_offset: le_u64 >>
        type_specific_data_len: le_u32 >>
        error_correction_data_len: le_u32 >>
        flags: le_u16 >>
        reserved: le_u32 >>
        type_specific_data: take!(type_specific_data_len) >>
        error_correction_data: take!(error_correction_data_len) >>

        (StreamPropertiesData{
            stream_type,
            error_correction_type,
            time_offset,
            flags,
            reserved,
            type_specific_data,
            error_correction_data,
        })
    )
);

#[derive(Debug, PartialEq)]
pub struct HeaderExtensionData<'a> {
    reserved_1: Uuid,
    reserved_2: u16,
    extension_data: &'a [u8],
}

named!(pub header_extension_data<HeaderExtensionData>,
    do_parse!(
        reserved_1: guid >>
        reserved_2: le_u16 >>
        extension_data_size: le_u32 >>
        extension_data: take!(extension_data_size) >>
        (HeaderExtensionData{reserved_1, reserved_2, extension_data})
    )
);

#[derive(Debug, PartialEq)]
pub struct ContentDescriptorData {
    title: Vec<u16>,
    author: Vec<u16>,
    copyright: Vec<u16>,
    description: Vec<u16>,
    rating: Vec<u16>,
}

named!(pub content_descriptor_data<ContentDescriptorData>,
    do_parse!(
        title_len: le_u16 >>
        author_len: le_u16 >>
        copyright_len: le_u16 >>
        description_len: le_u16 >>
        rating_len: le_u16 >>

        title: take!(title_len) >>
        author: take!(author_len) >>
        copyright: take!(copyright_len) >>
        description: take!(description_len) >>
        rating: take!(rating_len) >>

        (ContentDescriptorData{
            title: wchar_str(title)?.1,
            author: wchar_str(author)?.1,
            copyright: wchar_str(copyright)?.1,
            description: wchar_str(description)?.1,
            rating: wchar_str(rating)?.1,
        })
    )
);

#[derive(Debug, PartialEq)]
pub enum HeaderObject<'a> {
    StreamProperties(StreamPropertiesData<'a>),
    ContentDescriptor(ContentDescriptorData),
    HeaderExtension(HeaderExtensionData<'a>),
    Unknown(Object<'a>)
}

named!(pub header_object<HeaderObject>,
    switch!(object,
        Object{guid: STREAM_PROPERTIES_OBJECT, data} => do_parse!(
            (HeaderObject::StreamProperties(stream_properties_data(data)?.1))
        ) |
        Object{guid: HEADER_EXTENSION_OBJECT, data} => do_parse!(
            (HeaderObject::HeaderExtension(header_extension_data(data)?.1))
        ) |
        Object{guid: CONTENT_DESCRIPTION_OBJECT, data} => do_parse!(
            (HeaderObject::ContentDescriptor(content_descriptor_data(data)?.1))
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

#[derive(Debug, PartialEq)]
pub struct DataObject {
}

named!(pub data_object<DataObject>,
    do_parse!(
        guid: tag!(DATA_OBJECT.as_bytes_ms()) >>
        size: le_u64 >>
        data: take!(size - 24) >>
        (DataObject{})
    )
);

#[derive(Debug, PartialEq)]
pub struct IndexObject {
}

named!(pub index_object<IndexObject>,
    do_parse!(
        header: object_header >>
        data: take!(header.size - 24) >>
        (IndexObject{})
    )
);

#[derive(Debug, PartialEq)]
pub struct IndexObjects {
    pub objects: Vec<IndexObject>
}

named!(pub index_objects<IndexObjects>,
    do_parse!(
        objects: many0!(complete!(index_object)) >>
        (IndexObjects{objects})
    )
);

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub header: HeaderObjects<'a>,
    pub data: DataObject,
    pub indices: IndexObjects,
}

named!(pub container<Container>,
    do_parse!(
        header: header_objects >>
        data: data_object >>
        indices: index_objects >>
        (Container{header, data, indices})
    )
);

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};

    use crate::guid::HEADER_OBJECT;

    use super::*;

    const BASIC_WMV: &'static [u8] = include_bytes!("../samples/basic.wmv");

    #[test]
    fn guids() {
        assert_eq!(
            guid(&[
                0x30, 0x26, 0xb2, 0x75, 0x8e, 0x66, 0xcf, 0x11,
                0xa6, 0xd9, 0x00, 0xaa, 0x00, 0x62, 0xce, 0x6c,
            ]),
            Ok((&b""[..], HEADER_OBJECT))
        );
    }

    #[test]
    fn basic_stream_properties() {
        assert_eq!(
            header_object(&[
                0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6,
                0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65, 0x72, 0x00, 0x00, 0x00, 
                0x00, 0x00, 0x00, 0x00, 0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 
                0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B, 
                0x50, 0xCD, 0xC3, 0xBF, 0x8F, 0x61, 0xCF, 0x11, 0x8B, 0xB2, 
                0x00, 0xAA, 0x00, 0xB4, 0xE2, 0x20, 0x00, 0x00, 0x00, 0x00, 
                0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x00, 0x08, 0x00, 
                0x00, 0x00, 0x01, 0x00, 0x70, 0x33, 0x77, 0x00, 0x61, 0x01, 
                0x01, 0x00, 0x80, 0x3E, 0x00, 0x00, 0xD0, 0x07, 0x00, 0x00, 
                0x80, 0x02, 0x10, 0x00, 0x0A, 0x00, 0x00, 0x22, 0x00, 0x00, 
                0x0E, 0x00, 0x80, 0x07, 0x00, 0x00, 0x01, 0x80, 0x02, 0x80,
                0x02, 0x01, 0x00, 0x00,
            ]),
            Ok((&b""[..], HeaderObject::StreamProperties(StreamPropertiesData{
                stream_type: AUDIO_MEDIA,
                error_correction_type: AUDIO_SPREAD,
                time_offset: 0,
                flags: 1,
                reserved: 7811952,
                type_specific_data: &[
                    0x61, 0x01, 0x01, 0x00, 0x80, 0x3E, 0x00, 0x00,
                    0xD0, 0x07, 0x00, 0x00, 0x80, 0x02, 0x10, 0x00,
                    0x0A, 0x00, 0x00, 0x22, 0x00, 0x00, 0x0E, 0x00,
                    0x80, 0x07, 0x00, 0x00,
                ],
                error_correction_data: &[
                    0x01, 0x80, 0x02, 0x80, 0x02, 0x01, 0x00, 0x00,
                ],
            }))),
        );
    }

    #[test]
    fn broken_content_descriptor() {
        let err = header_object(&[
            0x33, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9,
            0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C, 0x67, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x2E, 0x00, 0x11, 0x00, 0x02, 0x00,
            0x02, 0x00, 0x02, 0x00, 0x54, 0x00, 0x68, 0x00, 0x65, 0x00,
            0x20, 0x00, 0x4D, 0x00, 0x61, 0x00, 0x74, 0x00, 0x72, 0x00,
            0x69, 0x00, 0x78, 0x00, 0x20, 0x00, 0x50, 0x00, 0x61, 0x00,
            0x72, 0x00, 0x74, 0x00, 0x20, 0x00, 0x32, 0x00, 0x20, 0x00,
            0x6F, 0x00, 0x66, 0x00, 0x20, 0x00, 0x32, 0x00, 0x00, 0x00,
            0x63, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x66, 0x00, 0x75, 0x00,
            0x7A, 0x00, 0x65, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ]).expect_err("expected failure on broken header");
        match err {
            nom::Err::Error(Error{code: ErrorKind::Eof, ..}) => {}
            _ => panic!(format!("expected eof error, got {:?}", err))
        }
    }

    #[test]
    fn basic_content_descriptor() {
        assert_eq!(
            header_object(&[
                0x33, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9,
                0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C, 0x68, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x2E, 0x00, 0x12, 0x00, 0x02, 0x00,
                0x02, 0x00, 0x02, 0x00, 0x54, 0x00, 0x68, 0x00, 0x65, 0x00,
                0x20, 0x00, 0x4D, 0x00, 0x61, 0x00, 0x74, 0x00, 0x72, 0x00,
                0x69, 0x00, 0x78, 0x00, 0x20, 0x00, 0x50, 0x00, 0x61, 0x00,
                0x72, 0x00, 0x74, 0x00, 0x20, 0x00, 0x32, 0x00, 0x20, 0x00,
                0x6F, 0x00, 0x66, 0x00, 0x20, 0x00, 0x32, 0x00, 0x00, 0x00,
                0x63, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x66, 0x00, 0x75, 0x00,
                0x7A, 0x00, 0x65, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]),
            Ok((&b""[..],
                HeaderObject::ContentDescriptor(ContentDescriptorData{
                    title: "The Matrix Part 2 of 2\0".encode_utf16().collect(),
                    author: "confuzed\0".encode_utf16().collect(),
                    copyright: vec![0],
                    description: vec![0],
                    rating: vec![0],
                })
            ))
        )
    }

    #[test]
    fn basic_wmv() {
        assert_eq!(
            container(&BASIC_WMV),
            Ok((&b""[..], Container{
                header: HeaderObjects{
                    objects: Vec::new(),
                },
                data: DataObject{
                },
                indices: IndexObjects{
                    objects: vec![IndexObject{}],
                },
            }))
        );
    }
}
