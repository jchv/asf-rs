use asf::parse;
use std::{env::args, fs::File, io::Read};

fn main() {
    for name in args().skip(1) {
        let mut buffer = Vec::new();
        let mut f = File::open(name).expect("opening file failed");
        f.read_to_end(&mut buffer).expect("reading file failed");
        println!("{:?}", parse(&buffer).expect("parsing file failed"));
    }
}
