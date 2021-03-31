// 2020 openEuler Developer Contest - Question 17
// Author' email: zhaos@nbjl.nankai.edu.cn
extern crate libc;
pub mod uring;
use uring::*;

fn main() {
    let urctx: SampleContext = SampleContext::new(1);
    urctx.submit("/home/zs/stratovirt/sample_program/src/testFile.txt".to_string());
    urctx.read_from_cq();
}
