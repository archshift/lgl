extern crate lgl;

use std::slice;
use std::str;

#[repr(C)]
pub struct LogBufferView {
    buf_ptr: *const u8,
    buf_size: usize
}

#[repr(C)]
pub struct LogBufferMutView {
    buf_ptr: *mut u8,
    buf_size: usize
}

#[no_mangle]
pub extern fn lgl_log(buf: LogBufferView) {
    let s = unsafe {
        let slice = slice::from_raw_parts(buf.buf_ptr, buf.buf_size);
        str::from_utf8(slice).unwrap()
    };
    lgl::log(s)
}

#[no_mangle]
pub extern fn lgl_buffer(buf: LogBufferMutView) -> LogBufferView {
    let new_slice = unsafe {
        lgl::buffer(slice::from_raw_parts_mut(buf.buf_ptr, buf.buf_size))
    };
    LogBufferView { buf_ptr: new_slice.as_ptr(), buf_size: new_slice.len() }
}

#[no_mangle]
pub extern fn lgl_buffer_size() -> usize {
    lgl::BUFFER_SIZE
}
