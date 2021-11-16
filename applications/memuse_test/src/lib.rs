#![no_std]
// #![feature(plugin)]
// #![plugin(application_main_fn)]

// #[macro_use] extern crate log;
#[macro_use] extern crate terminal_print;
use memuse::mymemuse;

pub fn main() -> isize {
    let tid = 2;
    let mem_type = memuse::MemType::Heap;
    println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
    let res = mymemuse(tid, mem_type);
    println!("memuse result: {:?}", res);
    
    let tid = 2;
    let mem_type = memuse::MemType::CallStack;
    println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
    let res = mymemuse(tid, mem_type);
    println!("memuse result: {:?}", res);

    let tid = 100;
    let mem_type = memuse::MemType::CallStack;
    println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
    let res = mymemuse(tid, mem_type);
    println!("memuse result: {:?}", res);

    0
}