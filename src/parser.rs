use nom::IResult;

use crate::{data::DataObject, error::Error, header::HeaderObjects, index::IndexObjects};

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub header: HeaderObjects<'a>,
    pub data: DataObject<'a>,
    pub indices: IndexObjects,
}

impl<'a> Container<'a> {
    pub fn parse(input: &'a[u8]) -> IResult<&'a[u8], Container, Error<&'a[u8]>> {
        let (input, header) = HeaderObjects::parse(input)?;
        let (input, data) = DataObject::parse(input)?;
        let (input, indices) = IndexObjects::parse(input)?;
        Ok((input, Container{header, data, indices}))
    }
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
        match Container::parse(&BASIC_WMV) {
            Ok((_remainder, container)) => {
                println!("{:?}", container);
            },
            Err(nom::Err::Error(e)) => {
                println!("{:?}", e);
            }
            _ => {
                panic!("unexpected error");
            }
        }
        
    }
}
