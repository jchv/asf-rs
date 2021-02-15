use crate::{data::DataObject, header::HeaderObjects, index::IndexObjects};

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub header: HeaderObjects<'a>,
    pub data: DataObject<'a>,
    pub indices: IndexObjects,
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

    #[test]
    fn basic_wmv_print() {
        println!("Dump: {:?}", Container::parse(&BASIC_WMV).expect("to parse successfully").1);
    }
}
