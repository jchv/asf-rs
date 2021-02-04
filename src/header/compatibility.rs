use nom::number::streaming::le_u8;


#[derive(Debug, PartialEq)]
pub struct CompatibilityData {
    profile: u8,
    mode: u8,
}

named!(pub compatibility_data<CompatibilityData>,
    do_parse!(
        profile: le_u8 >>
        mode: le_u8 >>
        (CompatibilityData{
            profile,
            mode,
        })
    )
);
