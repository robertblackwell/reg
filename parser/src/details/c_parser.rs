//! This module provides a Rust callable binding to the  NodeJS http-parser library.

use ::{libc, Parser};
use std::mem::*;
use std::mem::size_of;
use super::c_types::ParserType;
use super::c_types::HttpMethod;
use super::c_functions::*;
use details::c_types::ParserError;

///
/// C_Parser is a Rust version of struct http_parser with some
/// Rust methods added to make it easier to manage.
/// The added Rust methods complete the transition to the Rust world
///
#[repr(C)]
pub struct C_Parser {
    /**
    * reserves space for http-parser bit fields
    * type:                 2 enum http_parser_type
    * flags:                8 enum flags from http_paser.h
    * state:                7 enum state from http_paser.c
    * header_state:         7 enum header_state from http_paser.c
    * index:                7
    * lenient_http_headers: 1
    *
    * should be confirmed for each release of http_parser
    */
    _internal_state: u32,


    _nread: u32,
    _content_length: u64,

    // Read-Only
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,

    /**
    * More bit fields
    *   status_code     16
    *   method          8
    *   http_errno      7
    *   upgrade         1
    *
    * should be confirmed for each release of http_parser
    */
    pub _extended_status: u32,

    // Public Interface
    pub data: *mut libc::c_void
}

unsafe impl Send for C_Parser { }

impl C_Parser {
    pub fn new(parser_type: ParserType) -> C_Parser {
        let mut p: C_Parser = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { http_parser_init(&mut p as *mut _, parser_type); }
        return p;
    }

    pub fn body_is_final(&self) -> bool
    {
        unsafe { return http_body_is_final(self) == 1; }
    }

    pub fn should_keep_alive(&self) -> bool
    {
        unsafe { return http_should_keep_alive(self) == 1; }
    }
    pub fn parser_is_upgrade(&self) -> bool
    {
        unsafe {
            return ex_http_parser_is_upgrade(self) == 1;
        }
    }
    pub fn parser_pause(&self, paused: libc::c_int)
    {
        unsafe { return http_parser_pause(self, paused); }
    }

    pub fn parser_errno(&self) -> ParserError
    {
        unsafe {
            return ParserError::from_value(ex_http_parser_errno(self) as u32);
        }
    }
    pub fn parser_errno_set(&mut self, err: ParserError)
    {
        unsafe{
            self._extended_status = ParserError::value(&err) as u32;
        }
    }
    pub fn parser_errno_name(&self) ->  &'static str {
        let errno = ParserError::value(&self.parser_errno());
        unsafe {
            let err_str = http_errno_name(errno);
            let buf = std::ffi::CStr::from_ptr(err_str);
            return std::str::from_utf8(buf.to_bytes()).unwrap();
        }
    }
    /// In case of a parsing error returns its mnemonic name.
    pub fn error_description(&self) -> &'static str {
        let errno = ParserError::value(&self.parser_errno());
        unsafe {
            let err_str = http_errno_description(errno);
            let buf = std::ffi::CStr::from_ptr(err_str);
            return std::str::from_utf8(buf.to_bytes()).unwrap();
        }
    }

    pub fn parser_method(&self) -> HttpMethod
    {
        unsafe{
            return HttpMethod::from_value(ex_http_parser_method(self as *const _) as u8);
        }
    }
    pub fn parser_method_set(&mut self, m: HttpMethod)
    {
        self._extended_status = HttpMethod::value(&m) as u32;
    }
    /// Returns an HTTP method static string (`GET`, `POST`, and so on).
    pub fn method_name(&self) -> &'static str {
        let method_code =  self.parser_method();
        unsafe {
            let methodstr = http_method_str(method_code as u8);
            let buf = std::ffi::CStr::from_ptr(methodstr);
            return std::str::from_utf8(buf.to_bytes()).unwrap();
        }
    }

    pub fn parser_status_code(&self) -> u16
    {
        unsafe{return ex_http_parser_status_code(self as *const _) as u16;};
    }
}

