use crate::{
    combinators::{length_take, span_bytes},
    error::Error,
    span::Span,
};
use nom::bits::streaming::tag as tag_bits;
use nom::bits::streaming::take as take_bits;
use nom::{
    bits::bits,
    combinator::{map, peek},
    multi::many0,
    number::streaming::{le_u16, le_u32, le_u8},
    IResult,
};
use nom::{
    branch::alt,
    bytes::streaming::take,
    combinator::{complete, rest, rest_len, value},
    error::context,
    multi::count,
};

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
    NormalPayload {
        stream_flags: StreamFlags,
        media_object_number: u32,
        offset_into_media_object: u32,
        replicated_data: &'a [u8],
        payload_data: &'a [u8],
    },
    CompressedPayload {
        stream_flags: StreamFlags,
        media_object_number: u32,
        presentation_time: u32,
        presentation_time_delta: u8,
        sub_payload_data: Vec<&'a [u8]>,
    },
}

#[derive(Debug, PartialEq)]
pub enum PayloadData<'a> {
    SinglePayload(Payload<'a>),
    MultiplePayloads(Vec<Payload<'a>>),
}

impl MultiplePayloadsFlag {
    pub fn parse(
        data: (Span, usize),
    ) -> IResult<(Span, usize), MultiplePayloadsFlag, Error<(Span, usize)>> {
        context(
            "MultiplePayloadsFlag",
            alt((
                value(MultiplePayloadsFlag::SinglePayload, tag_bits(0, 1usize)),
                value(MultiplePayloadsFlag::MultiplePayloads, tag_bits(1, 1usize)),
            )),
        )(data)
    }
}

impl ErrorCorrectionFlag {
    pub fn parse(
        data: (Span, usize),
    ) -> IResult<(Span, usize), ErrorCorrectionFlag, Error<(Span, usize)>> {
        context(
            "ErrorCorrectionFlag",
            nom::combinator::map(nom::bits::complete::take(1usize), |x: u8| match x == 1 {
                true => ErrorCorrectionFlag::Present,
                false => ErrorCorrectionFlag::Absent,
            }),
        )(data)
    }
}

impl FieldType {
    pub fn parse(
        input: (Span, usize),
    ) -> IResult<(Span, usize), Self, Error<(Span, usize)>> {
        context(
            "FieldType",
            nom::combinator::map(nom::bits::complete::take(2usize), |x: u8| match x {
                1 => Self::Byte,
                2 => Self::Word,
                3 => Self::Dword,
                _ => Self::None,
            }),
        )(input)
    }

    pub fn field<'a>(self) -> impl Fn(Span<'a>) -> IResult<Span<'a>, u32, Error<Span<'a>>> {
        move |input: Span| -> IResult<Span, u32, Error<Span<'a>>> {
            match self {
                Self::None => Ok((input, 0)),
                Self::Byte => map(le_u8, |x| x as u32)(input),
                Self::Word => map(le_u16, |x| x as u32)(input),
                Self::Dword => map(le_u32, |x| x as u32)(input),
            }
        }
    }
}

impl LengthTypeFlags {
    pub fn parse<'a>(
        input: (Span<'a>, usize),
    ) -> IResult<(Span<'a>, usize), Self, Error<(Span<'a>, usize)>> {
        context("LengthTypeFlags", move |input: (Span<'a>, usize)| {
            let (input, error_correction_flag) = ErrorCorrectionFlag::parse(input)?;
            let (input, packet_len_type) = FieldType::parse(input)?;
            let (input, padding_len_type) = FieldType::parse(input)?;
            let (input, sequence_type) = FieldType::parse(input)?;
            let (input, multiple_payloads_present) = MultiplePayloadsFlag::parse(input)?;
            Ok((
                input,
                Self {
                    multiple_payloads_present,
                    sequence_type,
                    padding_len_type,
                    packet_len_type,
                    error_correction_flag,
                },
            ))
        })(input)
    }
}

