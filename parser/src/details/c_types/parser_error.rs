//! This module provides a Rust callable binding to the  NodeJS http-parser library.

use libc;
use std::mem::*;
use std::mem::size_of;
use super::super::c_functions::{http_method_str, http_errno_description, http_errno_name};
use details::c_types::parser_error::ParserError::{HPE_EAGAIN, HPE_IOERR};


#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    HPE_OK,
    HPE_CB_message_begin,
    HPE_CB_url,
    HPE_CB_header_field,
    HPE_CB_header_value,
    HPE_CB_headers_complete,
    HPE_CB_body,
    HPE_CB_message_complete,
    HPE_CB_status,
    HPE_CB_chunk_header,
    HPE_CB_chunk_complete,
    HPE_INVALID_EOF_STATE,
    HPE_HEADER_OVERFLOW,
    HPE_CLOSED_CONNECTION,
    HPE_INVALID_VERSION,
    HPE_INVALID_STATUS,
    HPE_INVALID_METHOD,
    HPE_INVALID_URL,
    HPE_INVALID_HOST,
    HPE_INVALID_PORT,
    HPE_INVALID_PATH,
    HPE_INVALID_QUERY_STRING,
    HPE_INVALID_FRAGMENT,
    HPE_LF_EXPECTED,
    HPE_INVALID_HEADER_TOKEN,
    HPE_INVALID_CONTENT_LENGTH,
    HPE_UNEXPECTED_CONTENT_LENGTH,
    HPE_INVALID_CHUNK_SIZE,
    HPE_INVALID_CONSTANT,
    HPE_INVALID_INTERNAL_STATE,
    HPE_STRICT,
    HPE_PAUSED,
    HPE_UNKNOWN,
    HPE_IOERR,
    HPE_EAGAIN
}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
///
/// turn a general io error into one of two parser erros/
/// distinquis EAGAIN for future use of async io
///
impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> ParserError {
        if error.kind() == std::io::ErrorKind::WouldBlock {
            return ParserError::HPE_EAGAIN;
        } else {
            return ParserError::HPE_IOERR;
        }
    }
}
impl ParserError {
    pub fn value(&self) -> u8 {
        match self {
            ParserError::HPE_OK=> 0,
            ParserError::HPE_CB_message_begin=> 1,
            ParserError::HPE_CB_url=> 2,
            ParserError::HPE_CB_header_field=> 3,
            ParserError::HPE_CB_header_value=> 4,
            ParserError::HPE_CB_headers_complete=> 5,
            ParserError::HPE_CB_body=> 6,
            ParserError::HPE_CB_message_complete=> 7,
            ParserError::HPE_CB_status=> 8,
            ParserError::HPE_CB_chunk_header=> 9,
            ParserError::HPE_CB_chunk_complete=> 10,
            ParserError::HPE_INVALID_EOF_STATE=> 11,
            ParserError::HPE_HEADER_OVERFLOW=> 12,
            ParserError::HPE_CLOSED_CONNECTION=> 13,
            ParserError::HPE_INVALID_VERSION=> 14,
            ParserError::HPE_INVALID_STATUS=> 15,
            ParserError::HPE_INVALID_METHOD=> 16,
            ParserError::HPE_INVALID_URL=> 17,
            ParserError::HPE_INVALID_HOST=> 18,
            ParserError::HPE_INVALID_PORT=> 19,
            ParserError::HPE_INVALID_PATH=> 20,
            ParserError::HPE_INVALID_QUERY_STRING=> 21,
            ParserError::HPE_INVALID_FRAGMENT=> 22,
            ParserError::HPE_LF_EXPECTED=> 23,
            ParserError::HPE_INVALID_HEADER_TOKEN=> 24,
            ParserError::HPE_INVALID_CONTENT_LENGTH=> 25,
            ParserError::HPE_UNEXPECTED_CONTENT_LENGTH=> 26,
            ParserError::HPE_INVALID_CHUNK_SIZE=> 27,
            ParserError::HPE_INVALID_CONSTANT=> 28,
            ParserError::HPE_INVALID_INTERNAL_STATE=> 29,
            ParserError::HPE_STRICT=> 30,
            ParserError::HPE_PAUSED=> 31,
            ParserError::HPE_UNKNOWN=> 32,
            ParserError::HPE_EAGAIN => 99,
            ParserError::HPE_IOERR => 98
        }
    }
    pub fn from_value(value: u32) -> ParserError {
        match value {
            0 => ParserError::HPE_OK,
            1 => ParserError::HPE_CB_message_begin,
            2 => ParserError::HPE_CB_url,
            3 => ParserError::HPE_CB_header_field,
            4 => ParserError::HPE_CB_header_value,
            5 => ParserError::HPE_CB_headers_complete,
            6 => ParserError::HPE_CB_body,
            7 => ParserError::HPE_CB_message_complete,
            8 => ParserError::HPE_CB_status,
            9 => ParserError::HPE_CB_chunk_header,
            10 => ParserError::HPE_CB_chunk_complete,
            11 => ParserError::HPE_INVALID_EOF_STATE,
            12 => ParserError::HPE_HEADER_OVERFLOW,
            13 => ParserError::HPE_CLOSED_CONNECTION,
            14 => ParserError::HPE_INVALID_VERSION,
            15 => ParserError::HPE_INVALID_STATUS,
            16 => ParserError::HPE_INVALID_METHOD,
            17 => ParserError::HPE_INVALID_URL,
            18 => ParserError::HPE_INVALID_HOST,
            19 => ParserError::HPE_INVALID_PORT,
            20 => ParserError::HPE_INVALID_PATH,
            21 => ParserError::HPE_INVALID_QUERY_STRING,
            22 => ParserError::HPE_INVALID_FRAGMENT,
            23 => ParserError::HPE_LF_EXPECTED,
            24 => ParserError::HPE_INVALID_HEADER_TOKEN,
            25 => ParserError::HPE_INVALID_CONTENT_LENGTH,
            26 => ParserError::HPE_UNEXPECTED_CONTENT_LENGTH,
            27 => ParserError::HPE_INVALID_CHUNK_SIZE,
            28 => ParserError::HPE_INVALID_CONSTANT,
            29 => ParserError::HPE_INVALID_INTERNAL_STATE,
            30 => ParserError::HPE_STRICT,
            31 => ParserError::HPE_PAUSED,
            98 => ParserError::HPE_IOERR,
            99 => ParserError::HPE_EAGAIN,
            _ => ParserError::HPE_UNKNOWN,
        }
    }

