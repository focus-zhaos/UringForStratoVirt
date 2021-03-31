// 2020 openEuler Developer Contest - Question 17
// Author' email: zhaos@nbjl.nankai.edu.cn

use libc::*;
use std::mem::size_of;
use std::fs::File;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io::Result;
pub const __NR_IO_URING_SETUP: i64 = 425;
pub const __NR_IO_URING_ENTER: i64 = 426;
pub const __NR_IO_URING_REGISTER: i64 =427;

pub const IORING_REGISTER_EVENTFD: u32 = 4;
pub const IORING_ENTER_GETEVENTS: u32 = 1u32;

pub const MAP_POPULATE: c_int = 0x08000;

pub const IORING_OFF_SQ_RING: u64 = 0;
pub const IORING_OFF_CQ_RING: u64 = 0x8000000;
pub const IORING_OFF_SQES: u64 = 0x10000000;

#[repr(C)]
#[derive(Default)]
pub struct IoSqringOffsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

#[repr(C)]
#[derive(Default)]
pub struct IoCqringOffsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub resv: [u64; 2],
}

#[repr(C)]
#[derive(Default)]
pub struct IoUringParams {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub resv: [u32; 5],
    pub sq_off: IoSqringOffsets,
    pub cq_off: IoCqringOffsets,
}

#[repr(C)]
pub union IoUringSqeUnion1 {
    pub rw_flags: i32,
    pub fsync_flags: u32,
    pub poll_events: u16,
    pub sync_range_flags: u32,
    pub msg_flags: u32,
}

#[repr(C)]
pub union IoUringSqeUnion2 {
    pub buf_index: u16,
    pub __pad2: [u64; 3],
}

#[repr(C)]
pub struct IoUringSqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: i32,
    pub off: u64,
    pub addr: u64,
    pub len: u32,
    pub sqe_union1: IoUringSqeUnion1,
    pub user_data: u64,
    pub sqe_union2: IoUringSqeUnion2,
}

#[repr(C)]
#[derive(Default)]
pub struct IoUringCqe {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct Iovec {
    pub iov_base: u64,
    pub iov_len: u64,
}


#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum UringCmd {
    IORING_OP_NOP = 0,
    IORING_OP_READV	= 1,
    IORING_OP_WRITEV = 2,
    IORING_OP_FSYNC = 3,
    IORING_OP_READ_FIXED = 4,
    IORING_OP_WRITE_FIXED = 5,
}


pub struct SampleContext {
    pub ring_fd: i32,
    pub sq_tail: *mut u32,
    pub sq_mask: *mut u32,
    pub sq_arr: *mut u32,
    pub sqes: *mut IoUringSqe,
    pub cq_head: *mut u32,
    pub cq_tail: *mut u32,
    pub cq_mask: *mut u32,
    pub cqes: *mut IoUringCqe,
}

