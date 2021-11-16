#![no_std]
// #![feature(plugin)]
// #![plugin(application_main_fn)]

#[macro_use] extern crate log;

use task;
 
#[derive(Debug)]
pub enum MemuseError {
    InvalidTid,
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
    // info!("Hello, world! (from hello application)");"TaskNotExist"
    info!("tid: {:?}, mem_type: {:?}", tid, mem_type);
    match mem_type {
        MemType::Heap => (),
        MemType::CallStack => (),
        _ => return Err(MemuseError::InvalidMemtype),
    }
    let task_ref = task::get_task(tid).ok_or(MemuseError::TaskNotExist);
    
    task_ref
}