    ///
    /// these two functions are redundant - as the Display trait automatically gives to_string()
    ///
    pub fn name(&self) -> &'static str {
        if *self == HPE_EAGAIN {
            return "EAGAIN=WOULD_BLOCK"
        }
        if *self == HPE_IOERR {
            return "IOERR";
        }
        unsafe {
            let name_str = http_errno_name(self.value());
            let buf = std::ffi::CStr::from_ptr(name_str);
            return std::str::from_utf8(buf.to_bytes()).unwrap();
        }
    }
    pub fn description(&self) -> &'static str {
        unsafe {
            let description_str = http_errno_description(self.value());
            let buf = std::ffi::CStr::from_ptr(description_str);
            return std::str::from_utf8(buf.to_bytes()).unwrap();
        }
    }
}


#[cfg(test)]
mod tests {

use super::*;
use std::io;

///
/// this set of functions mock up an error scenario to
/// ensure that ParserErrors and general io errors are correctly passed up
/// the call tree
///

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    NoError,
    ParserError,
    NoEnt,
    IoError,
    WouldBlockError,
}

    /// function to generate std::io::Errors
    pub fn f_ioerror(typ: Type) -> std::result::Result<u16, std::io::Error> {
        if typ == Type::WouldBlockError {
            // generate an io error ewouldblk
            let f = std::io::Error::from_raw_os_error(11); // wouldblock
            return Err(f);
        } else if typ == Type::IoError {
            // generate an io error enoent
            let f = std::io::Error::from_raw_os_error(2); //enoent
            return Err(f);
        } else if typ == Type::NoEnt {
            // lets try a real io error from std::fs
            let f = std::fs::File::open("afilethatdoesnotexist") ?;
        }

        return Ok(99)
    }

    /// utility function to generate both ParserErrors and/or std::io::Errors
    pub fn f_possible_error(n: Type) -> std::result::Result<u16, ParserError> {
        let mut r: u16 = 32;
        if n == Type::NoError {
            // no error
            r = 66;
        } else if n == Type::ParserError {
            // generate a banana error
            return Err(super::super::ParserError::HPE_CB_chunk_header);
        } else {
            f_ioerror(n)?;
        }
        Ok(r)
    }

    ///
    /// calls a function that may return an Ok result or std::io::Error or ParserErrors
    ///
    pub fn call_error_generating_function(n: Type) -> String {
        let m = pass_back_result(n);
        match m {
            Ok(m) => {
                println!("Ok");
                return "Ok".to_string();
            },
            Err(ref e) => {
                let xio = (*e == super::super::ParserError::HPE_IOERR);
                let xwblk = (*e == super::super::ParserError::HPE_EAGAIN);
                let desc = e.to_string();
                println!("Got error {}", e);
                return e.to_string();
            }
        }
        println!("pass_on_parent");
        return "should not get here".to_string();
    }
    ///
    /// call a function that MAY return an error, and the error may be std::io::Error or ParserError
    /// use the ? notation to quick return on error
    ///
    pub fn pass_back_result(n: Type) -> std::result::Result<u16, super::super::ParserError> {
        f_possible_error(n)?;
        Ok(99)
    }
    #[test]
    fn test_parser_errors_passon()
    {
        let s1 = call_error_generating_function(Type::WouldBlockError);
        let s2 = call_error_generating_function(Type::IoError);
        let s3 = call_error_generating_function(Type::ParserError);
        let s4 = call_error_generating_function(Type::NoEnt);

        assert!(s1 == "HPE_EAGAIN".to_string());
        assert!(s2 == "HPE_IOERR".to_string());
        assert!(s3 == "HPE_CB_chunk_header".to_string());
        assert!(s4 == "HPE_IOERR".to_string());

    }
    #[test]
    fn test_parser_error() {
        let pe_eagain = ParserError::HPE_EAGAIN;
        let pe_ioerr = ParserError::HPE_IOERR;
        let pe_else = ParserError::HPE_CB_header_field;
        let s1 = pe_eagain.to_string();
        let s2 = pe_ioerr.to_string();
        let s3 = pe_else.to_string();
        assert!(s3 == "HPE_CB_header_field".to_string());
    }

}