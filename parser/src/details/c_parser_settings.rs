
use libc;
use std::mem::*;
use std::mem::size_of;
use details::c_types::ParserType;
use super::c_parser::C_Parser;

pub type HttpCallback = extern fn(*mut C_Parser) -> libc::c_int;
pub type HttpDataCallback = extern fn(*mut C_Parser, *const u32, libc::size_t) -> libc::c_int;

///
/// This struct is a Rust version of  http_parser_settings
///
/// The impl for this struct is in parser.rs
///
#[repr(C)]
pub struct C_ParserSettings {
    pub on_message_begin:    HttpCallback,
    pub on_url:              HttpDataCallback,
    pub on_status:           HttpDataCallback,
    pub on_header_field:     HttpDataCallback,
    pub on_header_value:     HttpDataCallback,
    pub on_headers_complete: HttpCallback,
    pub on_body:             HttpDataCallback,
    pub on_message_complete: HttpCallback,
    pub on_chunk_header:     HttpCallback,
    pub on_chunk_complete:   HttpCallback
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use super::super::c_parser::C_Parser;
    use super::super::c_functions::ex_http_parser_settings_struct_sizeof;
    use super::super::c_functions::ex_http_parser_struct_sizeof;

    ///
    /// simple check that the size of the two important c struct (http_parser and http_parser_settings)
    /// are the same as their Rust equivalent
    ///
    #[test]
    fn method_sizof_c_parser_settings() {
        let sizeof_rust_parser = size_of::<C_Parser>();
        let sizeof_rust_settings = size_of::<C_ParserSettings>();
        let c_parser_struct_sizeof: usize;
        let c_settings_sizeof: usize;
        unsafe {
            c_parser_struct_sizeof = ex_http_parser_struct_sizeof() as usize;
            c_settings_sizeof = ex_http_parser_settings_struct_sizeof() as usize;
        }
        assert!(sizeof_rust_parser == c_parser_struct_sizeof);
        assert!(sizeof_rust_settings == c_settings_sizeof);
    }
}