use nom::{ErrorConvert, IResult, combinator::complete, error::ParseError};

use crate::{data::DataObject, header::HeaderObjects, index::IndexObjects};

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub header: HeaderObjects<'a>,
    pub data: DataObject<'a>,
    pub indices: IndexObjects,
}

impl<'a> Container<'a> {
    pub fn parse<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Container, E2> {
        let (input, header) = complete(HeaderObjects::parse::<E2>)(input)?;
        let (input, data) = complete(DataObject::parse::<E1, E2>)(input)?;
        let (input, indices) = complete(IndexObjects::parse::<E2>)(input)?;
        Ok((input, Container{header, data, indices}))
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    const BASIC_WMV: &'static [u8] = include_bytes!("../samples/basic.wmv");

    #[test]
    fn basic_wmv() {
        let (remaining, _data) = Container::parse::<VerboseError<_>, VerboseError<_>>(&BASIC_WMV).expect("to parse successfully");
        assert_eq!(remaining.len(), 0);
    }

    #[test]
    fn basic_wmv_print() {
        println!("Dump: {:?}", Container::parse::<VerboseError<_>, VerboseError<_>>(&BASIC_WMV).expect("to parse successfully").1);
    }
}
