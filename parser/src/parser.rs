#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_comparisons)]
extern crate libc;

use std::{str, option, assert, marker::Send};
use std::fmt::{Display, Formatter, Result};
use std::vec::Vec;

use http_message::{HeaderPair, HttpHeaders, HttpMessage};

use details::c_parser::C_Parser;
use details::c_parser_settings::C_ParserSettings;
use details::c_functions::*;
use details::c_types::*;

#[inline]
unsafe fn unwrap_parser<'a>(http: *mut C_Parser) -> &'a mut Parser<'a> {
    &mut *((*http).data as *mut Parser)
}

#[derive(Debug, PartialEq, Eq)]
enum HeaderState {
    Nothing,
    Value,
    Field,
}

impl Display for HeaderState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            HeaderState::Nothing => write!(f, "HeaderState::Nothing"),
            HeaderState::Field => write!(f, "HeaderState::Field"),
            HeaderState::Value => write!(f, "HeaderState::Value")
        }
    }
}

pub struct ParserReturnValue {
    ///
    /// number of bytes not consumed
    ///
    pub not_consumed: usize,
    ///
    /// end of message
    ///
    pub eom: bool,
    ///
    /// was there an error
    ///
    pub error: bool,
    ///
    ///
    ///
    pub parser_error: ParserError,
}

#[allow(dead_code)]
pub struct Parser<'a> {
    c_parser_struct: C_Parser,
    c_settings_struct: C_ParserSettings,
    parser_type: ParserType,
    flags: u32,
    header_state: HeaderState,
    target: Vec<u8>,

    reason: Vec<u8>,
    status_code: u32,
    headers: Vec<HeaderPair>,
    body: Vec<u8>,
    header_key: String,
    header_value: String,
    pub message: &'a mut HttpMessage,
    pub started: bool,
    pub message_done: bool,
    signature: String,
}

unsafe impl Send for Parser<'_> {}

