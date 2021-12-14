#![no_std]
// #![feature(plugin)]
// #![plugin(application_main_fn)]

#[macro_use] extern crate log;
use core::ops::{DerefMut, Deref};

use task;
 
#[derive(Debug)]
pub enum MemuseError {
    InvalidMemtype,
    TaskNotExist,
}

#[derive(Debug)]
pub enum MemType {
    Heap,
    CallStack,
}

pub type MemuseRes = Result<usize, MemuseError>;

fn get_heap(task_ref: task::TaskRef) -> usize{
    let heap_mem = task_ref.heap_mem.lock();
    *heap_mem
}

fn get_stack(task_ref: task::TaskRef) -> usize{
    task_ref.stack_size()
}

pub fn mymemuse(tid: usize, mem_type: MemType) -> MemuseRes {
    let mut res: usize = 0;
    // print to the kernel log
    info!("tid: {:?}, mem_type: {:?}", tid, mem_type);
    // check if the task exists
    let task_ref = match task::get_task(tid).ok_or(MemuseError::TaskNotExist) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    // check if the memory type is valid
    match mem_type {
        MemType::Heap => res = get_heap(task_ref),
        MemType::CallStack => res = get_stack(task_ref),
        _ => return Err(MemuseError::InvalidMemtype),
    };

    Ok(res)
}
