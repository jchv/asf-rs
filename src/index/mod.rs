use crate::object::*;

#[derive(Debug, PartialEq)]
pub struct IndexObject {
}

#[derive(Debug, PartialEq)]
pub struct IndexObjects {
    pub objects: Vec<IndexObject>
}

impl IndexObject {
    named!(pub parse<IndexObject>,
        do_parse!(
            header: object_header >>
            data: take!(header.size - 24) >>
            (IndexObject{})
        )
    );
}

impl IndexObjects {
    named!(pub parse<IndexObjects>,
        do_parse!(
            objects: many0!(complete!(IndexObject::parse)) >>
            (IndexObjects{objects})
        )
    );
}
