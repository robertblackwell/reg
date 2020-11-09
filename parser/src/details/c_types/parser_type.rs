//! This module provides a Rust callable binding to the  NodeJS http-parser library.

use libc;
use std::mem::*;
use std::mem::size_of;
use super::super::c_functions::{http_method_str, http_errno_description, http_errno_name};
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserType {
    HttpRequest,
    HttpResponse,
    HttpBoth
}
impl std::fmt::Display for ParserType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser_type() {
        let pt = ParserType::HttpResponse;
        let s = pt.to_string();
        println!("{}", s);
    }

}