use std::{convert::TryInto, io::Write};

use nom::{IResult, bytes::streaming::take, error::ParseError, number::streaming::le_u32};


#[derive(Debug, PartialEq)]
pub struct ContentBrandingData<'a> {
    pub banner_image_type: u32,
    pub banner_image_data: &'a [u8],
    pub banner_image_url: &'a [u8],
    pub copyright_url: &'a [u8],
}

impl<'a> ContentBrandingData<'a> {
    pub fn parse<E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], Self, E> {
        let (input, banner_image_type) = le_u32(input)?;
        let (input, banner_image_data_size) = le_u32(input)?;
        let (input, banner_image_data) = take(banner_image_data_size)(input)?;
        let (input, banner_image_url_length) = le_u32(input)?;
        let (input, banner_image_url) = take(banner_image_url_length)(input)?;
        let (input, copyright_url_length) = le_u32(input)?;
        let (input, copyright_url) = take(copyright_url_length)(input)?;
        Ok((input, Self{
            banner_image_type,
            banner_image_data,
            banner_image_url,
            copyright_url,
        }))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let banner_image_data_len: u16 = self.banner_image_data.len().try_into()?;
        let banner_image_url_len: u16 = self.banner_image_url.len().try_into()?;
        let copyright_url_len: u16 = self.copyright_url.len().try_into()?;
        w.write_all(&self.banner_image_type.to_le_bytes())?;
        w.write_all(&banner_image_data_len.to_le_bytes())?;
        w.write_all(self.banner_image_data)?;
        w.write_all(&banner_image_url_len.to_le_bytes())?;
        w.write_all(self.banner_image_url)?;
        w.write_all(&copyright_url_len.to_le_bytes())?;
        w.write_all(self.copyright_url)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        4 + 4 + self.banner_image_data.len() + 4 + self.banner_image_url.len() + 4 + self.copyright_url.len()
    }
}
