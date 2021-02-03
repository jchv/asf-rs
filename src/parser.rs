use nom::number::streaming::{le_u64};

use crate::{guid::*, header::*, object::*};

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
