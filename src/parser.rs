use std::collections::HashMap;
use std::{path::Path, fs::File};
use std::io::Read;

use crate::builder::EzyKeyValuePairBuilder;
use crate::error::EzyErr;
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
const LAST_UPDATED: u8 = 0x0F;

fn is_consicutive_nulls(s: &mut u8) -> bool {
    if *s == 2 {
        *s = 0;
        return true;
    }else {
        *s += 1;
        return false;
    }
}


pub fn parse_from_file(path: &Path) -> Result<EzyKeyValuePair, EzyErr> {
    let file;
    match File::open(path){
        Ok(value) => file = value,
        Err(err) => return Err(EzyErr::IoError(err))
    }
    let mut buffer = Vec::new();
    let mut builder = EzyKeyValuePairBuilder::new();
    let mut null_byte_count: u8 = 0;
    for result_byte in file.bytes() {
        match result_byte {
            Ok(byte) => {
                if is_consicutive_nulls(&mut null_byte_count) {
                    builder.parse(&mut buffer);
                }
                else {
                    buffer.push(byte);
                }
            },
            Err(err) => return Err(EzyErr::IoError(err)),
        }
    }
    Ok(builder.build())
}

