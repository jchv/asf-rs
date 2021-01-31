use uuid::Uuid;
use nom::number::streaming::{le_u64, le_u8};


named!(pub guid<Uuid>,
    do_parse!(
        b00: le_u8 >> b01: le_u8 >> b02: le_u8 >> b03: le_u8 >>
        b04: le_u8 >> b05: le_u8 >> b06: le_u8 >> b07: le_u8 >>
        b08: le_u8 >> b09: le_u8 >> b10: le_u8 >> b11: le_u8 >>
        b12: le_u8 >> b13: le_u8 >> b14: le_u8 >> b15: le_u8 >>
        (Uuid::from_bytes([
            b03, b02, b01, b00, b05, b04, b07, b06,
            b08, b09, b10, b11, b12, b13, b14, b15,
        ]))
    )
);

pub struct ObjectHeader {
    pub guid: Uuid,
    pub size: u64,
}

named!(pub object_header<ObjectHeader>,
    do_parse!(guid: guid >> size: le_u64 >> (ObjectHeader{guid, size}))
);

pub enum Object<'a> {
    Unknown(Uuid, &'a [u8])
}

named!(pub object<Object>,
    switch!(object_header,
        ObjectHeader{guid, size} => do_parse!(
            data: take!(size as usize) >>
            (Object::Unknown(guid, data))
        )
    )
);

#[cfg(test)]
mod tests {
    use crate::guid::HEADER_OBJECT;

    use super::*;

    #[test]
    fn guids() {
        assert_eq!(
            guid(&[
                0x30, 0x26, 0xb2, 0x75, 0x8e, 0x66, 0xcf, 0x11,
                0xa6, 0xd9, 0x00, 0xaa, 0x00, 0x62, 0xce, 0x6c,
            ]),
            Ok((&b""[..], HEADER_OBJECT))
        );
    }
}
