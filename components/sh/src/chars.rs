use std::io::{Read, ErrorKind, Result};
use std::str;

// https://tools.ietf.org/html/rfc3629
static UTF8_CHAR_WIDTH: [u8; 256] = [
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
    0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
    3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
    4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

/// Given a first byte, determine how many bytes are in this UTF-8 character
#[inline]
pub fn utf8_char_width(b: u8) -> usize {
return UTF8_CHAR_WIDTH[b as usize] as usize;
}

fn read_one_byte(reader: &mut Read) -> Option<Result<u8>> {
let mut buf = [0];
loop {
    return match reader.read(&mut buf) {
        Ok(0) => None,
    Ok(..) => Some(Ok(buf[0])),
Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => Some(Err(e)),
        };
    }
}

pub struct Chars<'a> {
    inner: &'a mut Read,
}

pub fn from<'a>(r: &'a mut Read) -> Chars<'a> {
Chars {
    inner: r,
}
    }

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let first_byte = match read_one_byte(&mut self.inner) {
            None => return None,
            Some(Ok(b)) => b,
            Some(Err(e)) => panic!(e),
        };
        let width = utf8_char_width(first_byte);
        if width == 1 { return Some(first_byte as char) }
        if width == 0 { panic!("Something was not UTF8") }

        let mut buf = [first_byte, 0, 0, 0];

        {
            let mut start = 1;
            while start < width {
                match self.inner.read(&mut buf[start..width]) {
                    Ok(0) => panic!("Something was not UTF8"),
                Ok(n) => start += n,
                          Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => panic!(e),
    }
}
        }

        Some(match str::from_utf8(&buf[..width]).ok() {
            Some(s) => s.chars().next().unwrap(),
            None => panic!("Something was not UTF8"),
        })
    }
}