impl<'a> Parser<'a> {
    /// Creates a new parser instance for an HTTP response.
    pub fn response(message: &mut HttpMessage) -> Parser<'_> {
        let p = Parser::new(ParserType::HttpResponse, message);
        return p;
    }

    /// Creates a new parser instance for an HTTP request.
    pub fn request(message: &mut HttpMessage) -> Parser<'_> {
        let p = Parser::new(ParserType::HttpRequest, message);
        return p;
    }
    /// Creates a new parser instance to handle both HTTP requests and responses.
    pub fn request_and_response(message: &mut HttpMessage) -> Parser<'_> {
        let p = Parser::new(ParserType::HttpBoth, message);
        return p;
    }
    pub fn new(ptype: ParserType, message: &mut HttpMessage) -> Parser {
        let mut p = Parser {
            parser_type: ParserType::HttpBoth,
            c_parser_struct: C_Parser::new(ptype),
            // c_settings_struct: HttpParserSettings::<Parser>::new(),
            c_settings_struct: C_ParserSettings {
                on_url: on_url_wrapper,
                on_message_begin: on_message_begin_wrapper,
                on_status: on_status_wrapper,
                on_header_field: on_header_field_wrapper,
                on_header_value: on_header_value_wrapper,
                on_headers_complete: on_headers_complete_wrapper,
                on_body: on_body_wrapper,
                on_message_complete: on_message_complete_wrapper,
                on_chunk_header: on_chunk_header_wrapper,
                on_chunk_complete: on_chunk_complete_wrapper,
            },
            flags: 0,
            header_state: HeaderState::Nothing,
            target: Vec::with_capacity(100),
            reason: Vec::with_capacity(100),
            header_key: "".to_string(),
            header_value: "".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
            status_code: 0,
            message: message,
            started: false,
            message_done: false,
            signature: ptype.to_string(),
        };
        p.c_parser_struct.data = &mut p as *mut _ as *mut libc::c_void;
        return p;

    }
    /// Parses the provided `data` and returns a number of bytes read.
    pub fn parse(&mut self, data: &[u8]) -> usize {
        let c_parser_ptr = &mut self.c_parser_struct as *mut _;
        let c_parser_settings_ptr = &mut self.c_settings_struct as *const _;
        let size: usize;
        unsafe {
            size = http_parser_execute(c_parser_ptr,
                                       c_parser_settings_ptr,
                                       data.as_ptr(),
                                       data.len() as libc::size_t) as usize;
        };
        unsafe {
            self.flags = http_get_struct_flags(&self.c_parser_struct as *const _);
        }
        size
    }
    ///
    /// determines if http_parser is in error (error = true && error = PAUSED) is not treated as an error
    /// as this is the way we pause the parsing at eom
    ///
    pub fn is_error(&mut self) -> bool {
        let c_parser_ptr = &mut self.c_parser_struct as *mut _;
        let c_parser_settings_ptr = &mut self.c_settings_struct as *const _;
        let errno = self.errno();
        let ss = errno.to_string();
        // HPE_CB_message_complete is rased because we terminated the parsing from within on_message_complete
        // so that is not an error
        return (errno != ParserError::HPE_OK) && (errno != ParserError::HPE_PAUSED) && (errno != ParserError::HPE_CB_message_complete);
    }

    ///
    /// consume is a wrapper for http_parser_execute intended to provide a more convenient
    /// synchronous interface.
    ///
    /// Its primary purpose is to ensure that it returns as soon as message_complete is detected
    /// to facilitate synchronous reading of complete messages.
    ///
    /// It should be noted that http_parser_execute DOES not have this characteristic in the event that http message pipelining
    /// results in some of the end bytes of one message and some of the initial bytes of the next message are in the same buffer.
    ///
    /// This is an unlikely, but not impossible situation. The http/1.1 spec still includes pipelinign.
    /// Most modern browsers have the ability to perform pipelining of http/1.1  requests, though most have
    /// it disabled by default. All servers should be able to handle http/1.1 pipelining.
    ///
    /// http_parser_execute() handles this issue by announcing the completion of a message via the on_message_complete() callback,
    /// but not by returning.
    ///
    /// This is an inconvenient (unworkable ?) solution for a synchronous reader, so the goal is to wrapp http_parser_execute()
    /// with consume() such that consume() returns after each full message regadless of whether the current buffer has been exhausted.
    ///
    /// More specifically, consume() will return as follows:
    /// 1. consumed all the bytes fed in (return value 0) and message complete is false
    /// 2. consumed all the bytes fed in (return value 0) and message complete has become true during this buffer
    ///     This is the situation that arises when the most recent buffer completed a message
    ///     and did not start the next message.
    ///     It should be noted that http pipelining allows the end of one message and start of the next
    ///     message to be contained in the same TCP buffer.
    /// 3.  did not consume all the bytes fed in, there are two possibilities
    ///     a) as described in the http_parser docs this will be the situation when a parsing error
    ///         is detected.
    ///     b) in this application of http_parser the on_message_complete() callback returns 1 to terminate/pause the parser
    ///         when message_complete is detected. As a consequence in that the current buffer contains the completion of
    ///         one message and the start of the next message. Thus http_paser_execute will return a non zero value
    ///         without having detected a parsing error.
    ///     The way these two situations are differentiated is by the message_complete  value:
    ///         a) return value != 0 and ! message_complete means a parsing error
    ///         b) return value != 0 and message_complete means one message is complete the the residual of the
    ///            buffer contains the start of the next message.
    ///         In case b) the residual buffer must be retained and re-presented to http_parser_execute
    ///         again before other input bytes so that the start of the new message gets correctly parsed.
    ///
    /// Consume() clarifies these various situations via the nature of its return type.
    ///
    ///
    pub fn consume(&mut self, data: &[u8]) -> ParserReturnValue {
        self.started = true;
        let mut rv = ParserReturnValue {
            not_consumed: 0,
            eom: false,
            error: false,
            parser_error: ParserError::HPE_OK,
        };
        let length = data.len();
        let mut total_parsed: usize = 0;
        let nparsed = self.parse(data);
        total_parsed = total_parsed + nparsed;

        rv.not_consumed = length - nparsed;
        if self.is_error() {
            rv.error = true;
            rv.parser_error = self.errno();
        } else if self.message_done {
            rv.eom = true;
        } else if nparsed == length {
            // rv already setup
        }
        return rv;
    }

    /// Returns an HTTP request or response version.
    pub fn http_version(&self) -> (u16, u16) {
        (self.c_parser_struct.http_major, self.c_parser_struct.http_minor)
    }

    /// Returns an HTTP response status code (think *404*) of the http message currently being parsed.
    pub fn status_code(&self) -> u16 {
        return self.c_parser_struct.parser_status_code();
    }

    /// returns the httm method (think GET) of the http message currently being parsed.
    pub fn method(&self) -> HttpMethod {
        return self.c_parser_struct.parser_method();
    }
    /// Returns an HTTP method static string (`GET`, `POST`, and so on).
    pub fn method_name(&self) -> &'static str {
        return self.c_parser_struct.method_name();
    }
    /// Returns the ParserError value for the Parser
    fn errno(&self) -> ParserError {
        return self.c_parser_struct.parser_errno();
    }
    /// Returns the short name of the current parsers error.
    pub fn error_description(&self) -> &'static str {
        return self.c_parser_struct.error_description();
    }
    /// Returns the description of the current parsers error.
    pub fn error_name(&mut self) -> &'static str
    {
        return self.c_parser_struct.parser_errno_name();
    }
    /// Checks if the current message being parsed is an upgrade request (e.g. WebSocket).
    pub fn is_upgrade(&self) -> bool
    {
        return self.c_parser_struct.parser_is_upgrade();
    }
    /// Checks if it was the final body chunk.
    pub fn is_final_chunk(&self) -> bool {
        return self.c_parser_struct.body_is_final();
    }

    ///
    /// Indicates whether keep_alive should be active on the connection.
    /// http-parser applies the standards rules.
    /// http/1.0 defaults to connection: close.
    /// http/1.1 defaults to keep_alive
    /// both can be overriden by the "Connection: ????" header
    ///
    pub fn should_keep_alive(&self) -> bool {
        self.c_parser_struct.should_keep_alive()
    }

    pub fn pause(&mut self) {
        self.c_parser_struct.parser_pause(1);
    }

    pub fn unpause(&mut self) {
        self.c_parser_struct.parser_pause(0);
    }
    ///
    /// callbacks
    ///
    fn on_url(&mut self, slice: &[u8]) -> bool {
        println!("on_url before target: [{}] slice: {}", String::from_utf8_lossy(&self.target), str::from_utf8(slice).unwrap());
        self.target.extend(slice.iter().clone());
        self.message.set_target(String::from_utf8(self.target.clone()).unwrap());
        true
    }
    fn on_status(&mut self, slice: &[u8]) -> bool {
        println!("on_status");
        self.reason.extend(slice.iter().clone());
        self.message.set_reason(String::from_utf8(self.reason.clone()).unwrap());
        true
    }
    fn on_header_field(&mut self, slice: &[u8]) -> bool {
        println!("on_header_field header-state: {} before header key: [{}] slice: {}", self.header_state, &self.header_key, str::from_utf8(slice).unwrap());
        if self.header_state == HeaderState::Nothing || self.header_state == HeaderState::Value {
            if self.header_key.len() != 0 {
                assert!(self.header_value.len() != 0, "header_key.len() != 0 and header_value.len() == 0");
                self.headers.push(HeaderPair::new(&self.header_key, &self.header_value));
                self.message.add_header(&self.header_key, &self.header_value);
            }
            self.header_state = HeaderState::Field;
            self.header_key.clear();
            let b = std::str::from_utf8(slice).unwrap().to_string();
            self.header_key.push_str(&b);
        } else {
            let b = std::str::from_utf8(slice).unwrap().to_string();
            self.header_key.push_str(&b);
        }
        let ss = self.header_key.clone();
        println!("on header key: {:?}", ss);
        true
    }
    fn on_header_value(&mut self, slice: &[u8]) -> bool {
        println!("on_header_value header-state: {} before header value: [{}] value: {}", self.header_state, &self.header_value, str::from_utf8(slice).unwrap());
        if self.header_state == HeaderState::Nothing || self.header_state == HeaderState::Field {
            self.header_state = HeaderState::Value;
            self.header_value.clear();
            self.header_value.push_str(&(str::from_utf8(slice).unwrap().to_string()));
        } else {
            self.header_value.push_str(&(str::from_utf8(slice).unwrap().to_string()));
        }
        let ss = self.header_value.clone();
        println!("on header value: {:?}", ss);
        true
    }
    fn on_body(&mut self, slice: &[u8]) -> bool {
        println!("on_body before body: [{}] slice: {}", String::from_utf8_lossy(&self.body), str::from_utf8(slice).unwrap());
        self.body.extend(slice.iter().clone());
        true
    }
    fn on_headers_complete(&mut self) -> bool {
        println!("on_headers_complete");
        //
        // The last header key/value pair will still be in the parsers buffers when headers_complete is detected
        // so make sure to stash that header away
        //
        self.headers.push(HeaderPair::new(&self.header_key, &self.header_value));
        self.message.add_header(&self.header_key, &self.header_value);

        self.message.method_enum = self.method();
        self.message.set_method(self.method_name().to_string());
        self.message.set_status_code(self.status_code());
        self.message.set_is_upgrade(self.is_upgrade());
        self.message.set_should_keep_alive(self.should_keep_alive());
        true
    }
    fn on_message_begin(&mut self) -> bool {
        println!("on_message_begin");
        true
    }
    fn on_message_complete(&mut self) -> bool {
        println!("on_message_complete");
        self.message_done = true;
        false
    }
    fn on_chunk_header(&mut self) -> bool {
        println!("on_chunk_header");
        true
    }
    fn on_chunk_complete(&mut self) -> bool {
        println!("on_chunk_complete");
        true
    }
}

