// Attirbutes are metadadta applied to some module, crate or item
// condition compilation of code
// set crate name, version and type(binary or library)
// disable lints(warnings)
// enable complier features(macros, glob imports, etc.)
// link to foreign library
// mark functions as unit tests
// mark functions that will be part of a benchmark
//
// #![crate_attribute] --- whole crate
// #[crate_attribute]  --- module or item
//
// #[attribute = "value"]
// #[attribute(key="value")]
// #[attribute(value)]
// #[attribute(value, value2, value3)]
// #[cfg(...)] : the cfg attribute, enable conditional compilation allowing for checks at
// compile-time
// cfg!: the cfg macro in boolean expressions, evaluates to true or false literals allowing for
// checks at run-time

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are NOT running linux!");
}

fn used_function() {
    println!("called used_function()");
}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

#[cfg(my_con)]
fn conditional_function() {
    println!("condition met");
}

fn main() {
    used_function();

    are_you_on_linux();
    println!("Are you sure?");

    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux");
    } else {
        println!("Yes. It's definitely NOT linux");
    }

    conditional_function();
}