impl PropertyFlags {
    pub fn parse<'a>(
        input: (Span<'a>, usize),
    ) -> IResult<(Span<'a>, usize), Self, Error<(Span<'a>, usize)>> {
        context("PropertyFlags", move |input: (Span<'a>, usize)| {
            let (input, stream_number_len_type) = FieldType::parse(input)?;
            let (input, media_object_number_len_type) = FieldType::parse(input)?;
            let (input, offset_into_media_object_len_type) = FieldType::parse(input)?;
            let (input, replicated_data_len_type) = FieldType::parse(input)?;
            Ok((
                input,
                Self {
                    replicated_data_len_type,
                    offset_into_media_object_type: offset_into_media_object_len_type,
                    media_object_number_len_type,
                    stream_number_len_type,
                },
            ))
        })(input)
    }
}

impl PayloadFlags {
    pub fn parse<'a>(
        input: (Span<'a>, usize),
    ) -> IResult<(Span<'a>, usize), Self, Error<(Span<'a>, usize)>> {
        context("PayloadFlags", move |input: (Span<'a>, usize)| {
            let (input, payload_len_type) = FieldType::parse(input)?;
            let (input, number_of_payloads) = take_bits(6usize)(input)?;
            Ok((
                input,
                Self {
                    number_of_payloads,
                    payload_len_type,
                },
            ))
        })(input)
    }
}

impl StreamFlags {
    pub fn parse<'a>(
        input: (Span<'a>, usize),
    ) -> IResult<(Span<'a>, usize), Self, Error<(Span<'a>, usize)>> {
        context("StreamFlags", move |input: (Span<'a>, usize)| {
            let (input, key_frame) = map(take_bits(1usize), |x: u8| x == 1)(input)?;
            let (input, stream_number) = take_bits(7usize)(input)?;
            Ok((
                input,
                Self {
                    stream_number,
                    key_frame,
                },
            ))
        })(input)
    }
}

impl<'a> DataPacket<'a> {
    pub fn parser(
        total_data_packets: u64,
        total_packet_len: u64,
    ) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, DataPacket, Error<Span<'a>>> {
        let fixed_packet_len = total_packet_len / total_data_packets;
        context("DataPacket", move |input: Span<'a>| {
            let initial_remainder = rest_len(input)?.1;
            let (input, error_correction_present) = map(peek(le_u8), |x| x & 0x80 != 0)(input)?;
            let (input, error_correction_data) =
                nom::combinator::cond(error_correction_present, ErrorCorrectionData::parse)(input)?;
            let (input, payload_parsing_data) = PayloadParsingData::parse(input)?;
            let header_len = initial_remainder - rest_len(input)?.1;
            let (input, raw_payload) = match payload_parsing_data.packet_length {
                0 => take(
                    fixed_packet_len - header_len as u64 - payload_parsing_data.padding_len as u64,
                )(input)?,
                _ => take(payload_parsing_data.packet_length)(input)?,
            };
            let payload_parser = {
                PayloadData::parser(
                    payload_parsing_data
                        .length_type_flags
                        .multiple_payloads_present,
                    payload_parsing_data.property_flags,
                )
            };
            let payload = payload_parser(raw_payload)?.1;
            let (input, _) = take(payload_parsing_data.padding_len)(input)?;
            Ok((
                input,
                DataPacket {
                    error_correction_data,
                    payload_parsing_data,
                    payload,
                },
            ))
        })
    }
}

impl ErrorCorrectionData {
    pub fn parse<'a>(input: Span<'a>) -> IResult<Span<'a>, ErrorCorrectionData, Error<Span<'a>>> {
        context("ErrorCorrectionData", move |input: Span<'a>| {
            let (input, flags) = le_u8(input)?;
            let (input, ec_type) = le_u8(input)?;
            let (input, ec_cycle) = le_u8(input)?;
            Ok((
                input,
                ErrorCorrectionData {
                    flags,
                    ec_type,
                    ec_cycle,
                },
            ))
        })(input)
    }
}

