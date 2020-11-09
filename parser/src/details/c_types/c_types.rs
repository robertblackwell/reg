//! This module provides a Rust callable binding to the  NodeJS http-parser library.

use libc;
use std::mem::*;
use std::mem::size_of;
use super::c_functions::{http_method_str, http_errno_description, http_errno_name};

///
/// This enum was implemented so that request handlers could perform a fast dispatch on http method
/// without resorting to a string compare
///
#[derive(Debug, PartialEq, Eq)]
enum HttpMethod {
    DELETE,
    GET,
    HEAD,
    POST,
    PUT,
    CONNECT,
    OPTIONS,
    TRACE,
    COPY,
    LOCK,
    MKCOL,
    MOVE,
    PROPFIND,
    PROPPATCH,
    SEARCH,
    UNLOCK,
    BIND,
    REBIND,
    UNBIND,
    ACL,
    REPORT,
    MKACTIVITY,
    CHECKOUT,
    MERGE,
    MSEARCH,
    NOTIFY,
    SUBSCRIBE,
    UNSUBSCRIBE,
    PATCH,
    PURGE,
    MKCALENDAR,
    LINK,
    UNLINK,
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

impl HttpMethod {
    ///
    ///This function matches the underlying value up with the constants defined by http_parser.h
    ///
    pub fn value(&self) -> u8 {
        match *self {
            HttpMethod::DELETE => 0,
            HttpMethod::GET =>  1,
            HttpMethod::HEAD =>  2,
            HttpMethod::POST =>  3,
            HttpMethod::PUT => 4,
            HttpMethod::CONNECT => 5,
            HttpMethod::OPTIONS =>  6,
            HttpMethod::TRACE =>  7,
            HttpMethod::COPY => 8,
            HttpMethod::LOCK => 9,
            HttpMethod::MKCOL => 10,
            HttpMethod::MOVE => 11,
            HttpMethod::PROPFIND =>12,
            HttpMethod::PROPPATCH =>13,
            HttpMethod::SEARCH =>14,
            HttpMethod::UNLOCK =>15,
            HttpMethod::BIND =>16,
            HttpMethod::REBIND =>17,
            HttpMethod::UNBIND =>18,
            HttpMethod::ACL =>19,
            HttpMethod::REPORT =>20,
            HttpMethod::MKACTIVITY =>21,
            HttpMethod::CHECKOUT =>22,
            HttpMethod::MERGE =>23,
            HttpMethod::MSEARCH =>24,
            HttpMethod::NOTIFY =>25,
            HttpMethod::SUBSCRIBE =>26,
            HttpMethod::UNSUBSCRIBE =>27,
            HttpMethod::PATCH =>28,
            HttpMethod::PURGE =>29,
            HttpMethod::MKCALENDAR =>30,
            HttpMethod::LINK => 31,
            HttpMethod::UNLINK => 32,
        }
    }
    fn name(&self) -> &'static str {
        unsafe {
            let method_str = http_method_str(self.value());
            let buf = std::ffi::CStr::from_ptr(method_str);
            return std::str::from_utf8(buf.to_bytes()).unwrap();
        }
    }

}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum ParserType {
    HttpRequest,
    HttpResponse,
    HttpBoth
}
impl ParserType {
    ///
    /// match the underlying value of the enum values with the
    /// values used in http_parser.h
    ///
    pub fn value(&self) -> u8 {
        match *self {
            ParserType::HttpRequest => 0,
            ParserType::HttpResponse => 1,
            ParserType::HttpBoth => 2,
        }
    }
    pub fn to_string(&self) -> String {
        match *self {
            ParserType::HttpRequest => "ParserType::HttpRequest".to_string(),
            ParserType::HttpResponse => "ParserType::HttpResponse".to_string(),
            ParserType::HttpBoth => "ParserType::HttpBoth".to_string()
        }
    }
    // fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    //     match *self {
    //         HeaderState::Nothing => write!(f, "HeaderState::Nothing"),
    //         HeaderState::Field => write!(f, "HeaderState::Field"),
    //         HeaderState::Value => write!(f, "HeaderState::Value")
    //     }
    // }
}
#[allow(non_upper_case_globals)]
mod parse_error {
    const XHPE_OK: u8 = 0;
    const XHPE_CB_message_begin: u8 = 1;
    const XHPE_CB_url: u8 = 2;
    const XHPE_CB_header_field: u8 = 3;
    const XHPE_CB_header_value: u8 = 4;
    const XHPE_CB_headers_complete: u8 = 5;
    const XHPE_CB_body: u8 = 6;
    const XHPE_CB_message_complete: u8 = 7;
    const XHPE_CB_status: u8 = 8;
    const XHPE_CB_chunk_header: u8 = 9;
    const XHPE_CB_chunk_complete: u8 = 10;
    const XHPE_INVALID_EOF_STATE: u8 = 11;
    const XHPE_HEADER_OVERFLOW: u8 = 12;
    const XHPE_CLOSED_CONNECTION: u8 = 13;
    const XHPE_INVALID_VERSION: u8 = 14;
    const XHPE_INVALID_STATUS: u8 = 15;
    const XHPE_INVALID_METHOD: u8 = 16;
    const XHPE_INVALID_URL: u8 = 17;
    const XHPE_INVALID_HOST: u8 = 18;
    const XHPE_INVALID_PORT: u8 = 19;
    const XHPE_INVALID_PATH: u8 = 20;
    const XHPE_INVALID_QUERY_STRING: u8 = 21;
    const XHPE_INVALID_FRAGMENT: u8 = 22;
    const XHPE_LF_EXPECTED: u8 = 23;
    const XHPE_INVALID_HEADER_TOKEN: u8 = 24;
    const XHPE_INVALID_CONTENT_LENGTH: u8 = 25;
    const XHPE_UNEXPECTED_CONTENT_LENGTH: u8 = 26;
    const XHPE_INVALID_CHUNK_SIZE: u8 = 27;
    const XHPE_INVALID_CONSTANT: u8 = 28;
    const XHPE_INVALID_INTERNAL_STATE: u8 = 29;
    const XHPE_STRICT: u8 = 30;
    const XHPE_PAUSED: u8 = 31;
    const XHPE_UNKNOWN: u8 = 32;

    // pub fn name(errno: u64) -> &'static str {
    //     unsafe {
    //         let name_str = http_errno_name(self.value());
    //         let buf = std::ffi::CStr::from_ptr(name_str);
    //         return std::str::from_utf8(buf.to_bytes()).unwrap();
    //     }
    // }
    // pub fn description(errno: u64) -> &'static str {
    //     unsafe {
    //         let description_str = http_errno_description(self.value());
    //         let buf = std::ffi::CStr::from_ptr(description_str);
    //         return std::str::from_utf8(buf.to_bytes()).unwrap();
    //     }
    // }

}
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
}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
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
            _ => ParserError::HPE_UNKNOWN,
        }
    }

    ///
    /// these two functions are redundant - as the Display trait automatically gives to_string()
    ///
    pub fn name(&self) -> &'static str {
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
    use std::mem::size_of;

    #[test]
    fn c_types_method_enum() {
        let x = HttpMethod::CHECKOUT;
        let y = x.value();
        let z = x.name();
        assert!(y == 22);
        if x != HttpMethod::POST {
            assert!(true);
            println!("Should be here");
        } else {
            assert!(false);
            println!("Should not be here");
        }

    }

    #[test]
    fn test_method_tostring() {
        let m = HttpMethod::LINK;
        let s = m.to_string();
        println!("{}", s);
    }
}