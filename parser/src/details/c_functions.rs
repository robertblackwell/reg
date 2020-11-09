//! This module provides a Rust callable binding to the  NodeJS http-parser library.

use libc;
use super::c_types::ParserType;
use super::c_parser::C_Parser;
use super::c_parser_settings::C_ParserSettings;

#[allow(dead_code)]
extern "C" {
    /** functions that operate on *http_parser */
    pub fn http_parser_init(parser: *mut C_Parser, parser_type: ParserType);
    pub fn http_parser_execute(parser: *mut C_Parser, settings: *const C_ParserSettings, data: *const u8, len: libc::size_t) -> libc::size_t;
    pub fn http_body_is_final(parser: *const C_Parser) -> libc::c_int;
    pub fn http_should_keep_alive(parser: *const C_Parser) -> libc::c_int;
    pub fn http_parser_pause(parser: *const C_Parser, paused: libc::c_int);

    // Helper provided by this package to predictably use aligned bit-field struct
    // see http_parser_ext.c
    pub fn http_get_struct_flags(parser: *const C_Parser) -> u32;

    /** acts on http_parser_settings*/
    pub fn http_parser_settings_init(settings: *mut C_ParserSettings);

    /** utility functions from http_parser*/
    pub fn http_parser_version() -> u32;
    pub fn http_method_str(method_code: u8) -> *const libc::c_char;
    pub fn http_errno_name(http_errno: u8) -> *const libc::c_char;
    pub fn http_errno_description(http_errno: u8) -> *const libc::c_char;

    // additional utility functions from c_extension.c
    pub fn ex_http_parser_struct_sizeof() -> u32;
    pub fn ex_http_parser_settings_struct_sizeof() -> u32;
    pub fn ex_http_parser_errno(parser: *const C_Parser) -> libc::c_int;
    pub fn ex_http_parser_method(parser: *const C_Parser) -> libc::c_int;
    pub fn ex_http_parser_status_code(parser: *const C_Parser) -> libc::c_int;
    pub fn ex_http_parser_is_upgrade(parser: *const C_Parser) -> libc::c_int;

    pub fn ex_http_parser_errno_set(parser: *mut C_Parser, errno: libc::c_int);
    pub fn ex_http_parser_method_set(parser: *mut C_Parser, m: libc::c_int);
    pub fn ex_http_parser_status_code_set(parser: *mut C_Parser, sc: libc::c_int);
    pub fn ex_http_parser_is_upgrade_set(parser: *mut C_Parser, sc: libc::c_int);

}

#[cfg(test)]
mod tests {
    use super::*;

    // test that ex_http functions that access method, errno and status_code work
    #[test]
    fn functions_t() {
        let mut c_struct = C_Parser::new(ParserType::HttpBoth);
        let mut mref = &mut c_struct;
        unsafe {let mut ptr: *mut C_Parser = mref;}
        let mut ptr: *mut C_Parser;
        unsafe {
            ptr = &mut c_struct;
            ex_http_parser_method_set(ptr, 32);
            ex_http_parser_errno_set(ptr, 23);
            ex_http_parser_status_code_set(ptr, 31);
            let mout = ex_http_parser_method(ptr);
            let errout = ex_http_parser_errno(ptr);
            let scout = ex_http_parser_status_code(ptr);
            println!("Done");
        }
    }
}
