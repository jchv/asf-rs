use nom::number::streaming::le_u64;

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
    use super::*;

    const BASIC_WMV: &'static [u8] = include_bytes!("../samples/basic.wmv");

    #[test]
    fn basic_wmv() {
        let (remaining, _data) = Container::parse(&BASIC_WMV).expect("to parse successfully");
        assert_eq!(remaining.len(), 0);
    }
}
