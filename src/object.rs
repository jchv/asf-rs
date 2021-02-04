
use nom::number::streaming::le_u64;
use uuid::Uuid;

use crate::guid::*;


#[derive(Debug, PartialEq)]
pub struct ObjectHeader {
    pub guid: Uuid,
    pub size: u64,
}

#[derive(Debug, PartialEq)]
pub struct Object<'a> {
    pub guid: Uuid,
    pub data: &'a [u8],
}

named!(pub object_header<ObjectHeader>,
    do_parse!(guid: guid >> size: le_u64 >> (ObjectHeader{guid, size}))
);

named!(pub object<Object>,
    do_parse!(
        header: object_header >>
        data: take!(header.size - 24) >>
        (Object{guid: header.guid, data})
    )
);
