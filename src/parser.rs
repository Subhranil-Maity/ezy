use std::collections::HashMap;
use std::{path::Path, fs::File};
use std::io::{Error, Read};

use crate::{EzyKeyValuePair, EzyValue};


const SIGNED_INTEGER_8: u8 = 0x01;
const SIGNED_INTEGER_16: u8 = 0x02;
const SIGNED_INTEGER_32: u8 = 0x03;
const SIGNED_INTEGER_64: u8 = 0x04;
const SIGNED_INTEGER_128: u8 = 0x05;
const UNSIGNED_INTEGER_8: u8 = 0x06;
const UNSIGNED_INTEGER_16: u8 = 0x07;
const UNSIGNED_INTEGER_32: u8 = 0x08;
const UNSIGNED_INTEGER_64: u8 = 0x09;
const UNSIGNED_INTEGER_128: u8 = 0x0A;
const FLOAT_32: u8 = 0x0B;
const FLOAT_64: u8 = 0x0C;
const BOOL: u8 = 0x0D;
const STRING: u8 = 0x0E;


pub fn parse_from_file(path: &Path) -> Result<EzyKeyValuePair, Error> {
    let file = File::open(path)?;
    let mut buffer = Vec::new();
    let mut pair = HashMap::<String, EzyValue>::new();
    for result_byte in file.bytes() {
        match result_byte {
            Ok(byte) => {
                if byte == 0 {
                    parse_ezy(&mut buffer, &mut pair);
                }
                else {
                    buffer.push(byte);
                }
            },
            Err(err) => return Err(err),
        }
    }
    Ok(EzyKeyValuePair { pairs: pair, version: String::from(""), last_updated: 111 })
}

fn parse_ezy(buffer: &mut Vec<u8>, pair: &mut HashMap<String, EzyValue>) {
    
}

