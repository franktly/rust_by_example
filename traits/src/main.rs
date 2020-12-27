use random::Source;
use std::ops;

fn main() {
    traits_basic();
    traits_derive();
    return_traits_with_dyn();
    traits_operator_overloading();
    traits_drop();
    traits_iterators();
    traits_impl_trait();
    traits_clone();
    traits_supertraits();
    traits_disambiguating_overlapping();
}

fn traits_disambiguating_overlapping() {
    trait UsernameWidget {
        fn get(&self) -> String;
    }

    trait AgeWidget {
        fn get(&self) -> u8;
    }

    struct Form {
        username: String,
        age: u8,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "Rustacean".to_owned(),
        age: 28,
    };

    // multiple `get`found and can not decide which one
    // println!("{}", form.get());
    println!("Username Widget: {}", <Form as UsernameWidget>::get(&form));
    println!("Age Widget: {}", <Form as AgeWidget>::get(&form));
}

// Rust doesn't have inheritance but a trait can be a supertrait of another trait(Base class or
// interface like other PL)
fn traits_supertraits() {
    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        // fn name(&self) -> String;
        fn university(&self) -> String;
    }

    trait Programmer: Person {
        // fn name(&self) -> String;
        fn fav_language(&self) -> String;
    }

    trait CsStudent: Programmer + Student {
        // fn name(&self) -> String;
        // fn fav_language(&self) -> String;
        // fn university(&self) -> String;
        fn git_username(&self) -> String;
    }

    struct Cs {
        name: &'static str,
        university: &'static str,
        fav_pl: &'static str,
        git_accout: &'static str,
    }

    impl Person for Cs {
        fn name(&self) -> String {
            String::from(self.name)
        }
    }

    //Person is a supertrait of Programmer
    //Implementing Programmer requires you to also impl Person
    impl Programmer for Cs {
        fn fav_language(&self) -> String {
            String::from(self.fav_pl)
        }
    }

    //Person is a supertrait of Student
    //Implementing Student requires you to also impl Person
    impl Student for Cs {
        fn university(&self) -> String {
            String::from(self.university)
        }
    }

    //Student and Programmer are both supertrait of CsStudent
    //Implementing CsStudent requires you both to  impl Student and Programmer
    impl CsStudent for Cs {
        fn git_username(&self) -> String {
            String::from(self.git_accout)
        }
    }

    fn comp_sci_student_greeting(student: &dyn CsStudent) -> String {
        format!(
            "My name is {}, graduated from {}, favarite PL is {}, git account is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    let cs = Cs {
        name: "Bill",
        university: "Hust",
        fav_pl: "Rust",
        git_accout: "dreamfly@163.com",
    };

    println!("Personal Information: {}", comp_sci_student_greeting(&cs));
}

fn traits_clone() {
    // A unit struct without resources
    #[derive(Debug, Clone, Copy)]
    struct Unit;

    //A tuple struct with resources that implements the `Clone` trait
    #[derive(Clone, Debug)]
    struct Pair(Box<i32>, Box<i32>);

    let unit = Unit;
    let copied_unit = unit;
    println!("original: {:?}, copied: {:?}", unit, copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error for lost its resource for `pair`
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    println!("after cloned moved_pair: {:?}", moved_pair);
    println!("cloned pair: {:?}", cloned_pair);

    drop(moved_pair);
    // `moved_pair` has been dropped
    // println!("after cloned AND DROPPED moved_pair: {:?}", moved_pair);
    println!("after DROPPED cloned pair: {:?}", cloned_pair);
}

// Functions return a type that implements `MyTrait`
// Use `-> impl MyTrait` instead to simplify type signatures
fn traits_impl_trait() {
    use std::iter;
    use std::vec::IntoIter;

    // Origin Version
    // Combine two `Vec<u32>` and returns an iterator over it
    fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let mut v3 = combine_vecs(v1, v2);
    /*
     *  Loop Infinity
     *     for i in v3.into_iter() {
     *         println!("{:?}", i);
     *     }
     *
     */
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());

    println!("All Done");
}

