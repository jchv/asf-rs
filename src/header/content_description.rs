use crate::{span::Span, widestr::*};
use nom::{bytes::streaming::take, error::ParseError, number::streaming::le_u16, IResult};
use std::{convert::TryInto, io::Write};

#[derive(Debug, PartialEq)]
pub struct ContentDescriptionData {
    pub title: WideStr,
    pub author: WideStr,
    pub copyright: WideStr,
    pub description: WideStr,
    pub rating: WideStr,
}

impl ContentDescriptionData {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, title_len) = le_u16(input)?;
        let (input, author_len) = le_u16(input)?;
        let (input, copyright_len) = le_u16(input)?;
        let (input, description_len) = le_u16(input)?;
        let (input, rating_len) = le_u16(input)?;

        let (input, title) = take(title_len)(input)?;
        let (input, author) = take(author_len)(input)?;
        let (input, copyright) = take(copyright_len)(input)?;
        let (input, description) = take(description_len)(input)?;
        let (input, rating) = take(rating_len)(input)?;

        Ok((
            input,
            Self {
                title: WideStr::parse(title)?.1,
                author: WideStr::parse(author)?.1,
                copyright: WideStr::parse(copyright)?.1,
                description: WideStr::parse(description)?.1,
                rating: WideStr::parse(rating)?.1,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let title_len: u16 = self.title.size_of().try_into()?;
        let author_len: u16 = self.author.size_of().try_into()?;
        let copyright_len: u16 = self.copyright.size_of().try_into()?;
        let description_len: u16 = self.description.size_of().try_into()?;
        let rating_len: u16 = self.rating.size_of().try_into()?;

        w.write_all(&title_len.to_le_bytes())?;
        w.write_all(&author_len.to_le_bytes())?;
        w.write_all(&copyright_len.to_le_bytes())?;
        w.write_all(&description_len.to_le_bytes())?;
        w.write_all(&rating_len.to_le_bytes())?;

        self.title.write(w)?;
        self.author.write(w)?;
        self.copyright.write(w)?;
        self.description.write(w)?;
        self.rating.write(w)?;

        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 2;
        len += 2;
        len += 2;
        len += 2;
        len += 2;
        len += self.title.size_of();
        len += self.author.size_of();
        len += self.copyright.size_of();
        len += self.description.size_of();
        len += self.rating.size_of();
        len
    }
}

#[cfg(test)]
mod tests {
    use crate::header::*;
    use nom::{
        error::{Error, ErrorKind, VerboseError},
        AsBytes,
    };

    use super::*;

    #[test]
    fn broken_content_descriptor() {
        let err = HeaderObject::parse(Span::new(&[
            0x33, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62,
            0xCE, 0x6C, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2E, 0x00, 0x11, 0x00,
            0x02, 0x00, 0x02, 0x00, 0x02, 0x00, 0x54, 0x00, 0x68, 0x00, 0x65, 0x00, 0x20, 0x00,
            0x4D, 0x00, 0x61, 0x00, 0x74, 0x00, 0x72, 0x00, 0x69, 0x00, 0x78, 0x00, 0x20, 0x00,
            0x50, 0x00, 0x61, 0x00, 0x72, 0x00, 0x74, 0x00, 0x20, 0x00, 0x32, 0x00, 0x20, 0x00,
            0x6F, 0x00, 0x66, 0x00, 0x20, 0x00, 0x32, 0x00, 0x00, 0x00, 0x63, 0x00, 0x6F, 0x00,
            0x6E, 0x00, 0x66, 0x00, 0x75, 0x00, 0x7A, 0x00, 0x65, 0x00, 0x64, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ]))
        .expect_err("expected failure on broken header");
        match err {
            nom::Err::Error(Error {
                code: ErrorKind::Eof,
                ..
            }) => {}
            _ => panic!("expected eof error, got {:?}", err),
        }
    }

    const BASIC_CONTENT_DESCRIPTOR_BYTES: &'static [u8] = &[
        0x33, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE,
        0x6C, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2E, 0x00, 0x12, 0x00, 0x02, 0x00,
        0x02, 0x00, 0x02, 0x00, 0x54, 0x00, 0x68, 0x00, 0x65, 0x00, 0x20, 0x00, 0x4D, 0x00, 0x61,
        0x00, 0x74, 0x00, 0x72, 0x00, 0x69, 0x00, 0x78, 0x00, 0x20, 0x00, 0x50, 0x00, 0x61, 0x00,
        0x72, 0x00, 0x74, 0x00, 0x20, 0x00, 0x32, 0x00, 0x20, 0x00, 0x6F, 0x00, 0x66, 0x00, 0x20,
        0x00, 0x32, 0x00, 0x00, 0x00, 0x63, 0x00, 0x6F, 0x00, 0x6E, 0x00, 0x66, 0x00, 0x75, 0x00,
        0x7A, 0x00, 0x65, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn parse_basic_content_descriptor() {
        assert_eq!(
            HeaderObject::parse::<VerboseError<_>>(Span::new(BASIC_CONTENT_DESCRIPTOR_BYTES))
                .expect("parse error")
                .1,
            HeaderObject::ContentDescription(ContentDescriptionData {
                title: WideStr::new("The Matrix Part 2 of 2\0"),
                author: WideStr::new("confuzed\0"),
                copyright: WideStr::new("\0"),
                description: WideStr::new("\0"),
                rating: WideStr::new("\0"),
            })
        )
    }

    #[test]
    fn write_basic_content_descriptor() {
        let mut buf = Vec::new();

        HeaderObject::ContentDescription(ContentDescriptionData {
            title: WideStr::new("The Matrix Part 2 of 2\0"),
            author: WideStr::new("confuzed\0"),
            copyright: WideStr::new("\0"),
            description: WideStr::new("\0"),
            rating: WideStr::new("\0"),
        })
        .write(&mut buf)
        .expect("write to succeed");

        assert_eq!(buf.as_bytes(), &BASIC_CONTENT_DESCRIPTOR_BYTES[..])
    }

    #[test]
    fn size_of_basic_content_descriptor() {
        assert_eq!(
            HeaderObject::ContentDescription(ContentDescriptionData {
                title: WideStr::new("The Matrix Part 2 of 2\0"),
                author: WideStr::new("confuzed\0"),
                copyright: WideStr::new("\0"),
                description: WideStr::new("\0"),
                rating: WideStr::new("\0"),
            })
            .size_of(),
            BASIC_CONTENT_DESCRIPTOR_BYTES.len()
        )
    }
}
