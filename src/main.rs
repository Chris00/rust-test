use std::{ffi::{c_int, c_void}, mem::size_of};

extern "C" {
    fn qsort(arr: *mut c_void, nmemb: usize, size: usize,
        cmp: Option<extern fn(a: *const c_void, b: *const c_void) -> c_int>);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arr: [i32; 4] = [3, 7, 1, 0];
    println!("{arr:?}");

    extern fn cmp(a: *const c_void, b: *const c_void) -> c_int {
        unsafe {
            let a = *(a as *const i32);
            let b = *(b as *const i32);
            if a < b { -1 }
            else if a > b { 1 }
            else { 0 }
        }
    }
    unsafe {
        qsort(
            arr.as_mut_ptr() as *mut c_void,
            arr.len(), size_of::<i32>(),
            Some(cmp))
    }

    println!("{arr:?}");
    Ok(())
}
