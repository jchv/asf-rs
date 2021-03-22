use crate::{span::Span, widestr::*};
use nom::{
    bytes::streaming::take,
    error::ParseError,
    multi::length_count,
    number::streaming::{le_u16, le_u8},
    IResult,
};
use std::{convert::TryInto, io::Write};

#[derive(Debug, PartialEq)]
pub struct LanguageListData {
    pub language_id_records: Vec<WideStr>,
}

impl LanguageListData {
    fn parse_id<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, WideStr, E> {
        let (input, length) = le_u8(input)?;
        let (input, data) = take(length)(input)?;
        Ok((input, WideStr::parse(data)?.1))
    }

    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, language_id_records) = length_count(le_u16, Self::parse_id)(input)?;
        Ok((
            input,
            Self {
                language_id_records,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let language_id_records_len: u16 = self.language_id_records.len().try_into()?;
        w.write_all(&language_id_records_len.to_le_bytes())?;
        for language_id_record in self.language_id_records.iter() {
            let language_id_record_len: u8 = language_id_record.size_of().try_into()?;
            w.write_all(&language_id_record_len.to_le_bytes())?;
            language_id_record.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 2;
        for language_id_record in self.language_id_records.iter() {
            len += 1;
            len += language_id_record.size_of()
        }
        len
    }
}
