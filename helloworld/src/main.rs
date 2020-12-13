use std::fmt::{self, Display, Formatter};

fn main() {
    comments();
    formatted_print();
    debug_trait();
    display_trait();
    formatting();
}

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the
        // first argument)
        write!(
            f,
            "{}: {:.3} degree {},{:.3} degree {}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // padding with zeros to a width of N with `:0N[X][O]`
        write!(
            f,
            "RGB({red:03}, {green:03}, {blue:03}) 0x{:02X}{:02X}{:02X}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

fn formatting() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.34778,
            lon: -6.2597,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
    ]
    .iter()
    {
        // Siwtch to use {:?} or {:#?} once you've added an implementation for fmt::Debug
        // #[derive(Debug)]
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation for fmt::Display.
        println!("{:?}", *color);
        println!("{}", *color);
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`

        let vec = &self.0;

        // If it errors, return the error. Otherwise continue
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comm. Use the ? operator to return on
            // errors
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

struct MyStruct(i32);

// To use the `{}` maker, the trait `fmt::Display` must be implemented mannully for the type
impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// fmt::Display may be cleaner than fmt::Debug
// fmt::Display is not implemented for any generic containers. fmt::Debug must then be used for
// these generic cases
//
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x field is {}, y field is {}", self.x, self.y)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real} + {imag}i", real = self.real, imag = self.imag)
    }
}

fn display_trait() {
    println!(
        "my struct print result for implement display trait: {} ",
        MyStruct(5)
    );

    let min_max = MinMax(0, 14);
    println!("Compare structure:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small range is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points: ");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Eror. Both `Debug` and `Display` were implemented, requires `fmt::Binary` to be implemented. This will not work
    // println!("What does Point2D look like in binary: {:b}? ", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare complex: ");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
}

// This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation requried to make this `struct`
// printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn debug_trait() {
    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name",
        "Slater",
        "Chiristian",
        actor = "actor's"
    );

    println!("Now {:?} will print", DebugPrintable(3));

    // The problem with `derive` is there is no control over how the results look
    println!("Now {:?} will print", Deep(DebugPrintable(7)));

    let name = "frank";
    let age = 32;
    let frank = Person { name, age };

    // Pretty print
    println!("{:#?}", frank);

    // Normal print
    println!("{:?}", frank);
}

fn formatted_print() {
    // In general, teh `{}` will be automatically replaced with any arguments. There will be
    // stringified
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is by providing a suffix.
    // the number  31i64 for example has the type i64
    println!("{} days for i64 type", 31i64);
    // There are various optional patterns this works with. Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:` :b means binary format
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 24
    );

    // You can right-align text with a specified width. This will output "     1". 5 white spaces and a
    // "1".

    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeros. This will output "000001"
    println!("{number:>0width$}", number = 1, width = 6);
    // :[fill]<width -- left-aligned with width colums
    // :[fill]^width -- center-aligned with width colums
    // :[fill]>width -- right-aligned with width colums
    println!("Hello {:<5}!", "x");
    println!("Hello {:-<5}!", "x");
    println!("Hello {:^5}!", "x");
    println!("Hello {:>5}!", "x");

    // .N with N width precision
    println!("Hello {0} is {1:.5}", "x", std::f64::consts::PI);
    println!("Hello {0} is {2:.1$}", "x", 6, std::f64::consts::PI);
    println!("Hello {} is {:.*}", "x", 7, std::f64::consts::PI);
    println!("Hello {} is {2:.*}", "x", 8, std::f64::consts::PI);

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bob");

    // Create a structure named `Structure` which contains an `i32`

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated handing. This will not
    // work
    println!("This struct `{:?}` won't print...", Structure(3));
}

fn comments() {
    // This is an example of a line comment
    // There are two slashes at the beginning of the line
    // And nothing written inside these will be read by the compiler
    //
    // println!("hello world!");
    //
    //
    /*
     * This is another type of comment, a block comment. In general,  line comments are the
     * recommended comment type. But block comments are extremely useful fo temporarily disabling
     * chunks of code. /* Block comments can be /*nested, */*/
     * os it takes only a few keystrokes to comment out everything in this main() function /*/*/* Try it yourself!*/*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's no actual need for it
    */

    // You can manipulate expressions more easily with block comments than line comments. Try
    // deleting the comment delimiters to change the result:
    let _y = 20;
    let pi = 3.1415;
    println!("pi is {}", pi);
    let x = 5 + /*90 + */ 5;
    println!("Is `x` 10 or 100? x =  {}", x);
}
