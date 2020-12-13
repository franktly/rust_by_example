#![allow(overflowing_literals)]
fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

fn aliasing() {
    // UpperCamelCase for Type Alias
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(none_camel_case_types)]
    type u64_t = u64;

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases DON'T provide any extra type and aliases are NOT new types
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

fn inference() {
    let elem = 5u8;

    // Create an empty vector (a growing array)
    let mut vec = Vec::new();

    // At this point the compiler doesn't know the exact type  of `vec`, it  just knows that it's a
    // vector of something (`Vec<_>`)
    vec.push(elem);

    println!("{:?}", vec);
}

fn literals() {
    let x = 1u8;
    let y = 2u16;
    let z = 4f32;

    let i = 1; // i32 by default
    let f = 1.0; // f64 by default

    println!("sizeo of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("sizeo of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("sizeo of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("sizeo of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("sizeo of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn casting() {
    let decimal = 65.4321_f32;

    // Error: No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error: There are limitation in  conversion rules and a float cannot be directly converted to a char
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T, T::MAX +1 is added or subtracted until the
    // value fits into the new type
    //
    // 1000 already fits in a u16
    println!("1000 as a u16 is {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept, while the rest towards
    // the most significant bit (MSB) get truncated
    println!("1000 as a u8 is : {}", 1000 as u8);

    // -1 + 256
    println!("-1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result first casting to the corresponding
    // unsigend type. if the MSB of the value is 1, the value is negative
    //
    //  Unless it already fits, of course
    println!("128 as a i16 is :{}", 128 as i16);
    println!("128 as a i8 is :{}", 128 as i8);

    println!("1000 as a i8 is :{}", 1000 as i8);

    // and the two's complement of 232 is -24
    println!("232 as a i8 is :{}", 232 as i8);
}
