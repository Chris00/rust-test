use std::{ffi::{c_int, c_void}, marker::PhantomData, mem::size_of};

extern "C" {
    fn qsort(arr: *mut c_void, nmemb: usize, size: usize,
        cmp: Option<extern fn(a: *const c_void, b: *const c_void) -> c_int>);
}

struct S<T> {
    marker: PhantomData<T>,
}

impl<T: Ord> S<T> {
    fn sort(arr: &mut [T]) {
        unsafe {
            qsort(
                arr.as_mut_ptr() as *mut c_void,
                arr.len(), size_of::<T>(),
                Some(Self::cmp))
        }
    }

    extern "C" fn cmp(a: *const c_void, b: *const c_void) -> c_int {
        unsafe {
            let a = &*(a as *const T);
            let b = &*(b as *const T);
            if a < b { -1 }
            else if a > b { 1 }
            else { 0 }
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arr: [i32; 4] = [3, 7, 1, 0];
    println!("{arr:?}");

    S::<i32>::sort(&mut arr);

    println!("{arr:?}");
    Ok(())
}
