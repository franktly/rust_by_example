use std::marker::PhantomData;
use std::ops::Add;

fn main() {
    generic_param();
    generic_function();
    generic_implementation();
    generic_traits();
    generic_trait_bounds();
    generic_empty_bounds();
    generic_multiple_bounds();
    generic_where_clauses();
    generic_new_type_idiom();
    generic_associated_items();
    generic_new_associated_types();
    generic_phantom_type_param();
    generic_phantom_unit_clarification_testcase();
}

fn generic_phantom_unit_clarification_testcase() {
    #[derive(Debug, Clone, Copy)]
    enum Inch {}

    #[derive(Debug, Clone, Copy)]
    enum Mm {}

    // `Length` is a type with phantom type parameter `Unit` and is not generic over the length
    // type(f64)
    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);

    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;

        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            Length(self.0 + rhs.0, PhantomData)
        }
    }

    // `one_foot` have phantom type paramter `Inch`
    let one_foot: Length<Inch> = Length(12.0, PhantomData);

    // `one_meter` have phantom type paramter `Mm`
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // Length implements `Copy` and add() does not consume but copied into `self` and `rhs`
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot plus one foot is {:?}", two_feet);
    println!("one meter plus one meter is {:?}", two_meters);

    // let one_feter = one_foot + one_meter;
    //  Compile-Error: type mismatch for not the same phantom type parameter
}

// A phantom type parameter is one that doesn't show up at runtime, but is checked statically at
// compile time
// Data types can use extra generic type parameters to act as markers or to perform type checking
// at compile time. These extra parameters hold no storage values, and have no runtime behavior

fn generic_phantom_type_param() {
    // A phantom tuple struct which is generic over `A` with hidden parameter `B`
    #[derive(PartialEq)]
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    #[derive(PartialEq)]
    struct PhantomStruct<A, B> {
        first: A,
        phantom: PhantomData<B>,
    };

    // `f32` and `f64` are the hidden paramters
    let _t1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _t2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    let _t3: PhantomTuple<char, f32> = PhantomTuple('q', PhantomData);

    let _s1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _s2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _s3: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Compile-time Error for type mismatch
    // println!("t1 = t2: {}", _t1 == _t2);
    // println!("s1 = s2: {}", _s1 == _s2);
    //
    println!("t1 = t3: {}", _t1 == _t3);
    println!("s1 = s3: {}", _s1 == _s3);
}

// Associated items refers to a set of rules pertaining to items of various types
// It is an extension to trait generic
// Allow traits to internally define new items for example when the trait is generic over its
// container type

fn generic_new_associated_types() {
    struct Container(i32, i32);

    // A trait which checks if 2 items are stored inside of container
    // Also retrieves first or last value

    trait Contains {
        // Define generic types here which method willbe able to utilize
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // Specify `A` and `B` to concrete type
        type A = i32;
        type B = i32;

        // n1, n2 are also valid here
        fn contains(&self, n1: &i32, n2: &i32) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }

        // Grab the first number
        fn first(&self) -> i32 {
            self.0
        }

        // Grab the last number
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let n1 = 3;
    let n2 = 10;

    let my_container = Container(n1, n2);
    println!(
        "Does my_container contain {} and{} : {}",
        &n1,
        &n2,
        my_container.contains(&n1, &n2)
    );

    println!("First number: {}", my_container.first());
    println!("Second number: {}", my_container.last());
    println!("The difference is: {}", difference(&my_container));
}

fn generic_associated_items() {
    struct Container(i32, i32);

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // explicitly requires `A` and  `B`

        fn first(&self) -> i32; // Does not explicitly requre `A` and `B`
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, n1: &i32, n2: &i32) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    let n1 = 3;
    let n2 = 10;

    let my_container = Container(n1, n2);

    println!(
        "Does my_container contains {} and {}: {}",
        &n1,
        &n2,
        my_container.contains(&n1, &n2),
    );

    println!("First number: {}", my_container.first());
    println!("Last number: {}", my_container.last());

    println!("The difference is: {}", difference(&my_container));
}

// The `newtype` idiom gives compile time guarantees that the right type of value is supplied to a
// program
fn generic_new_type_idiom() {
    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 > 18
    }

    let age = Years(4);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
    // `newtype` idiom needs the reference of Years while Days passed and cause compile error
    let years = Years(40);
    let years_as_primitive: i64 = years.0;
    println!("Old enough {}", old_enough(&years));
}

//`where` clauses can apply bounds to arbitrary types, rather than just to type parameters
fn generic_where_clauses() {
    /*
     *
     *     impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A,D> for Your Type {}
     *
     *     impl <A,D> MyTrait<A,D> for YourType where
     *         A: TraitB + TraitC,
     *         D: TraitE + TraitF { }
     */

    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(self);
    }

    impl<T> PrintInOption for T
    where
        // We want `Option<T>:Debug` as our bound bz that is what's being printed. Doing otherwise
        // would be using the wrong bound
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    // PrintInOption::print_in_option(vec);
    vec.print_in_option();
}

fn generic_multiple_bounds() {
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug Print: {:?}", t);
        println!("Display Print: {}", t);
    }

    fn compare_types<T: Debug, U: Display>(t: &T, u: &U) {
        println!("t: {:?}", t);
        println!("u: {}", u);
    }

    let string = "words";
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&vec); Vec doesn't implement `Display` trait
    compare_types(&vec, &string);
}

// A trait doesn't include any functionality can still be used as a bound. for example `Eq` and
// `Copy` trait from the std library
fn generic_empty_bounds() {
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // These two functions are only valid for types which implement these traits. The fact that the
    // traits are empty if irrelevant
    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }

    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A  turkey is {}", red(&_turkey));
    // Red is not implemeted by Turkey
}

fn generic_trait_bounds() {
    use std::fmt::Debug;
    /*
     *
     *     use std::fmt::Display;
     *     struct S<T: Display>(T);
     *     let s = S(vec![1]);
     *
     */
    trait HasArea {
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    // The generic `T` must implement `Debug` trait
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    // `T` must implement `HasArea` trait. Any type which meets the bound can access `HasArea`'s
    // function `area`
    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rect = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let tri = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rect);
    println!("Area: {}", area(&rect));

    // print_debug(&tri);  // tri does not implement the `Debug` trait
    // println!("Area: {}", area(&tri)); // tri does not implement the `HasArea` trait
}

fn generic_traits() {
    struct Empty;
    struct Null;

    // A trait generic over `T`
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an additional single parameter `T` and
        // does nothing with it
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and caller `U`
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments, deallocating both.
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty;
    // null;
    // uncomment the above two line will cause compile error
}

fn generic_implementation() {
    struct S;
    struct GenVal<T>(T);

    impl GenVal<f32> {}
    impl GenVal<S> {}

    // generic impl
    impl<T> GenVal<T> {}

    struct Val {
        val: f64,
    }

    struct GenericVal<T> {
        generic_val: T,
    }

    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    impl<T> GenericVal<T> {
        fn value(&self) -> &T {
            &self.generic_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenericVal { generic_val: 3i32 };

    println!("x = {}, y = {}", x.value(), y.value());
}

fn generic_function() {
    struct A; // concrete type
    struct S(A); // concrete type
    struct SGen<T>(T); // Generic type

    // not a generic function
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}

    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(8));

    // Explicitly specified type parameter `char` to `generic`
    generic::<char>(SGen('a'));
    // Implicitly specified type parameter `char` to `generic`
    generic(SGen('c'));
}

// type parameter(angle brackets and upper camel case)
fn generic_param() {
    struct A;

    struct Single(A);

    struct SingleGen<T>(T);

    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