impl SampleContext {
    // 示例程序构造函数，摒弃eventfd的同步模式
    pub fn new(max_size: i32) -> Self {
        let mut p: IoUringParams = Default::default();
        let ret = unsafe { syscall(__NR_IO_URING_SETUP, max_size, &mut p) as i32};

        let sq_size = (p.sq_off.array as usize) + (p.sq_entries as usize) * size_of::<u32>();
        let cq_size = (p.cq_off.cqes as usize) + (p.cq_entries as usize) * size_of::<IoUringCqe>();

        /*
        // In kernel version 5.4 and above
        if p.features & IORING_FEAT_SINGLE_MMAP {
            if  cq_size > sq_size {
                sq_size = cq_size;
            }
            cq_size = sq_size;
        }
        */
        unsafe {
            let mut sq_ptr = mmap (
                std::ptr::null_mut(),
                sq_size,
                PROT_READ | PROT_WRITE, 
                MAP_SHARED | MAP_POPULATE,
                ret,
                IORING_OFF_SQ_RING as i64
            );
    
            let mut cq_ptr = mmap (
                std::ptr::null_mut(),
                cq_size,
                PROT_READ | PROT_WRITE, 
                MAP_SHARED | MAP_POPULATE,
                ret,
                IORING_OFF_CQ_RING as i64
            );
    
            let mut sqe_ptr = mmap (
                std::ptr::null_mut(),
                (p.sq_entries as usize) * size_of::<IoUringSqe>(),
                PROT_READ | PROT_WRITE, 
                MAP_SHARED | MAP_POPULATE,
                ret,
                IORING_OFF_SQES as i64
            ) as *mut IoUringSqe;
    
            
            let mut sq_tail = (sq_ptr as *mut u8).add(p.sq_off.tail as usize) as *mut u32;
            let mut sq_mask = (sq_ptr as *mut u8).add(p.sq_off.ring_mask as usize) as *mut u32;
            let mut sq_arr = (sq_ptr as *mut u8).add(p.sq_off.array as usize) as *mut u32;
            let mut cq_head = (cq_ptr as *mut u8).add(p.cq_off.head as usize) as *mut u32;
            let mut cq_tail = (cq_ptr as *mut u8).add(p.cq_off.tail as usize) as *mut u32;
            let mut cq_mask = (cq_ptr as *mut u8).add(p.cq_off.ring_mask as usize) as *mut u32;
            let mut cqes = (cq_ptr as *mut u8).add(p.cq_off.cqes as usize) as *mut IoUringCqe;
        
            SampleContext {
                ring_fd: ret,
                sq_tail,
                sq_mask,
                sq_arr,
                sqes: sqe_ptr,
                cq_head,
                cq_tail,
                cq_mask,
                cqes,
            }
        }
    }
    // 示例程序提交函数，读取路径为pathstr的文件，注文件大小不超过512字节
    pub fn submit(&self, pathstr: String) {
        let mut f = File::open(pathstr).unwrap();
        let fd: RawFd = f.as_raw_fd();
        let mut buf: [c_char; 512] = [0; 512];

        let mut iov = Iovec {
            iov_base: (&mut buf as *mut [c_char; 512]) as u64,
            iov_len: 512,
        };

        unsafe {
            let mut tail = *(self.sq_tail);
            let index = tail & *(self.sq_mask);
            let mut sqe = self.sqes.add(index as usize);
            (*sqe).fd = fd;
            (*sqe).opcode = UringCmd::IORING_OP_READV as u8;
            (*sqe).addr = &mut iov as *mut Iovec as u64;
            (*sqe).len = 1;
            (*sqe).off = 0;
            (*sqe).user_data = &mut iov as *mut Iovec as u64;
            *(self.sq_arr.add(index as usize)) = index;
            tail = tail + 1;

            if *(self.sq_tail) != tail {
                *(self.sq_tail) = tail;
            }

            syscall(__NR_IO_URING_ENTER, 
                self.ring_fd,
                1,
                1,
                IORING_ENTER_GETEVENTS,
                std::ptr::null_mut() as *mut c_void,
                0
            );
        }

        
    }

    pub fn read_from_cq(&self) {
        unsafe {
            let mut head = *(self.cq_head);
            let mut cqe = self.cqes.add((head & *(self.cq_mask)) as usize);
            let mut res_iov = (*cqe).user_data as *mut Iovec;
            let mut res_buf = (*res_iov).iov_base as *mut c_char;
            for i in 0..100 {
                print!("{}", *(res_buf));
                res_buf = res_buf.add(1);
            }
            /*
            while head != *(self.cq_tail) {
                // get the entry from cq_head
                let mut cqe = self.cqes.add((head & *(self.cq_mask)) as usize);
                let mut res_iov = (*cqe).user_data as *mut Iovec;
                let mut res_buf = (*res_iov).iov_base as *mut char;
                while *res_buf != '\0' {
                    print!("{}", *res_buf);
                    res_buf.add(1);
                }
                head = head + 1;
            }
            */

            *(self.cq_head) = head;
        }
    }
}