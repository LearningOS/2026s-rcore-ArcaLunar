//! Process management syscalls
use crate::mm::{translated_byte_buffer, PageTable, VirtAddr};
use crate::task::{
    change_program_brk, current_user_token, exit_current_and_run_next, get_counter, invoke_map,
    invoke_unmap, suspend_current_and_run_next,
};
use crate::timer::{get_time_us, MICRO_PER_SEC};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let struct_size = core::mem::size_of::<TimeVal>();
    // get underlying frames
    let mut kbuffer = translated_byte_buffer(current_user_token(), ts as *const u8, struct_size);

    let val = TimeVal {
        sec: us / MICRO_PER_SEC,
        usec: us % MICRO_PER_SEC,
    };
    // convert to bytes
    let val_bytes =
        unsafe { core::slice::from_raw_parts((&val as *const TimeVal) as *const u8, struct_size) };

    let mut written = 0;
    for buf in kbuffer.iter_mut() {
        let len = buf.len().min(val_bytes.len() - written); // the actual length to write
        buf[..len].copy_from_slice(&val_bytes[written..written + len]); // copy to each frame
        written += len;
        if written == val_bytes.len() {
            break;
        }
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");

    let token = current_user_token();
    let pt = PageTable::from_token(token);
    let va = VirtAddr::from(id);
    let Some(pte) = pt.translate(va.floor()) else {
        return -1;
    };

    match trace_request {
        2 => get_counter(id),
        1 => {
            if pte.is_valid() && pte.writable() && pte.is_user() {
                pte.ppn().get_bytes_array()[va.page_offset()] = data as u8;
                0
            } else {
                -1
            }
        }
        0 => {
            if pte.is_valid() && pte.readable() && pte.is_user() {
                pte.ppn().get_bytes_array()[va.page_offset()] as isize
            } else {
                -1
            }
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap.");
    invoke_map(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap.");
    invoke_unmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