// The `Iterator` trait is used to implement iterators over collections such as arrays
// `next` method and `Item` type
fn traits_iterators() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        //  When the `Iterator` is finished, `None` is returned
        //  Otherwise th next value is wrapped in `Some` and returns
        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            // Since there's no endpoint to a Fibonacci sequence, the `Iterator` will never return
            // `None` and `Some` is always returned
            Some(self.curr)
        }
    }

    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }

    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using For expression: ");
    for i in 0..3 {
        println!("> {}", i);
    }

    // `take(n)` method reduces an `Iterator` to its first `n` terms
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> Fibonacii = {}", i);
    }

    // `skip(n)` method shortens an `Iterator` by dropping its first `n` terms
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> Fibonacii = {}", i);
    }

    let arr = [1u32, 2, 3, 4, 4, 5, 6, 6];
    println!("Iterate the following array: {:?}", arr);
    for i in arr.iter() {
        println!("> Arr = {}", i);
    }
}

// The `Drop` trait only has one method `drop`
// called automatically when an object goes out of scope
// free the resources that the implementor instance owns
fn traits_drop() {
    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    println!("Begin of the main function");

    let _a = Droppable { name: "AAA" };
    // Block A
    {
        // Block B
        let _b = Droppable { name: "BBB" };
        {
            let _c = Droppable { name: "CCC" };
            let _d = Droppable { name: "DDD" };
            println!("Exiting Block B");
        }
        println!("Just Exited Block B");

        println!("Exiting Block A");
    }
    println!("Just Exited Block A");

    // Manually drop `_a`
    drop(_a);

    println!("End of the main function");

    // `_a` won't be dropped again , bz it already has been dropped manually
}

// Operators are syntactic sugar for method calls part of the responding trait
// `+` -> `add` method `Add` trait
fn traits_operator_overloading() {
    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // implement the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");
            FooBar
        }
    }

    // implement the operation: Bar + Foo = BarFoo
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");
            BarFoo
        }
    }

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

// The Rust compiler needs to know how much space every function's return type requires
// All functions HAVE TO reuturn a CONCRETE type
// box is just a reference to some memory in the heap and has a statically-known size
// Return a pointer-to-trait-on-heap
// for exmaple: Box<dyn TraitName>

fn return_traits_with_dyn() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "Sheep Animal Mieeeee....."
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "Cow Animal Miuuuuu....."
        }
    }

    fn random_animal(rand_num: f64) -> Box<dyn Animal> {
        if rand_num < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    let mut source = random::default().seed([0, 1]);

    let rand_number = source.read::<f64>();
    println!("random number is {}", rand_number);
    let animal = random_animal(rand_number);
    println!(
        "You'v randomly chonsed an animal, it says {}",
        animal.noise()
    );
}
// Implementation of SOME traits via #[derive] attribute
// Comparison traits: Eq, PartialEq,Ord, PartialOrd
// Clone -- to create T from &T via a copy
// Hash -- to compute a hash from &T
// Defalut -- to create an empty instance of a data type
// Debug -- to format a value using the {:?} formatter

fn traits_derive() {
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            /*
             * let &Inches(inches) = self;
             * Centimeters(inches as f64 * 2.54)
             */
            Centimeters(self.0 as f64 * 2.54)
        }
    }

    struct Seconds(i32);

    // let _one_second = Seconds(1);
    // Seconds doesn't implment the `Debug` trait
    // println!("One second looks like: {:?}", _one_second);
    //
    // Seconds doesn't implment the `PartialEq` trait
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = {
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        }
    };

    println!("One foot is {} than one meter", cmp);
}

fn traits_basic() {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        fn new(name: &'static str) -> Self;
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            // Implementor methods can use the implementor's trait methods
            if self.is_naked() {
                println!("{} is already naked ...", self.name());
            } else {
                println!("{} gets a haircut", self.name);
                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`

        // Implement trait methods
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name: name,
                naked: false,
            }
        }

        // Implement trait methods
        fn name(&self) -> &'static str {
            self.name
        }

        // Implement trait methods
        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "NAKED Already"
            } else {
                "NEED TO BE NAKED"
            }
        }

        // Default trait methods can be overridden
        fn talk(&self) {
            println!("{} pause briefly... {}", self.name, self.noise());
        }
    }

    //  Type annotation is necessary
    // let mut dolly: Sheep = Animal::new("Dolly");
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
