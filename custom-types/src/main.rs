use crate::List::*;
use std::fmt::{self, Display};

fn main() {
    structures();
    enums();
    constants();
}

// Globals are declared outside all other scope
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}
fn constants() {
    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The thresold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`
    // THRESHOLD = 5;
}

// use enum to create a linked-list
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),

    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has type `&List` and `*self` has type `List`

        match *self {
            // Can't take owership of the tail, bz  `self` is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocation) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

#[allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// c-like enums

#[allow(dead_code)]

// enum with implicit discriminator(starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum WebEvent {
    // An `enum` may either be `unit-like`
    PageLoad,
    PageUnload,

    // like tuple structs,
    KeyPress(char),
    Paste(String),

    // or c-like structures
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x= {}, y = {}", x, y);
        }
    }
}

// Type aliases
#[derive(Debug)]
enum VeryVeboseEnumOfThingsToDoWidthNumbers {
    Add,
    Subtract,
}

impl VeryVeboseEnumOfThingsToDoWidthNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

impl Display for VeryVeboseEnumOfThingsToDoWidthNumbers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0}, {1}",
            stringify!(Self::Add),
            stringify!(Self::Subtract),
        )
    }
}

type Operations = VeryVeboseEnumOfThingsToDoWidthNumbers;

fn enums() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!("x alias is {}", x);
    println!("5 + 6 = {}", x.run(5, 6));

    // use
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;

    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Solders fight!"),
    }

    // `enums` can be cast as integers
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has len: {}", list.len());
    println!("{}", list.stringify());
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, i32);

// A struct with two fields

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let w = (rect.bottom_right.x - rect.top_left.x).abs();
    let h = (rect.bottom_right.y - rect.top_left.y).abs();

    w * h
}

fn structures() {
    let name = String::from("tly");
    let age = 32;
    let frank = Person { name, age };
    println!("{:?}", frank);

    let pt: Point = Point { x: 10.1, y: 0.5 };
    println!("point coordinates: ({},{})", pt.x, pt.y);

    // Make a new point by using struct update syntax to use other one
    let bottom_right = Point { x: 5.4, ..pt };

    println!("second point: ({},{})", bottom_right.x, bottom_right.y);

    // Destructure teh point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = pt;

    let rect = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!("rect is {:?}", rect);

    println!("rect area is {}", rect_area(&rect));

    // Instantiate a unit struct
    let _unit = Unit;

    let pair = Pair(1, 2);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(int1, int2) = pair;
    println!("int1 is {}, int2 is {}", int1, int2);
}
