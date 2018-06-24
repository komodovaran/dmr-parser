extern crate flate2;
extern crate xml;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::str::from_utf8;
use flate2::bufread::DeflateDecoder;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let f = File::open("ESStatistikListeModtag-20180617-233202.zip").unwrap();
    let mut f = BufReader::new(f);
    let mut header_buf = [0; 30];

    f.read_exact(&mut header_buf).expect("unable to read header");

    assert_eq!(header_buf[..4], [0x50, 0x4b, 0x03, 0x04]);
    let name_len = ((header_buf[27] as usize) << 8) + header_buf[26] as usize;
    let extra_len = ((header_buf[29] as usize) << 8) + header_buf[28] as usize;

    let mut name_extra_buf = vec![0; name_len + extra_len];
    f.read_exact(&mut name_extra_buf);
    println!("{}", from_utf8(&name_extra_buf[..name_len]).unwrap());

    let mut deflater = DeflateDecoder::new(f);
    let mut deflater = BufReader::new(deflater);

    let parser = EventReader::new(deflater);

    let mut count = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {ref name, ..}) if name.local_name == "Statistik" => count += 1,
            _ => continue,
        }
        if count % 1000 == 0 {
            println!("{}", count);
        }
    }
    println!("{}", count);
}