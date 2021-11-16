#![no_std]
// #![feature(plugin)]
// #![plugin(application_main_fn)]

#[macro_use] extern crate log;
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

pub type MemuseRes = Result<task::TaskRef, MemuseError>;

pub fn mymemuse(tid: usize, mem_type: MemType) -> MemuseRes {
    // print to the kernel log
    info!("tid: {:?}, mem_type: {:?}", tid, mem_type);
    // check if the memory type is valid
    match mem_type {
        MemType::Heap => (),
        MemType::CallStack => (),
        _ => return Err(MemuseError::InvalidMemtype),
    }
    // check if the task exists
    let task_ref = task::get_task(tid).ok_or(MemuseError::TaskNotExist);
    
    task_ref
}
