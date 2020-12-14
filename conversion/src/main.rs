use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

fn main() {
    from_and_into();
    try_from_and_try_into();
    string_conversion();
}

// `ToString` trait implementation can convert any type to a `String`
// implement the `fmt::Display` trait which automagically provides `ToString` and also allows
// printing the type as `print!` Macro

struct Circle {
    radius: f32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of Radius {}", self.radius)
    }
}

fn string_conversion() {
    let circle = Circle { radius: 5.0 };
    // implement fmt::Display trait automagically provides `ToString` trait with `to_string` method
    println!("{}", circle.to_string());

    // `FromStr` trait implementation will convert the string into the type specified

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("{} + {} = {}", parsed, turbo_parsed, sum);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

// return Result<Self, Self::Error>
impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
}

// `From` trait allows for a type to define how to create itself from another type,
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// The `Into` trait is simply the reciprocal of the `From` trait. `Into` trait will typically
// require specification of the type to convert into as the compiler is unable to determine this
// most of the time

fn from_and_into() {
    // std string conversion
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("my_string is {}", my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    // let num = int.into();
    println!("My number is {:?}", num);
}

fn try_from_and_try_into() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    println!("{:?}", EvenNumber::try_from(8));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    println!("{:?}", EvenNumber::try_from(5));

    // TryInto
    let res: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(res, Ok(EvenNumber(8)));
    println!("{:?}", res);

    let res: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(res, Err(()));
    println!("{:?}", res);
}
