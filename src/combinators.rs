use nom::{IResult, InputIter, InputLength, InputTake, Parser, ToUsize, bytes::streaming::take, error::ParseError};

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
