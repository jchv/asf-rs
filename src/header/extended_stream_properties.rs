use super::stream_properties::*;
use crate::{guid::*, object::*, span::Span, widestr::*};
use nom::{
    bytes::streaming::take,
    combinator::opt,
    error::ParseError,
    multi::count,
    number::streaming::{le_u16, le_u32, le_u64},
    IResult,
};
use std::{convert::TryInto, io::Write};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct StreamName {
    pub language_id_index: u16,
    pub stream_name: WideStr,
}

#[derive(Debug, PartialEq)]
pub struct PayloadExtensionSystem<'a> {
    pub id: Uuid,
    pub data_size: u16,
    pub info: Span<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ExtendedStreamPropertiesData<'a> {
    pub start_time: u64,
    pub end_time: u64,
    pub data_bitrate: u32,
    pub buffer_size: u32,
    pub initial_buffer_fullness: u32,
    pub alternate_data_bitrate: u32,
    pub alternate_buffer_size: u32,
    pub alternate_initial_buffer_fullness: u32,
    pub maximum_object_size: u32,
    pub flags: u32,
    pub stream_number: u16,
    pub stream_language_id_index: u16,
    pub average_time_per_frame: u64,
    pub stream_names: Vec<StreamName>,
    pub payload_extension_systems: Vec<PayloadExtensionSystem<'a>>,
    pub stream_properties_object: Option<StreamPropertiesData<'a>>,
}

impl StreamName {
    pub fn parse<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, language_id_index) = le_u16(input)?;
        let (input, stream_name_length) = le_u16(input)?;
        let (input, stream_name) = take(stream_name_length)(input)?;
        Ok((
            input,
            Self {
                language_id_index,
                stream_name: WideStr::parse(stream_name)?.1,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let stream_name_len: u16 = self.stream_name.size_of().try_into()?;
        w.write_all(&self.language_id_index.to_le_bytes())?;
        w.write_all(&stream_name_len.to_le_bytes())?;
        self.stream_name.write(w)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 2;
        len += 2;
        len += self.stream_name.size_of();
        len
    }
}

impl<'a> PayloadExtensionSystem<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, id) = guid(input)?;
        let (input, data_size) = le_u16(input)?;
        let (input, info_length) = le_u32(input)?;
        let (input, info) = take(info_length)(input)?;
        Ok((
            input,
            PayloadExtensionSystem {
                id,
                data_size,
                info,
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let info_len: u32 = self.info.len().try_into()?;
        w.write_all(&self.id.as_bytes_ms())?;
        w.write_all(&self.data_size.to_le_bytes())?;
        w.write_all(&info_len.to_le_bytes())?;
        w.write_all(&self.info)?;
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 16;
        len += 2;
        len += 4;
        len += self.info.len();
        len
    }
}

impl<'a> ExtendedStreamPropertiesData<'a> {
    pub fn parse<E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Self, E> {
        let (input, start_time) = le_u64(input)?;
        let (input, end_time) = le_u64(input)?;
        let (input, data_bitrate) = le_u32(input)?;
        let (input, buffer_size) = le_u32(input)?;
        let (input, initial_buffer_fullness) = le_u32(input)?;
        let (input, alternate_data_bitrate) = le_u32(input)?;
        let (input, alternate_buffer_size) = le_u32(input)?;
        let (input, alternate_initial_buffer_fullness) = le_u32(input)?;
        let (input, maximum_object_size) = le_u32(input)?;
        let (input, flags) = le_u32(input)?;
        let (input, stream_number) = le_u16(input)?;
        let (input, stream_language_id_index) = le_u16(input)?;
        let (input, average_time_per_frame) = le_u64(input)?;
        let (input, stream_name_count) = le_u16(input)?;
        let (input, payload_extension_system_count) = le_u16(input)?;
        let (input, stream_names) = count(StreamName::parse, stream_name_count.into())(input)?;
        let (input, payload_extension_systems) = count(
            PayloadExtensionSystem::parse,
            payload_extension_system_count.into(),
        )(input)?;
        let (input, stream_properties_object) = opt(object)(input)?;
        Ok((
            input,
            ExtendedStreamPropertiesData {
                start_time,
                end_time,
                data_bitrate,
                buffer_size,
                initial_buffer_fullness,
                alternate_data_bitrate,
                alternate_buffer_size,
                alternate_initial_buffer_fullness,
                maximum_object_size,
                flags,
                stream_number,
                stream_language_id_index,
                average_time_per_frame,
                stream_names,
                payload_extension_systems,
                stream_properties_object: stream_properties_object
                    .map(|x| StreamPropertiesData::parse(x.data))
                    .map_or(Ok(None), |r| r.map(Some))?
                    .map(|x| x.1),
            },
        ))
    }

    pub fn write<T: Write>(&self, w: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        let stream_name_count: u16 = self.stream_names.len().try_into()?;
        let payload_extension_system_count: u16 =
            self.payload_extension_systems.len().try_into()?;
        w.write_all(&self.start_time.to_le_bytes())?;
        w.write_all(&self.end_time.to_le_bytes())?;
        w.write_all(&self.data_bitrate.to_le_bytes())?;
        w.write_all(&self.buffer_size.to_le_bytes())?;
        w.write_all(&self.initial_buffer_fullness.to_le_bytes())?;
        w.write_all(&self.alternate_data_bitrate.to_le_bytes())?;
        w.write_all(&self.alternate_buffer_size.to_le_bytes())?;
        w.write_all(&self.alternate_initial_buffer_fullness.to_le_bytes())?;
        w.write_all(&self.maximum_object_size.to_le_bytes())?;
        w.write_all(&self.flags.to_le_bytes())?;
        w.write_all(&self.stream_number.to_le_bytes())?;
        w.write_all(&self.stream_language_id_index.to_le_bytes())?;
        w.write_all(&self.average_time_per_frame.to_le_bytes())?;
        w.write_all(&stream_name_count.to_le_bytes())?;
        w.write_all(&payload_extension_system_count.to_le_bytes())?;
        for stream_name in self.stream_names.iter() {
            stream_name.write(w)?;
        }
        for payload_extension_system in self.payload_extension_systems.iter() {
            payload_extension_system.write(w)?;
        }
        if let Some(stream_properties_object) = &self.stream_properties_object {
            stream_properties_object.write(w)?;
        }
        Ok(())
    }

    pub fn size_of(&self) -> usize {
        let mut len = 0;
        len += 8;
        len += 8;
        len += 4;
        len += 4;
        len += 4;
        len += 4;
        len += 4;
        len += 4;
        len += 4;
        len += 4;
        len += 2;
        len += 2;
        len += 8;
        len += 2;
        len += 2;
        for stream_name in self.stream_names.iter() {
            len += stream_name.size_of();
        }
        for payload_extension_system in self.payload_extension_systems.iter() {
            len += payload_extension_system.size_of();
        }
        if let Some(stream_properties_object) = &self.stream_properties_object {
            len += stream_properties_object.size_of();
        }
        len
    }
}
