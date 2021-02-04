use nom::number::streaming::le_u8;


#[derive(Debug, PartialEq)]
pub struct CompatibilityData {
    pub profile: u8,
    pub mode: u8,
}

impl CompatibilityData {
    named!(pub parse<Self>,
        do_parse!(
            profile: le_u8 >>
            mode: le_u8 >>
            (Self{
                profile,
                mode,
            })
        )
    );
}
