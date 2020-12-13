fn main() {
    base_variable_bindings();
    mutability();
    scope_and_shadowing();
    declare_first();
    freezing();
}

// When data is bound by the same name immutably, it also freezes, Frozen data can not be modified
// until the immutable binding goes out of scope
fn freezing() {
    let mut _mutable_integer = 7i32;
    println!("before frozen release is {}", _mutable_integer);
    {
        // Shadowing by imutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error!  `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // OK! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 4;

    println!("after frozen release  is {}", _mutable_integer);
}

// The compiler forbids use of unitialized variables , as this would lead to undefined behavior
fn declare_first() {
    let a_binding;
    {
        let x = 3;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding is {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding is {}", another_binding);

    another_binding = 2;

    println!("another binding is {}", another_binding);
}

fn scope_and_shadowing() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        let short_lived_binding = 2;

        println!("inner short : {}", short_lived_binding);
    }
    // End of the block

    // Error!
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // shadowing
    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;

    println!("shadowed in outside block: {}", shadowed_binding);
}

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
    // Error!
    // _immutable_binding += 1;
}

#[allow(dead_code)]
fn base_variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("An boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 4u32;

    let noisy_unused_variable = 2u32;
}
