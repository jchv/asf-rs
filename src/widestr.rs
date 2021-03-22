use std::{convert::TryInto, fmt, io::Write};

use nom::{
    combinator::{complete, eof, map},
    error::ParseError,
    multi::{length_count, many0},
    number::streaming::{le_u16, le_u32},
    sequence::terminated,
    IResult,
};

use crate::span::Span;

#[derive(PartialEq)]
pub struct WideStr(Vec<u16>);

impl WideStr {
    pub fn new(s: &str) -> Self {
        let w: Vec<u16> = s.encode_utf16().collect();
        WideStr(w)
    }

    pub fn to_str(&self) -> String {
        String::from_utf16_lossy(&self.0)
    }

    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        map(terminated(many0(complete(le_u16)), eof), |x| WideStr(x))(input)
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        for word in self.0.iter() {
            w.write_all(&word.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn size_of(&self) -> usize {
        self.0.len() * 2
    }

    pub fn parse_count16<'a, E: ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, Self, E> {
        map(length_count(le_u16, le_u16), |x| WideStr(x))(input)
    }

    pub fn write_count16<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let count: u16 = self.0.len().try_into()?;
        w.write_all(&count.to_le_bytes())?;
        for word in self.0.iter() {
            w.write_all(&word.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn size_of_count16(&self) -> usize {
        2 + self.0.len() * 2
    }

    pub fn parse_count32<'a, E: ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, Self, E> {
        map(length_count(le_u32, le_u16), |x| WideStr(x))(input)
    }

    pub fn write_count32<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let count: u32 = self.0.len().try_into()?;
        w.write_all(&count.to_le_bytes())?;
        for word in self.0.iter() {
            w.write_all(&word.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn size_of_count32(&self) -> usize {
        4 + self.0.len() * 2
    }
}

impl From<Vec<u16>> for WideStr {
    fn from(data: Vec<u16>) -> Self {
        Self(data)
    }
}

impl fmt::Debug for WideStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_str())
    }
}

impl fmt::Display for WideStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_str())
    }
}
