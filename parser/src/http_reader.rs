#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate libc;
///
/// This module provides the implementation of HttpReader that takes bytes as input from something that implements the Read Trait
/// and outputs std::io::Result<Box<HttpMessage>>
///
/// that is it synchrnously turns a stream of bytes into a stream of parsed http (1.1) messages
///

use std::io::Read;
use std::cmp::min;
use super::http_message::HttpMessage;
use super::parser::Parser;
use details::c_types::{ParserType, ParserError};
use std::alloc::alloc;
use std::ptr::slice_from_raw_parts_mut;

pub struct HttpReader<R: Read> {
    reader: R,
    iobuf: IoBuffer
}

impl <R: Read> HttpReader<R> {
    pub fn new(rdr: R) -> HttpReader<R> {
        HttpReader{
            reader: rdr,
            iobuf: IoBuffer::new(1000),
        }
    }
    pub fn read(&mut self, message: &mut HttpMessage) -> std::result::Result<usize, ParserError> {
        // create a new parser for each read
        let mut parser = Parser::new(ParserType::HttpBoth, message);
        let mut bytes_read: usize = 0;
        loop {
            //
            // handle nothing leftover in the buffer
            // only read more from self.reader if iobuf is empty
            //
            if self.iobuf.bytes_remaining() == 0 {
                // get a slice of free space from iobuf to read into
                let mut rslice = self.iobuf.get_space();
                let read_result = self.reader.read(rslice) ?;
                bytes_read = read_result;
                //
                // handle zero bytes read
                //
                if bytes_read == 0 {
                    if ! parser.started {
                        // eof no message started - there will not be any more bytes to parse so cleanup and exit
                        // return None - return value should be of type Result<Option<Box<HttpMessage>>, Err>
                        // return nothing to do - closed
                        // this is the common way to terminat for a server - since a server waits for a client to
                        // close the connection.
                        return Ok(0);
                    }
                    if parser.started && parser.message_done {
                        assert!(false, "should not get here zero bytes read and eom is signalled");
                    }
                    if parser.started && !parser.message_done {
                        assert!(self.iobuf.bytes_remaining()  == 0, "last read returned 0 bytes and its not eom - push eof into the parser. Confirm iobuf is empty");
                    }
                } else {
                    // got some bytes - which are already in the buffer, but need commit them
                    self.iobuf.commit(bytes_read);
                }
            }
            let sdata = self.iobuf.get_data_as_string();
            println!("before consume data is : {}", sdata);
            let presult = parser.consume(self.iobuf.get_data());
            if presult.error {
                return Err(presult.parser_error);
            } else {
                let n = self.iobuf.bytes_remaining();
                let bytes_consumed = n - presult.not_consumed;
                self.iobuf.consume(bytes_consumed);
                if presult.eom {
                    // parsing of a message just completed so signal complete
                    return Ok(presult.not_consumed);
                }
                assert!(self.iobuf.bytes_remaining() == 0, "should not happen - parser did not consume all bytes on last try and it is not eom");
            }
        }
    }
}
///
/// A buffer that supports partial consumption of the contents (via consume) and
/// adding more to the buffer without overwritting the data already in the buffer
/// via commit.
///
pub struct IoBuffer {
    start: usize,
    length: usize,
    remaining: usize,
    mem_ptr: *mut u8,
}
impl Drop for IoBuffer {
    fn drop(&mut self) {
        let align = std::mem::align_of::<u8>();
        let element_size = std::mem::size_of::<u8>();
        let layout = std::alloc::Layout::from_size_align(element_size * self.length, align)
            .expect("Issue constructing the memory layout.");

        unsafe {std::alloc::dealloc(self.mem_ptr, layout)};
    }
}
impl IoBuffer {
    pub fn new(capacity: usize) -> IoBuffer {
        // let capacity = 1000;
        let mut tmp_mem_ptr: *mut u8;
        let align = std::mem::align_of::<u8>();
        let element_size = std::mem::size_of::<u8>();
        let layout = std::alloc::Layout::from_size_align(element_size * capacity, align)
            .expect("Issue constructing the memory layout.");

        let ptr = unsafe { std::alloc::alloc(layout) } as *mut u8;
        IoBuffer {
            start: 0,
            length: 1000,
            remaining: 0,
            mem_ptr: ptr,
        }
    }
    pub fn get_space(&self) -> &mut [u8] {
        let slice_ptr = unsafe{self.mem_ptr.offset((self.start + self.remaining) as isize)};
        let slice_len = self.length - (self.start + self.remaining);
        let slicemut = std::ptr::slice_from_raw_parts_mut(slice_ptr, slice_len);
        let slicemut2: &mut [u8] = unsafe{&mut *slicemut};
        slicemut2
    }
    pub fn get_data(&self) -> &mut [u8] {
        let slice_ptr = unsafe{self.mem_ptr.offset((self.start) as isize)};
        let slice_len = self.remaining;
        let slicemut = std::ptr::slice_from_raw_parts_mut(slice_ptr, slice_len);
        let slicemut2: &mut [u8] = unsafe{&mut *slicemut};
        slicemut2

    }
    pub fn get_data_as_string(&self) -> String {
        return  String::from_utf8(self.get_data().to_vec()).unwrap();
    }
    pub fn bytes_remaining(&self) -> usize {
        return self.remaining;
    }
    pub fn commit(&mut self, howmany: usize) {
        self.remaining += howmany;
        assert!(self.start < self.length);
        assert!(self.remaining < self.length);
    }
    pub fn consume(&mut self, howmany: usize) {
        assert!(howmany <= self.remaining);
        self.remaining -= howmany;
        if self.remaining == 0 {
            self.start = 0;
        } else {
            self.start += howmany;
        }
        assert!(self.start < self.length);
    }
    pub fn to_string(&self) -> String {
        return String::from_utf8(self.get_data().to_vec()).unwrap();
    }
}