// impl std::fmt::Debug for Parser {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         let (version_major, version_minor) = self.http_version();
//         return write!(fmt,
//                       "status_code: {}\nmethod: {}\nerror: {}, {}\nupgrade: {}\nhttp_version: \
//                        {}.{}",
//                       self.status_code(),
//                       self.http_method(),
//                       self.error(),
//                       self.error_description(),
//                       self.is_upgrade(),
//                       version_major,
//                       version_minor);
//     }
// }

/// Returns a version of the underlying `http-parser` library.
pub fn version() -> (u32, u32, u32) {
    let version = unsafe { http_parser_version() };

    let major = (version >> 16) & 255;
    let minor = (version >> 8) & 255;
    let patch = version & 255;

    (major, minor, patch)
}

/**
* The following 10 functions are Rust callback functions called by http_parser.c. There are two
* categories of callbacks
*   1. Notify Callbacks of Rust type HttpCallBack (see ffi.rs)
*    on_message_begin_wrapper
*    on_message_complete_wrapper
*    on_headers_complete_wrapper
*
*   1. Data Callbacks of Rust type HttpDataCallBack (see ffi.rs). These functions are passed a memory buffer and length
*       containing a (possibly partial) parse result for url, status, headers, body or chunk.
*    on_url_wrapper
*    on_status_wrapper
*    on_header_field_wrapper
*    on_header_value_wrapper
*    on_body_wrapper
*    on_chunk_header_wrapper
*    on_chunk_complete_wrapper
*
* Each of these functions passes the call onto the appropriate method in an implementation of ParserHandler.
*
* WARNING: http_parser.c expects the callbacks to return 0 on success which will allow parsing to continue
* and non-zero to pause parsing. Whereas the handler methods return a boolean
* for which true means success+continue and false means pause or error; this is effectively the reverse of
* the C interface.
*
*
*/
extern "C" fn on_message_begin_wrapper(http: *mut C_Parser) -> libc::c_int {
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_message_begin() {
        0
    } else {
        1
    }
}

