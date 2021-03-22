use nom::{IResult, InputIter, InputLength, InputTake, Parser, ToUsize, bytes::streaming::take, error::ParseError};
use nom_locate::LocatedSpan;

pub fn length_take<I, N, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, I, E>
where
  I: InputIter + InputTake + InputLength,
  N: ToUsize,
  F: Parser<I, N, E>,
  E: ParseError<I>,
{
  move |i: I| {
    let (i, count) = f.parse(i)?;
    take(count)(i)
  }
}

pub fn span_bytes<'a, E, F>(mut f: F) -> impl FnMut(LocatedSpan<&'a [u8]>) -> IResult<LocatedSpan<&'a [u8]>, &'a [u8], E>
where
  E: ParseError<LocatedSpan<&'a [u8]>>,
  F: Parser<LocatedSpan<&'a [u8]>, LocatedSpan<&'a [u8]>, E>,
{
  move |i: LocatedSpan<&[u8]>| {
    let (i, o1) = f.parse(i)?;
    Ok((i, o1.fragment()))
  }
}
