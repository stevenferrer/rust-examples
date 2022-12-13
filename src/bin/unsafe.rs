#![allow(warnings)]

use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    // uncomment to see segfault ;)
    // let address = 0x012345usize;
    // let r = address as *const i32;
    // unsafe {
    //     println!("r is: {}", *r);
    // }

    let mut num = 5;

    // immutable raw pointer
    let r1 = &num as *const i32;
    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    // safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(&mut v, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // uncomment to see segfault
    // let address = 0x01234usize;
    // let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // using extern functions
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // modifying mutable static variables
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe function
unsafe fn dangerous() {
    println!("kaboom!!!")
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