impl PayloadParsingData {
    pub fn parse<'a>(input: Span<'a>) -> IResult<Span<'a>, PayloadParsingData, Error<Span<'a>>> {
        context("PayloadParsingData", move |input: Span<'a>| {
            let (input, length_type_flags) = bits(LengthTypeFlags::parse)(input)?;
            let (input, property_flags) = bits(PropertyFlags::parse)(input)?;
            let (input, packet_length) = length_type_flags.packet_len_type.field()(input)?;
            let (input, sequence) = length_type_flags.sequence_type.field()(input)?;
            let (input, padding_len) = length_type_flags.padding_len_type.field()(input)?;
            let (input, send_time) = le_u32(input)?;
            let (input, duration) = le_u16(input)?;
            Ok((
                input,
                PayloadParsingData {
                    length_type_flags,
                    property_flags,
                    packet_length,
                    sequence,
                    padding_len,
                    send_time,
                    duration,
                },
            ))
        })(input)
    }
}

impl<'a> PayloadData<'a> {
    pub fn parser(
        multiple: MultiplePayloadsFlag,
        property_flags: PropertyFlags,
    ) -> impl Fn(Span<'a>) -> IResult<Span<'a>, PayloadData<'a>, Error<Span<'a>>> {
        move |input: Span| match multiple {
            MultiplePayloadsFlag::SinglePayload => {
                Self::parser_single(property_flags)(input)
            }
            MultiplePayloadsFlag::MultiplePayloads => {
                Self::parser_multi(property_flags)(input)
            }
        }
    }

    pub fn parser_single(
        property_flags: PropertyFlags,
    ) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, PayloadData<'a>, Error<Span<'a>>> {
        map(Payload::parser(property_flags, None), |x| {
            PayloadData::SinglePayload(x)
        })
    }

    pub fn parser_multi(
        property_flags: PropertyFlags,
    ) -> impl Fn(Span<'a>) -> IResult<Span<'a>, PayloadData<'a>, Error<Span<'a>>> {
        move |input: Span| {
            let (input, payload_flags) = bits(PayloadFlags::parse)(input)?;
            let (input, payloads) = count(
                Payload::parser(property_flags, Some(payload_flags.payload_len_type)),
                payload_flags.number_of_payloads.into(),
            )(input)?;
            Ok((input, PayloadData::MultiplePayloads(payloads)))
        }
    }
}

impl<'a> Payload<'a> {
    pub fn parser(
        property_flags: PropertyFlags,
        payload_length_type: Option<FieldType>,
    ) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Payload, Error<Span<'a>>> {
        context("Payload", move |input: Span<'a>| {
            let (input, stream_flags) = bits(StreamFlags::parse)(input)?;
            let (input, media_object_number) =
                property_flags.media_object_number_len_type.field()(input)?;
            let (input, time_or_offset) =
                property_flags.offset_into_media_object_type.field()(input)?;
            let (input, replicated_data_len) =
                property_flags.replicated_data_len_type.field()(input)?;

            if replicated_data_len == 1 {
                // Compressed
                let (input, presentation_time_delta) = le_u8(input)?;
                let (input, payload_raw) = if let Some(len_type) = payload_length_type {
                    length_take(len_type.field())(input)?
                } else {
                    rest(input)?
                };
                let (_, sub_payload_data) =
                    many0(complete(span_bytes(length_take(le_u8))))(payload_raw)?;
                Ok((
                    input,
                    Payload::CompressedPayload {
                        stream_flags,
                        media_object_number,
                        presentation_time: time_or_offset,
                        presentation_time_delta,
                        sub_payload_data,
                    },
                ))
            } else {
                // Uncompressed
                let (input, replicated_data) = span_bytes(take(replicated_data_len))(input)?;
                let (input, payload_data) = if let Some(len_type) = payload_length_type {
                    span_bytes(length_take(len_type.field()))(input)?
                } else {
                    span_bytes(rest)(input)?
                };
                Ok((
                    input,
                    Payload::NormalPayload {
                        stream_flags,
                        media_object_number,
                        offset_into_media_object: time_or_offset,
                        replicated_data,
                        payload_data,
                    },
                ))
            }
        })
    }
}