pub struct TestDataReader {
    data: Vec<String>,
    index: usize
}
impl TestDataReader {
    pub fn new(data: Vec<String>) -> TestDataReader {
        TestDataReader{
            index: 0,
            data: data
        }
    }
}
impl std::io::Read for TestDataReader {
    fn read(&mut self, buf: &mut [u8]) ->std::io::Result<usize> {
        if self.index >= self.data.len() {
            let tmp: usize = 0;
            return std::io::Result::Ok(tmp);
        }
        let data_len = self.data[self.index].len();
        if data_len == 0 {
            return std::io::Result::Ok(0);
        }
        if self.data[self.index] == "ioerror" {
            return Err(std::io::Error::from_raw_os_error(902));
        }
        let buf_len = buf.len();
        let number = min(data_len, buf_len);
        let source_bytes =&self.data[self.index].as_bytes()[0..number];
        println!("Source bytes len : {}", source_bytes.len());
        for b in 0..source_bytes.len() {
            buf[b] = source_bytes[b];
        }
        self.index += 1;
        // buf.copy_from_slice(source_bytes);
        return std::io::Result::Ok(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;
    use std::alloc;

    fn t() -> Vec<String> {
        let data = vec!("line1".to_string(), "linekkjhkjh2".to_string(), "line3".to_string());
        data
    }

    #[test]
    fn test_datasource() {
        let data = vec!("line1".to_string(), "linekkjhkjh2".to_string(), "line3".to_string());
        let mut tdata_source = TestDataReader::new(data.clone());
        let mut buf = [0; 100];
        //
        // a bit laborious - but clear
        //
        let mut ix = 0;
        {
            let n = tdata_source.read(&mut buf).unwrap();
            assert!(n != 0);
            let slc = &buf[0..n];
            let s1 = from_utf8(slc).unwrap();
            let s2 = &data[ix];
            assert!(s1 == s2);
            assert!(n == data[ix].len());
        }
        ix += 1;
        {
            let n = tdata_source.read(&mut buf).unwrap();
            assert!(n != 0);
            let slc = &buf[0..n];
            let s1 = from_utf8(slc).unwrap();
            let s2 = &data[ix];
            assert!(s1 == s2);
            assert!(n == data[ix].len());
        }
        ix += 1;
        {
            let n = tdata_source.read(&mut buf).unwrap();
            assert!(n != 0);
            let slc = &buf[0..n];
            let s1 = from_utf8(slc).unwrap();
            let s2 = &data[ix];
            assert!(s1 == s2);
            assert!(n == data[0].len());
        }
        ix += 1;
        {
            let n = tdata_source.read(&mut buf).unwrap();
            assert!(n == 0);
        }
        let er: std::io::Error = std::io::Error::from_raw_os_error(902);
        println!("Done");
    }
    #[test]
    fn test_datasource_ioerror() {
        let data = vec!("line1".to_string(), "ioerror".to_string());
        let mut tdata_source = TestDataReader::new(data.clone());
        let mut buf = [0; 100];
        //
        // a bit laborious - but clear
        //
        let mut ix = 0;
        {
            let n = tdata_source.read(&mut buf).unwrap();
            assert!(n != 0);
            let slc = &buf[0..n];
            let s1 = from_utf8(slc).unwrap();
            let s2 = &data[ix];
            assert!(s1 == s2);
            assert!(n == data[ix].len());
        }
        ix += 1;
        {
            match tdata_source.read(&mut buf) {
                Ok(n) => println!("OK return"),
                Err(e) => println!("error return {}", e),
            }
        }
    }

    fn slice_copy_to_from_string(slice: &mut [u8], s: String) {
        let s_as_bytes = s.as_bytes();
        let dst: &mut [u8] = slice;
        let src = s_as_bytes;
        for (dst, src) in dst.iter_mut().zip(src) { *dst = *src }
    }
    #[test]
    fn test_io_buffer() {
        println!("XXX  Hello test_io_buffer");
        let mut iobmem = IoBuffer::new(100);
        let slice1 = iobmem.get_space();
        slice_copy_to_from_string(slice1, "1234567890".to_string());
        iobmem.commit(10);
        let slice2 = iobmem.get_space();
        slice_copy_to_from_string(slice2, "abcdefghijklmnopqrstuvwxyz".to_string());
        iobmem.commit(26);
        //
        let iobmem_as_string = String::from_utf8(iobmem.get_data().to_vec()).unwrap();
        //
        let d1 = iobmem.get_data();
        assert!(d1.len() == 36);
        assert!(iobmem.start == 0);
        iobmem.consume(9);
        let d2 = iobmem.get_data();
        assert!(d2.len() == 27);
        assert!(iobmem.start == 9);
        let xs = iobmem.get_data_as_string();
        let ys = iobmem.get_data_as_string();
        assert!(iobmem.get_data_as_string() == "0abcdefghijklmnopqrstuvwxyz");
        println!("test_io_buffer {}",   iobmem.to_string());
    }
    #[test]
    fn test_reader()
    {
        let data = vec![
            "POST /atarget".to_string(),
            "/subdir/?a=1 HT".to_string(),
            "TP/1.1\r\n".to_string(),
            "Content-".to_string(),
            "Type: text/p".to_string(),
            "lain\r\n".to_string(),
            "Connection: close\r\n".to_string(),
            "Content-Length: 2\r\n".to_string(),
            "Hello: World\r\n\r\n".to_string(),
            "Hi".to_string()
        ];
        let mut tdata_source = TestDataReader::new(data.clone());
        let mut reader = HttpReader::new(tdata_source);
        let mut msg = HttpMessage::new();
        let r = reader.read(&mut msg);
        println!("Got a message ?");
    }

}