extern "C" fn on_url_wrapper(http: *mut C_Parser, data: *const u32, size: libc::size_t) -> libc::c_int {
    let slice = unsafe {
        std::slice::from_raw_parts(data as *const u8, size as usize)
    };

    let parser = unsafe {
        unwrap_parser(http)
    };

    if parser.on_url(slice) {
        0
    } else {
        1
    }
}

extern "C" fn on_status_wrapper(http: *mut C_Parser, data: *const u32, size: libc::size_t) -> libc::c_int {
    let slice = unsafe {
        std::slice::from_raw_parts(data as *const u8, size as usize)
    };
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_status(slice) {
        0
    } else {
        1
    }
}

extern "C" fn on_header_field_wrapper(http: *mut C_Parser, data: *const u32, size: libc::size_t) -> libc::c_int {
    let slice =
        unsafe {
            std::slice::from_raw_parts(data as *const u8, size as usize)
        };
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_header_field(slice) {
        0
    } else {
        1
    }
}

extern "C" fn on_header_value_wrapper(http: *mut C_Parser, data: *const u32, size: libc::size_t) -> libc::c_int {
    let slice = unsafe {
        std::slice::from_raw_parts(data as *const u8, size as usize)
    };
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_header_value(slice) {
        0
    } else {
        1
    }
}

extern "C" fn on_headers_complete_wrapper(http: *mut C_Parser) -> libc::c_int {
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_headers_complete() {
        0
    } else {
        1
    }
}

extern "C" fn on_body_wrapper(http: *mut C_Parser, data: *const u32, size: libc::size_t) -> libc::c_int {
    let slice = unsafe {
        std::slice::from_raw_parts(data as *const u8, size as usize)
    };
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_body(slice) {
        0
    } else {
        1
    }
}

