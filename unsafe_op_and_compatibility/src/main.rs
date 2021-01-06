// Four primary things that unsafe is used for:
// 1. dereferencing raw pointers
// 2. calling functions or methods which are `unsafe`(including calling a function over FFI)
// 3. accessing or modifying static mutable variables
// 4. implementing unsafe traits

fn main() {
    // Raw Pointers
    let raw_p: *const u32 = &10;
    unsafe {
        println!("raw p value is {}", *raw_p);
    }

    let s: &str = "123";
    let raw_sp: *const u8 = s.as_ptr();
    unsafe {
        println!("raw_spp[1] is {}", *raw_sp.add(1) as char);
    }

    let mut arr = [1, 2, 3];
    let raw_ap: *mut i32 = arr.as_mut_ptr();
    unsafe {
        println!("raw_ap is {}", raw_ap.is_null());
        *raw_ap = 11;
        println!("modified raw_ap is {}", *raw_ap);
        println!("modified raw_ap[1] is {}", *raw_ap.add(1));
        println!("modified raw_ap[2] is {}", *raw_ap.add(2));
        println!("modified offset raw_ap[1] is {}", *raw_ap.offset(1));
        println!("modified offset raw_ap[2] is {}", *raw_ap.offset(2));
    }

    let mut mut_int: u32 = 15;
    let int_ptr: *mut u32 = &mut mut_int as *mut u32;
    unsafe {
        println!("original int_ptr is {}", *int_ptr);
        *int_ptr = 18;
        println!("modified int_ptr is {}", *int_ptr);
        println!("modified mut_int is {}", mut_int);
    }

    let ptr: *mut u8 = &mut 10u8 as *mut u8;
    unsafe {
        if let Some(val_back) = ptr.as_ref() {
            println!("got back the value: {}", val_back);

            let val = &*ptr;
            println!("2 got back the value: {}", val);

            let val = *ptr;
            println!("3 got back the value: {}", val);
        }
    }

    // calling Unsafe functions
    use std::slice;
    let some_vector = vec![1, 2, 3, 4];
    let ptr = some_vector.as_ptr();
    let len = some_vector.len();

    // `std::slice::from_raw_parts` creates a slice given a pointer to the first element and a
    // length
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(ptr, len);
        assert_eq!(some_vector.as_slice(), my_slice);
    }

    // Forwards-compatibility
    // keywords with `r#keyword`

    // `try` is a keyword in Edition 2018 but not in 2015(convert 2015 code to 2018 Edition using
    // r#keyword)
    fn r#try() {
        println!("slove keyword compatibility with r#keyword");
    }

    r#try();
}
