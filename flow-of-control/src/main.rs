fn main() {
    if_else_test();
    loop_test();
    nesting_and_labels();
    return_from_loops();
    while_test();
    for_loops();
    match_test();
    destructuring();
    guards_test();
    binding_test();
    if_let_test();
    while_let_test();
}

fn while_let_test() {
    let mut opt = Some(0);

    loop {
        match opt {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9 and Quit ");
                    opt = None;
                } else {
                    println!("current is {} and increasing... ", i);
                    opt = Some(i + 1);
                }
            }
            _ => {
                // Exit the loop when destructure failed and MORE BETTER WAY using `while let`
                // instead
                break;
            }
        }
    }

    // while let instead

    opt = Some(2);
    while let Some(i) = opt {
        if i > 9 {
            println!("Greater than 9 and Quit ");
            opt = None;
        } else {
            println!("current is {} and increasing... ", i);
            opt = Some(i + 1);
        }
    }
    // Less rightward drift and doesn't require explicitly handing the failing case(None will break
    // automatically)
}

fn if_let_test() {
    // int value
    let opt = Some(7);

    match opt {
        Some(i) => {
            println!("this is a really long string and {:?}", i);
        }
        _ => {} // require bz `match ` is exhaustive. wasting space
    }

    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    // `if let` destructure `opt` into `Some(i)` , evaluate the block (`{}`)
    if let Some(i) = opt {
        println!("Matched {:?}", i);
    }

    // specify a failure, use an `else`
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number");
    }

    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}", i);
        // Destructure failed. evaluate an `else if` condition to see if the alternate failure
        // branch should be taken
    } else if i_like_letters {
        println!("Didn't match a number");
    } else {
        println!("Anything else and go with an emotion");
    }

    // enum value

    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // variable matches Foo::Bar
    if let Foo::Bar = a {
        println!("foobar is a");
    }

    // Foo enum neither implements nor derives PartialEq and the following causes a compile-time
    // error
    // if Foo::Bar == a {
    // println!(" a is equal to foobar");
    // }

    if let Foo::Bar = b {
        println!("foobar is b");
    }

    if let Foo::Qux(val) = c {
        println!("c is {}", val);
    }

    //  works with binding value to 100
    if let Foo::Qux(n @ 100) = c {
        println!("biding c is {}", n);
    }
}

