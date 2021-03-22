extern crate nom;
extern crate nom_locate;
extern crate uuid;

pub mod data;
pub mod error;
pub mod guid;
pub mod header;
pub mod index;
pub mod object;
pub mod span;
pub mod widestr;

pub(crate) mod combinators;

use nom::{Err, IResult};

use crate::{
    data::DataObject, error::Error, header::HeaderObjects, index::IndexObjects, span::Span,
};

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub header: HeaderObjects<'a>,
    pub data: DataObject<'a>,
    pub indices: IndexObjects,
}

impl<'a> Container<'a> {
    pub(crate) fn parse(input: Span<'a>) -> IResult<Span<'a>, Container, Error<Span<'a>>> {
        let (input, header) = HeaderObjects::parse(input)?;
        let (input, data) = DataObject::parse(input)?;
        let (input, indices) = IndexObjects::parse(input)?;
        Ok((
            input,
            Container {
                header,
                data,
                indices,
            },
        ))
    }
}

pub fn parse<'a>(data: &'a [u8]) -> Result<Container, Err<Error<Span<'a>>>> {
    Ok(Container::parse(Span::new(data))?.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_WMV: &'static [u8] = include_bytes!("../samples/basic.wmv");

    #[test]
    fn basic_wmv() {
        let (remaining, _data) =
            Container::parse(Span::new(BASIC_WMV)).expect("to parse successfully");
        assert_eq!(remaining.len(), 0);
    }
}