extern "C" fn on_message_complete_wrapper(http: *mut C_Parser) -> libc::c_int {
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_message_complete() {
        0
    } else {
        1
    }
}

extern "C" fn on_chunk_header_wrapper(http: *mut C_Parser) -> libc::c_int {
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_chunk_header() {
        0
    } else {
        1
    }
}

extern "C" fn on_chunk_complete_wrapper(http: *mut C_Parser) -> libc::c_int {
    let parser = unsafe {
        unwrap_parser(http)
    };
    if parser.on_chunk_complete() {
        0
    } else {
        1
    }
}

#[allow(unused_macros)]
macro_rules! notify_fn_wrapper {
    ( $callback:ident ) => ({
        extern "C" fn $callback<H: ParserHandler>(http: *mut C_Parser) -> libc::c_int {
            let context = unsafe { unwrap_context::<H>(http) };
            if context.handler.$callback(context.parser) { 0 } else { 1 }
        };

        $callback::<H>
    });
}
#[allow(unused_macros)]
macro_rules! data_fn_wrapper {
    ( $callback:ident ) => ({
        extern "C" fn $callback<H: ParserHandler>(http: *mut C_Parser, data: *const u32, size: libc::size_t) -> libc::c_int {
            let slice = unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) };
            let context = unsafe { unwrap_context::<H>(http) };
            if context.handler.$callback(context.parser, slice) { 0 } else { 1 }
        };

        $callback::<H>
    });
}

// impl C_ParserSettings {
//     fn new() -> C_ParserSettings {
//         C_ParserSettings {
//             on_url:              on_url_wrapper,
//             on_message_begin:    on_message_begin_wrapper,
//             on_status:           on_status_wrapper,
//             on_header_field:     on_header_field_wrapper,
//             on_header_value:     on_header_value_wrapper,
//             on_headers_complete: on_headers_complete_wrapper,
//             on_body:             on_body_wrapper,
//             on_message_complete: on_message_complete_wrapper,
//             on_chunk_header:     on_chunk_header_wrapper,
//             on_chunk_complete:   on_chunk_complete_wrapper,
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let data = vec![
            "POST /atarget",
            "/subdir/?a=1 HT",
            "TP/1.1\r\n",
            "Content-",
            "Type: text/p",
            "lain\r\n",
            "Connection: close\r\n",
            "Content-Length: 2\r\n",
            "Hello: World\r\n\r\n",
            "Hi"
        ];
        let mut message: HttpMessage = HttpMessage::new();
        {
            let mut parser = Parser::request(&mut message);
            for s in data {
                let n = parser.parse(s.as_bytes());
                println!("Parse loop s: {} nparsed: {}", s, n);
                assert!(n == s.len());
            }
            assert!(parser.message_done);
        }
        assert!(message.get_target() == "/atarget/subdir/?a=1".to_string());
        assert!(message.method == "POST".to_string());
        let v1 = message.headers.find_by_key("Content-Length").unwrap().value.clone();
        let v2 = message.headers.find_by_key("Content-Type").unwrap().value.clone();
        let v3 = message.headers.find_by_key("Hello").unwrap().value.clone();
        assert!(!message.should_keep_alive);

        println!("\n At end of program");
    }

    #[test]
    fn t2() {
        let data = vec![
            "POST /atarget",
            "/subdir/?a=1 HT",
            "TP/1.1\r\n",
            "Content-",
            "Type: text/p",
            "lain\r\n",
            "Content-Length: 2\r\n",
            "Hello: World\r\n\r\n",
            "Hi"
        ];
        let mut message: HttpMessage = HttpMessage::new();
        {
            let mut parser = Parser::request(&mut message);
            for s in data {
                let rv = parser.consume(s.as_bytes());
                println!("Parse loop s: {} nparsed: {} eom:{} ", s, rv.not_consumed, rv.eom);
                assert!(rv.not_consumed == 0);
            }
        }
        assert!(message.get_target() == "/atarget/subdir/?a=1".to_string());
        assert!(message.method == "POST".to_string());
        assert!(message.should_keep_alive);
        let v1 = message.headers.find_by_key("Content-Length").unwrap().value.clone();
        let v2 = message.headers.find_by_key("Content-Type").unwrap().value.clone();
        let v3 = message.headers.find_by_key("Hello").unwrap().value.clone();
        println!("\n At end of program");
    }

    fn test_parser() {}
}