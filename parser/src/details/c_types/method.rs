//! This module provides enum HttpMethod

use libc;
use std::mem::*;
use std::mem::size_of;
use super::super::c_functions::{http_method_str, http_errno_description, http_errno_name};

///
/// This enum was implemented so that request handlers could perform a fast dispatch on http method
/// without resorting to a string compare
///
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
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
    pub fn from_value(value: u8) -> HttpMethod {
        match value {
            0 => HttpMethod::DELETE,
            1 => HttpMethod::GET,
            2 => HttpMethod::HEAD,
            3 => HttpMethod::POST,
            4 => HttpMethod::PUT,
            5 => HttpMethod::CONNECT,
            6 => HttpMethod::OPTIONS,
            7 => HttpMethod::TRACE,
            8 => HttpMethod::COPY,
            9 => HttpMethod::LOCK,
            10 => HttpMethod::MKCOL,
            11 => HttpMethod::MOVE,
            12 => HttpMethod::PROPFIND,
            13 => HttpMethod::PROPPATCH,
            14 => HttpMethod::SEARCH,
            15 => HttpMethod::UNLOCK,
            16 => HttpMethod::BIND,
            17 => HttpMethod::REBIND,
            18 => HttpMethod::UNBIND,
            19 => HttpMethod::ACL,
            20 => HttpMethod::REPORT,
            21 => HttpMethod::MKACTIVITY,
            22 => HttpMethod::CHECKOUT,
            23 => HttpMethod::MERGE,
            24 => HttpMethod::MSEARCH,
            25 => HttpMethod::NOTIFY,
            26 => HttpMethod::SUBSCRIBE,
            27 => HttpMethod::UNSUBSCRIBE,
            28 => HttpMethod::PATCH,
            29 => HttpMethod::PURGE,
            30 => HttpMethod::MKCALENDAR,
            31 => HttpMethod::LINK,
            _ => HttpMethod::UNLINK,
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