#![no_std]
// #![feature(plugin)]
// #![plugin(application_main_fn)]

// #[macro_use] extern crate log;
#[macro_use] extern crate terminal_print;
extern crate alloc;
use alloc::vec;
use memuse::mymemuse;
use bm::getpid;


fn recursive(depth: usize){
    if depth == 10{
        let tid = getpid();
        let mem_type = memuse::MemType::CallStack;
        println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
        let res = mymemuse(tid, mem_type);
        println!("memuse result: {:?}", &res);
        return;
    }
    recursive(depth + 1);
}


pub fn main() -> isize {
        let mut tid = getpid();
        let mut v = vec![1, 2, 3];

        let mut mem_type = memuse::MemType::Heap;
        println!("tid:{:?}, mem_type:{:?}", tid, mem_type);
        let mut res = mymemuse(tid, mem_type);
        println!("memuse result: {:?}", res);

        tid = getpid();
        mem_type = memuse::MemType::Heap;
        println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
        res = mymemuse(tid, mem_type);
        println!("memuse result: {:?}", &res);

        v.push(4);

        tid = getpid();
        mem_type = memuse::MemType::Heap;
        println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
        res = mymemuse(tid, mem_type);
        println!("memuse result: {:?}", &res);        

        tid = getpid();
        mem_type = memuse::MemType::CallStack;
        println!("tid:{:?}, mem_type:{:?}", tid, &mem_type);
        res = mymemuse(tid, mem_type);
        println!("memuse result: {:?}", &res);   

        recursive(0);
    0
}