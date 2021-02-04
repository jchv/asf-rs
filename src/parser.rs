use nom::number::streaming::{le_u64};

use crate::{guid::*, header::*, object::*};

#[derive(Debug, PartialEq)]
pub struct DataObject {
}

#[derive(Debug, PartialEq)]
pub struct IndexObject {
}

#[derive(Debug, PartialEq)]
pub struct IndexObjects {
    pub objects: Vec<IndexObject>
}

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub header: HeaderObjects<'a>,
    pub data: DataObject,
    pub indices: IndexObjects,
}

impl DataObject {
    named!(pub parse<DataObject>,
        do_parse!(
            guid: tag!(DATA_OBJECT.as_bytes_ms()) >>
            size: le_u64 >>
            data: take!(size - 24) >>
            (DataObject{})
        )
    );
}

impl IndexObject {
    named!(pub parse<IndexObject>,
        do_parse!(
            header: object_header >>
            data: take!(header.size - 24) >>
            (IndexObject{})
        )
    );
}

impl IndexObjects {
    named!(pub parse<IndexObjects>,
        do_parse!(
            objects: many0!(complete!(IndexObject::parse)) >>
            (IndexObjects{objects})
        )
    );
}

impl<'a> Container<'a> {
    named!(pub parse<Container>,
        do_parse!(
            header: complete!(HeaderObjects::parse) >>
            data: complete!(DataObject::parse) >>
            indices: complete!(IndexObjects::parse) >>
            (Container{header, data, indices})
        )
    );
}

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
            Container::parse(&BASIC_WMV),
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
