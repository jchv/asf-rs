use nom::number::streaming::le_u32;


#[derive(Debug, PartialEq)]
pub struct ContentBrandingData<'a> {
    pub banner_image_type: u32,
    pub banner_image_data: &'a [u8],
    pub banner_image_url: &'a [u8],
    pub copyright_url: &'a [u8],
}

impl<'a> ContentBrandingData<'a> {
    named!(pub parse<ContentBrandingData>,
        do_parse!(
            banner_image_type: le_u32 >>
            banner_image_data_size: le_u32 >>
            banner_image_data: take!(banner_image_data_size) >>
            banner_image_url_length: le_u32 >>
            banner_image_url: take!(banner_image_url_length) >>
            copyright_url_length: le_u32 >>
            copyright_url: take!(copyright_url_length) >>
            (ContentBrandingData{
                banner_image_type,
                banner_image_data,
                banner_image_url,
                copyright_url,
            })
        )
    );
}
