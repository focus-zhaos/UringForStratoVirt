// 2020 openEuler Developer Contest - Question 14
// Author' email: zhaos@nbjl.nankai.edu.cn

extern crate libc;
use util::aio::*;
use std::ffi::{CString, CStr};

fn main() {
    let urctx: SampleContext = SampleContext::new(1);
    let pathstr = CString::new("src/testFile.txt").unwrap();
    let c_path = pathstr.into_raw();
    urctx.submit(c_path);
    urctx.read_from_cq();
}
