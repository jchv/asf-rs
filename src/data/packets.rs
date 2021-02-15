use nom::{ErrorConvert, InputLength, bytes::streaming::take, combinator::{complete, rest}, error::ParseError, multi::count};
use nom::{IResult, bits::bits, combinator::{map, peek}, multi::many0, number::streaming::{le_u8, le_u16, le_u32}};
use nom::bits::streaming::take as take_bits;

use crate::combinators::length_take;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MultiplePayloadsFlag {
    SinglePayload,
    MultiplePayloads,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorCorrectionFlag {
    Absent,
    Present,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FieldType {
    None,
    Byte,
    Word,
    Dword,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LengthTypeFlags {
    multiple_payloads_present: MultiplePayloadsFlag,
    sequence_type: FieldType,
    padding_len_type: FieldType,
    packet_len_type: FieldType,
    error_correction_flag: ErrorCorrectionFlag,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PropertyFlags {
    replicated_data_len_type: FieldType,
    offset_into_media_object_type: FieldType,
    media_object_number_len_type: FieldType,
    stream_number_len_type: FieldType,
}

#[derive(Debug, PartialEq)]
pub struct PayloadFlags {
    number_of_payloads: u8,
    payload_len_type: FieldType,
}

#[derive(Debug, PartialEq)]
pub struct StreamFlags {
    stream_number: u8,
    key_frame: bool,
}

#[derive(Debug, PartialEq)]
pub struct DataPacket<'a> {
    error_correction_data: Option<ErrorCorrectionData>,
    payload_parsing_data: PayloadParsingData,
    payload: PayloadData<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ErrorCorrectionData {
    flags: u8,
    ec_type: u8,
    ec_cycle: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PayloadParsingData {
    length_type_flags: LengthTypeFlags,
    property_flags: PropertyFlags,
    packet_length: u32,
    sequence: u32,
    padding_len: u32,
    send_time: u32,
    duration: u16,
}

#[derive(Debug, PartialEq)]
pub enum Payload<'a> {
    NormalPayload{
        stream_flags: StreamFlags,
        media_object_number: u32,
        offset_into_media_object: u32,
        replicated_data: &'a [u8],
        payload_data: &'a [u8],
    },
    CompressedPayload{
        stream_flags: StreamFlags,
        media_object_number: u32,
        presentation_time: u32,
        presentation_time_delta: u8,
        sub_payload_data: Vec<&'a [u8]>,
    }
}

#[derive(Debug, PartialEq)]
pub enum PayloadData<'a> {
    SinglePayload(Payload<'a>),
    MultiplePayloads(Vec<Payload<'a>>),
}

impl MultiplePayloadsFlag {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(data: (&'a[u8], usize)) -> IResult<(&'a[u8], usize), MultiplePayloadsFlag, E> {
        nom::combinator::map(
            nom::bits::complete::take(1usize),
            |x: u8| match x == 1 {
                true => MultiplePayloadsFlag::MultiplePayloads,
                false => MultiplePayloadsFlag::SinglePayload,
            }
        )(data)
    }
}

impl ErrorCorrectionFlag {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(data: (&'a[u8], usize)) -> IResult<(&'a[u8], usize), ErrorCorrectionFlag, E> {
        nom::combinator::map(
            nom::bits::complete::take(1usize),
            |x: u8| match x == 1 {
                true => ErrorCorrectionFlag::Present,
                false => ErrorCorrectionFlag::Absent,
            }
        )(data)
    }
}

impl FieldType {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), FieldType, E> {
        nom::combinator::map(
            nom::bits::complete::take(2usize),
            |x: u8| match x {
                1 => FieldType::Byte,
                2 => FieldType::Word,
                3 => FieldType::Dword,
                _ => FieldType::None,
            }
        )(input)
    }

    pub fn field<'a, E: ParseError<&'a[u8]>>(self) -> impl Fn(&'a [u8]) -> IResult<&'a[u8], u32, E> {
        move |input: &[u8]| -> IResult<&[u8], u32, E> {
            match self {
                FieldType::None => Ok((input, 0)),
                FieldType::Byte => map(le_u8, |x| x as u32)(input),
                FieldType::Word => map(le_u16, |x| x as u32)(input),
                FieldType::Dword => map(le_u32, |x| x as u32)(input),
            }
        }
    }
}

impl LengthTypeFlags {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(data: (&'a[u8], usize)) -> IResult<(&'a[u8], usize), LengthTypeFlags, E> {
        let (data, multiple_payloads_present) = MultiplePayloadsFlag::parse(data)?;
        let (data, sequence_type) = FieldType::parse(data)?;
        let (data, padding_len_type) = FieldType::parse(data)?;
        let (data, packet_len_type) = FieldType::parse(data)?;
        let (data, error_correction_flag) = ErrorCorrectionFlag::parse(data)?;
        return Ok((data, LengthTypeFlags{
            multiple_payloads_present,
            sequence_type,
            padding_len_type,
            packet_len_type,
            error_correction_flag,
        }))
    }
}

impl PropertyFlags {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(data: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), PropertyFlags, E> {
        let (data, replicated_data_len_type) = FieldType::parse(data)?;
        let (data, offset_into_media_object_len_type) = FieldType::parse(data)?;
        let (data, media_object_number_len_type) = FieldType::parse(data)?;
        let (data, stream_number_len_type) = FieldType::parse(data)?;
        return Ok((data, PropertyFlags{
            replicated_data_len_type,
            offset_into_media_object_type: offset_into_media_object_len_type,
            media_object_number_len_type,
            stream_number_len_type,
        }))
    }
}

impl PayloadFlags {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(input: (&'a[u8], usize)) -> IResult<(&'a[u8], usize), PayloadFlags, E> {
        let (input, number_of_payloads) = take_bits(6usize)(input)?;
        let (input, payload_len_type) = FieldType::parse(input)?;
        return Ok((input, PayloadFlags{
            number_of_payloads,
            payload_len_type,
        }))
    }
}

impl StreamFlags {
    pub fn parse<'a, E: ParseError<(&'a[u8], usize)>>(input: (&'a[u8], usize)) -> IResult<(&'a[u8], usize), StreamFlags, E> {
        let (input, stream_number) = take_bits(7usize)(input)?;
        let (input, key_frame) = map(take_bits(1usize), |x: u8| x == 1)(input)?;
        return Ok((input, StreamFlags{
            stream_number,
            key_frame,
        }))
    }
}

impl<'a> DataPacket<'a> {
    pub fn parse<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], DataPacket, E2> {
        let (input, error_correction_present) = map(peek(le_u8), |x| x & 0x80 != 0)(input)?;
        let (input, error_correction_data) = nom::combinator::cond(error_correction_present, ErrorCorrectionData::parse)(input)?;
        let (input, payload_parsing_data) = PayloadParsingData::parse::<E1, E2>(input)?;
        let (input, raw_payload_data) = if payload_parsing_data.packet_length == 0 {
            take(input.input_len() as u32 - payload_parsing_data.padding_len)(input)?
        } else {
            take(payload_parsing_data.packet_length - payload_parsing_data.padding_len)(input)?
        };
        let (input, _) = take(payload_parsing_data.padding_len)(input)?;
        let payload_parser = PayloadData::parser::<E1, E2>(payload_parsing_data.length_type_flags.multiple_payloads_present, payload_parsing_data.property_flags);
        let payload = payload_parser(raw_payload_data)?.1;
        Ok((input, DataPacket{
            error_correction_data,
            payload_parsing_data,
            payload,
        }))
    }
}

impl ErrorCorrectionData {
    pub fn parse<'a, E: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], ErrorCorrectionData, E> {
        let (input, flags) = le_u8(input)?;
        let (input, ec_type) = le_u8(input)?;
        let (input, ec_cycle) = le_u8(input)?;
        Ok((input, ErrorCorrectionData{
            flags,
            ec_type,
            ec_cycle,
        }))
    }
}

impl PayloadParsingData {
    pub fn parse<'a, E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(input: &'a[u8]) -> IResult<&'a[u8], PayloadParsingData, E2> {
        let (input, length_type_flags) = bits::<_, _, E1, _, _>(LengthTypeFlags::parse)(input)?;
        let (input, property_flags) = bits::<_, _, E1, _, _>(PropertyFlags::parse)(input)?;
        let (input, packet_length) = length_type_flags.packet_len_type.field()(input)?;
        let (input, sequence) = length_type_flags.sequence_type.field()(input)?;
        let (input, padding_len) = length_type_flags.padding_len_type.field()(input)?;
        let (input, send_time) = le_u32(input)?;
        let (input, duration) = le_u16(input)?;
        Ok((input, PayloadParsingData{
            length_type_flags,
            property_flags,
            packet_length,
            sequence,
            padding_len,
            send_time,
            duration,
        }))
    }
}

impl<'a> PayloadData<'a> {
    pub fn parser<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(multiple: MultiplePayloadsFlag, property_flags: PropertyFlags) -> impl Fn(&'a[u8]) -> IResult<&'a[u8], PayloadData<'a>, E2> {
        move |input: &[u8]| -> IResult<&[u8], PayloadData, E2> {
            match multiple {
                MultiplePayloadsFlag::SinglePayload => Self::parser_single::<E1, E2>(property_flags.clone())(input),
                MultiplePayloadsFlag::MultiplePayloads => Self::parser_multi::<E1, E2>(property_flags.clone())(input),
            }
        }
    }

    pub fn parser_single<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(property_flags: PropertyFlags) -> impl FnMut(&'a[u8]) -> IResult<&'a[u8], PayloadData<'a>, E2> {
        map(Payload::parser::<E1, E2>(property_flags, None), |x| PayloadData::SinglePayload(x))
    }

    pub fn parser_multi<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(property_flags: PropertyFlags) -> impl Fn(&'a[u8]) -> IResult<&'a[u8], PayloadData<'a>, E2> {
        move |input: &[u8]| -> IResult<&[u8], PayloadData, E2> {
            let (input, payload_flags) = bits::<_, _, E1, _, _>(PayloadFlags::parse)(input)?;
            let (input, payloads) = count(Payload::parser::<E1, E2>(property_flags.clone(), Some(payload_flags.payload_len_type)), payload_flags.number_of_payloads.into())(input)?;
            Ok((input, PayloadData::MultiplePayloads(payloads)))
        }
    }
}

impl<'a> Payload<'a> {
    pub fn parser<E1: ParseError<(&'a[u8], usize)> + ErrorConvert<E2>, E2: ParseError<&'a[u8]>>(property_flags: PropertyFlags, payload_length_type: Option<FieldType>) -> impl Fn(&'a[u8]) -> IResult<&'a[u8], Payload, E2> {
        move |input: &[u8]| -> IResult<&'a[u8], Payload, E2> {
            let (input, stream_flags) = bits::<_, _, E1, _, _>(StreamFlags::parse)(input)?;
            let (input, media_object_number) = property_flags.media_object_number_len_type.field()(input)?;
            let (input, time_or_offset) = property_flags.offset_into_media_object_type.field()(input)?;
            let (input, replicated_data_len) = property_flags.replicated_data_len_type.field()(input)?;

            if replicated_data_len == 1 {
                // Compressed
                let (input, presentation_time_delta) = le_u8(input)?;
                let (input, payload_data) = if let Some(len_type) = payload_length_type {
                    length_take(len_type.field())(input)?
                } else {
                    rest(input)?
                };
                let sub_payload_data = complete(many0(length_take(le_u8)))(payload_data)?.1;
                Ok((input, Payload::CompressedPayload{
                    stream_flags,
                    media_object_number,
                    presentation_time: time_or_offset,
                    presentation_time_delta,
                    sub_payload_data,
                }))
            } else {
                // Uncompressed
                let (input, replicated_data) = take(replicated_data_len)(input)?;
                let (input, payload_data) = if let Some(len_type) = payload_length_type {
                    length_take(len_type.field())(input)?
                } else {
                    rest(input)?
                };
                Ok((input, Payload::NormalPayload{
                    stream_flags,
                    media_object_number,
                    offset_into_media_object: time_or_offset,
                    replicated_data,
                    payload_data,
                }))
            }
        }
    }
}
