#![feature(lang_items)]
#![allow(internal_features)]
#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]
include!("../common.rs");
#[allow(dead_code)]
#[derive(Clone,Copy)]
enum Maybe{
    Some(i32),
    None,
}
#[allow(dead_code)]
#[derive(Clone,Copy)]
enum SimpleEnum{
    A,
    B,
    C,
    D,
    E,
    F,
}
#[allow(dead_code)]
fn simple_enum(){
    //let simple_enum = SimpleEnum::A;
    //let _ = black_box(simple_enum);
}
fn main(){
    
    //simple_enum();
    //let maybe:*mut Maybe = core::ptr::null_mut();
    //test_eq!(maybe,core::ptr::null_mut());
    let maybe:*mut Option<i32> = unsafe{malloc(5)}.cast();
    let tag:*mut u8 = maybe.cast();
    unsafe{*tag = 0};
    if let Some(_) = unsafe{*maybe}{
        core::intrinsics::abort();
    }
    //let maybe = Maybe::Some(8);
    black_box(maybe);
}

