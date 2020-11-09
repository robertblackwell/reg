#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate libc;



use std::assert;
use std::marker::Send;
use std::{fmt, str, option};
use std::fmt::{Display, Formatter, Result};
use std::vec::Vec;
use details::c_types::HttpMethod;

///
/// HeaderPair captures a single http header line with a header name and a header value.
///     -   Header names are required to be encoded in US_ASCII
///     -   Header values are generally in US-ASCII but there is provision in the Http standard for some use of
///         non ascii character sets.
///
///         For the purposes of this learning exercise we will ignore this latter possibility and assume:
///             all valid header names and values are UTF8 encoded
///
#[derive(Debug)]
pub struct HeaderPair {
    pub key: String,
    pub value: String,
}
impl HeaderPair {
    pub fn new(akey: &str, avalue: &str)-> HeaderPair {
        HeaderPair {
            key: akey.clone().to_string(),
            value: avalue.clone().to_string(),
        }
    }
    /// adds an additional value to self.value and self.value_str
    pub fn append_value(&mut self, extra_value: &str) {
        self.value.push_str(&(", ".to_string()));
        self.value.push_str(extra_value);
    }
}
///
/// HttpHeaders is a single structure that implements the set of headers that are part of
/// a http message.
/// There are a couple of requirements of http headers that complicate this data structure.
/// 1.  the name of a http_header is case insensitive. For convenience all headers names are converted to
///     ASCII caps
/// 2.  http headers are sensitive to order of transmission and hence arrival order must be maintained.
/// 3.  multiple headers with the same name are permitted and in such a circumstances the value field
///     of all headers (with a given name) should be appended to the value field of first header with
///     that name, separated by a ','.
#[derive(Debug)]
pub struct HttpHeaders {
    hvec: Vec<HeaderPair>,
}
impl HttpHeaders {
    pub fn new() ->HttpHeaders {
        HttpHeaders {
            hvec: Vec::new()
        }
    }
    pub fn find_by_key(&mut self, akey: &str) -> Option<&mut HeaderPair> {
        for hp in &mut self.hvec {
            // let s1 = hp.key_str.to_ascii_lowercase();
            if hp.key == akey {
                return Some(hp);
            }
        }
        None
    }
    pub fn add(&mut self, header_pair: HeaderPair) {
        let h = self.find_by_key(&header_pair.key);
        if let Some(hp) = h {
            let s = header_pair.key;
            hp.append_value(&s);
        } else {
            self.hvec.push(header_pair);
        }
    }
}
#[derive(Debug)]
pub struct HttpMessage {
    pub version: (u16, u16),
    pub method: String,
    pub method_enum: HttpMethod,
    pub target: String,
    pub status_code: u16,
    pub reason: String,
    pub headers: HttpHeaders,
    pub body: Vec<u8>,
    pub is_upgrade: bool,
    pub should_keep_alive: bool,
}

impl HttpMessage {
    pub fn new() -> HttpMessage {
        HttpMessage{
            version:(1,1),
            method: "GET".to_string(),
            method_enum: HttpMethod::GET,
            target: "".to_string(),
            status_code: 0,
            reason: "".to_string(),
            headers: HttpHeaders::new(),
            body: Vec::new(),
            is_upgrade: false,
            should_keep_alive: false,

        }
    }
    pub fn set_version(&mut self, major: u16, minor: u16) {
        self.version = (major, minor);
    }
    pub fn set_status_code(&mut self, status_code: u16) {
        self.status_code = status_code;
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method.clone();
    }
    pub fn get_method(&self) ->String {return self.method.clone();}

    pub fn set_target(&mut self, target: String) {
        self.target = target.clone();
    }
    pub fn get_target(&mut self) ->String {return self.target.clone();}

    pub fn set_reason(&mut self, reason: String) {
        self.reason = reason.clone();
    }

    pub fn add_header(&mut self, akey: &str, avalue: &str) {
        self.headers.add(HeaderPair::new(akey, avalue));
    }
    pub fn set_is_upgrade(&mut self, yn: bool) {
        self.is_upgrade = yn;
    }
    pub fn set_should_keep_alive(&mut self, yn: bool) {
        self.should_keep_alive = yn;
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_pair_new() {
        println!("test_header_pair");
        let k = &("Akey".to_string());
        let v = &("Avalue".to_string());
        let hp = super::HeaderPair::new(k, v);
        assert!(hp.key == "Akey".to_string());
        assert!(hp.value == "Avalue".to_string());
        println!("Done");
    }
    #[test]
    fn test_header_pair_append_value() {
        println!("test_header_pair");
        let k = &("Akey".to_string());
        let v = &("Avalue".to_string());
        let x = &("Extra".to_string());
        let mut hp = super::HeaderPair::new(k, v);
        hp.append_value(&("Extra".to_string()));
        assert!(hp.key == "Akey".to_string());
        assert!(hp.value == "Avalue, Extra".to_string());
        println!("Done");
    }
    #[test]
    fn test_headers_new_add() {
        let mut hdrs = HttpHeaders::new();
        let k = &("Akey".to_string());
        let v = &("Avalue".to_string());
        let hp = super::HeaderPair::new(k, v);
        hdrs.add(hp);
        let x = hdrs.find_by_key(&("Akey".to_string()));
        if let Some(hp) = x {
            assert!(hp.value == "Avalue".to_string());
        } else {
            assert!(false, "Should be Some");
        }
        let x2 = hdrs.find_by_key(&("Wrongkey".to_string()));
        if let Some(hp2) = x2 {
            assert!(false, "Should not have found");
        } else {
            println!("Did not find it");
            assert!(true, "Should be Some");
        }
        println!("Done");
    }
    #[test]
    fn test_message() {
        let mut msg = HttpMessage::new();
        msg.set_version(2,3);
        msg.set_target("/atarget/subdir".to_string());
        msg.set_reason("System error".to_string());
        msg.set_method("POST".to_string());
        msg.set_status_code(37);
        let ak = "Akey".to_string();
        let av = "Avalue".to_string();
        msg.add_header(&ak, &av);
        let x = msg.headers.find_by_key(&ak);
        if let Some(h) = x {
            assert!(h.key == ak);
        } else {
            assert!(false);
        }
        assert!(msg.version.0 == 2);
        assert!(msg.version.1 == 3);
        assert!(msg.target == "/atarget/subdir".to_string());
        assert!(msg.reason == ("System error".to_string()));
        assert!(msg.method == ("POST".to_string()));
        assert!(msg.status_code == 37);
    }
}
