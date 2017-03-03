#[macro_use]
extern crate lazy_static;
extern crate log_buffer;

use std::cmp;
use std::fmt::Write;
use std::ptr;
use std::str;
use std::sync;

pub const BUFFER_SIZE: usize = 20*256;

lazy_static! {
    static ref LOG_BUFFER: sync::Mutex<log_buffer::LogBuffer<Vec<u8>>> =
        sync::Mutex::new(log_buffer::LogBuffer::new(vec![0; BUFFER_SIZE]));
}

pub fn log(s: &str) {
    LOG_BUFFER.lock().unwrap().write_str(s).unwrap();
}

pub fn buffer<'a>(buf: &'a mut [u8]) -> &'a mut [u8] {
    let mut buffer = LOG_BUFFER.lock().unwrap();
    let slice = buffer.extract();

    let len = cmp::min(slice.as_bytes().len(), buf.len());
    if len <= 0 { return &mut buf[..0] }

    unsafe { ptr::copy_nonoverlapping(slice.as_ptr(), buf.as_mut_ptr(), len) };
    &mut buf[0..len]
}
