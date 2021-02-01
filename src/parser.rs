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
    ContentDescriptor(ContentDescriptorData),
    Unknown(Object<'a>)
}

named!(pub header_object<HeaderObject>,
    switch!(object,
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
