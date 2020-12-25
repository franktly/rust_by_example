#![crate_type = "lib"] // crate type "lib" or "bin"
#![crate_name = "rary"] // crate name
pub fn public_function() {
    println!("called rary's public_function()");
}

fn private_function() {
    println!("called rary's private_function()");
}

pub fn indirect_access() {
    println!("called rary's indirect_access()");
    private_function();
}
