use std::fmt;

use nom::number::streaming::{le_u16, le_u32};


#[derive(PartialEq)]
pub struct WideStr(Vec<u16>);

impl WideStr {
    pub fn from_str(s: &str) -> Self {
        let w: Vec<u16> = s.encode_utf16().collect();
        return WideStr(w);
    }

    pub fn to_str(&self) -> String {
        String::from_utf16_lossy(&self.0)
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

named!(pub wchar_str<WideStr>,
    map!(terminated!(many0!(complete!(le_u16)), eof!()), |x| WideStr(x)));

named!(pub len16_prefixed_widestr<WideStr>,
    map!(length_count!(le_u16, le_u16), |x| WideStr(x)));

named!(pub len32_prefixed_widestr<WideStr>,
    map!(length_count!(le_u32, le_u16), |x| WideStr(x)));