fn binding_test() {
    fn age() -> u32 {
        15
    }

    println!("Tell me what type of person you are");

    //match a function's reteurn value
    // binding values to names(u32)
    match age() {
        0 => println!("baby"),
        n @ 1..=12 => println!("child of age {}", n),
        n @ 13..=19 => println!("teenager of age {}", n),
        n => println!("old person of age {}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    //match a function's reteurn value
    // binding values to names(Option)
    match some_number() {
        Some(n @ 42) => println!("The answer is {}!!!", n),
        Some(n) => println!("Not interesting is {}!!!", n),
        // anything else (`None` variant)
        _ => (),
    }
}

fn guards_test() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    // only match one arm
    match pair {
        (x, y) if x == y => println!("tuples are equal"),
        (x, y) if x + y == 0 => println!("tuples are antimatter"),
        (x, _) if x % 2 == 0 => println!("x in tuples are even"),
        (_, _) => println!("the rest"),
    }
}

fn destructuring() {
    // destructure tuples
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    // only match one arm
    match triple {
        (0, y, z) => println!("first is `0`, `y` is {}, `z` is {}", y, z),
        (1, ..) => println!("first is `1`, the rest doesn't matter"),
        _ => println!("it doesn't matter what you are"),
    }

    // destructure enums
    enum Color {
        Red,
        Blue,
        Green,
        // `u32` tuples: color models
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(12, 17, 40);

    println!("What color is it ?");

    match color {
        Color::Red => println!("Red Color"),
        Color::Blue => println!("Blue Color"),
        Color::Green => println!("Green Color"),
        Color::RGB(r, g, b) => {
            println!("RGB Color: R = {}, G = {}, B= {}", r, g, b);
        }
        Color::HSV(h, s, v) => {
            println!("HSV Color: H = {}, S = {}, V= {}", h, s, v);
        }
        Color::HSL(h, s, l) => {
            println!("HSL Color: H = {}, S = {}, L= {}", h, s, l);
        }
        Color::CMY(c, m, y) => {
            println!("CMY Color: C = {}, M = {}, Y= {}", c, m, y);
        }
        Color::CMYK(c, m, y, k) => {
            println!("CMYK Color: C = {}, M = {}, Y= {}, K = {}", c, m, y, k);
        } // Don't need another arm bz all variants have been examined
    }

    // destructure and derefer pointers OR ref
    // defer use `*`
    // destructure use `&` , `ref` , `ref mut`

    // Assign a reference of type ``i32, the `&` signifies there is a reference being assigned
    let reference = &4;

    match reference {
        // matching `&`  are dropped, then the `i32`should be assigned to `val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid `&` , defer it before matching
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // This is NOT a reference bz the right side is not one
    let _not_a_reference = 3;

    // Rust provide `ref` for modifying the assignment so that a reference is created for the
    // element; this reference is assigned
    // this is a REFERENCE via `ref` or `ref mut` signifies
    let ref _is_a_reference = 3;

    // by defing 2 values without references, references can be retrieved via `ref` and `ref mut`

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
        // r => println!("Got a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // defer first
            *m += 10;
            println!("Got a ref mut and add 10 to `mut_value`: {:?}", m);
        }
    }

    // destructure structs

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        // as a whole
        Foo { x, y } => {
            println!("x is {:?}, y is {:}", x, y);
        }

        //seperate
        Foo { x: (1, a), y } => {
            // println!("First of {:?} is 1, second is {:?}, y is {:?}", x, a, y); // x is out of
            // scope
            println!("First of x is 1, second is {:?}, y is {:?}", a, y);
        }

        // swap sequence
        Foo { y: 2, x: a } => {
            println!("y is 2, x is {:?}", a);
        }

        // ignore some variables
        Foo { x, .. } => {
            println!("x is {:?} and ignore the rest", x);
        } //
          // Foo { x } => {} pattern field missing
    }
}

fn match_test() {
    let number = 13;
    println!("Tell me about {}", number);

    match number {
        1 => {
            print!("One!")
        }
        2 | 3 | 5 | 7 | 11 => {
            print!("This is  a Prime!")
        }
        13..=19 => {
            println!("A teen");
        }
        _ => {
            println!("Any other not special");
        }
    };

    let boolean = true;

    let bin = match boolean {
        true => 1,
        false => 0,
    };

    println!("{} --> {}", boolean, bin);
}

fn for_loops() {
    // for and range
    for n in 1..11 {
        println!("cur is {}", n); // 1 inclusive and 11 exclusive
    }

    println!("********************");

    for n in 1..=11 {
        println!("cur is {}", n); // 1 inclusive and 11 inclusive as well
    }

    // for and iterators
}
fn while_test() {
    let mut n = 1;

    while n < 101 {
        if n % 10 == 0 {
            println!("{} is divisable by 10", n);
        } else if n % 5 == 0 {
            println!("{} is divisable by 5", n);
        } else if n % 3 == 0 {
            println!("{} is divisable by 3", n);
        } else {
            println!("{} is not divisable by 10 , 5, 3", n);
        }

        // increase cnt
        n += 1;
    }
}

fn return_from_loops() {
    let mut cnt = 0;

    let ret = loop {
        cnt += 1;

        if cnt == 10 {
            break cnt * 2;
        }
    };

    println!("loop returns {}", ret);
}
fn nesting_and_labels() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This breaks the inner looop
            // break;

            // This breaks the ounter loop
            break 'outer;
        }

        println!("This point will never be reached for breaking the outer loop");
    }

    println!("Exited the outer loop");
}
fn loop_test() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("Got Three and continue to skip the current loop");
            continue;
        }

        println!("cur is {}", count);

        if count == 5 {
            println!("OK, Got Five and break to exit the loop");
            break;
        }
    }
}

fn if_else_test() {
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is small, increase 10-fold");
        10 * n
    } else {
        println!(", and is big, half the number");
        n / 2
    };

    println!("{} ---> {}", n, big_n);
}
