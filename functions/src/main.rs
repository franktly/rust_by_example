#![feature(never_type)]
fn main() {
    base_fn_test();
    methods_test();
    closures_test();
    closure_capturing_test();
    closure_as_input_param();
    closure_type_anonymity();
    function_as_input_param();
    closure_as_output_param();
    closure_std();
    higher_order_functions();
    derive_functions();
}
// NEVER return using `!` empty type

fn derive_functions() {
    fn foo() -> ! {
        panic!("This call never returns");
    }

    fn some_fn() {
        ()
    }
    /*
     *
     *     let a: () = some_fn();
     *     println!("This function returns and you can see this line");
     *
     *     let x: ! = panic!("This call never returns");
     *     println!("You will never see this line");
     *
     */

    fn sume_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // the return type of this matech expression must be u32, bz of the type of the
            // "adddtion" variable
            let addition: u32 = match i % 2 == 1 {
                // return i32
                true => i,
                // the `continue` expression does not return never returns but still fine
                // bz it can be cast to any other one and therefore used at places where an exact type is required,
                false => continue,
            };

            acc += addition;
        }

        acc
    }

    println!(
        "sum of odd numbers up to 9(excluding): {}",
        sume_odd_numbers(9)
    );
}

fn higher_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    //Function Approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);

    println!("function style: {}", sum_of_squared_odd_numbers);
}

fn closure_std() {
    /*
    pub trait Iterator {
        // The type being iterated over
        type Item;

        // takes `&mut self` meaning the caller may be borrowed and modified, but not consumed
        fn any<F>(&mut self, f: F) -> bool
        where
            // any captured variable may at most be modified, not consumed. takes arguments to the
            // closure by VALUE
            F: FnMut(Self::Item) -> bool,
        {
            f()
        }
    }
    */

    // find through iterators
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // iter() yields `&i32` destructure to i32`
    println!(" 2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // into_iter() yields i32, no destructure
    println!(" 2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];

    // iter() yields `&i32`
    println!("2 in arr1: {}", arr1.iter().any(|&x| x == 2));

    // into_iter() yields `&i32`
    println!("2 in arr2: {}", arr2.into_iter().any(|&x| x == 2));

    // searching through iterators

    /*
    pub trait Iterator {
        type Item;

        fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where
            // takes arguments to the closures by reference
            P: FnMut(&Self::Item) -> bool,
        {
        }
    }
    */

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // destructure &&i32 to i32
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));

    // destructure &i32 to i32
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];

    println!("Find 2 in arr1: {:?}", arr1.iter().find(|&&x| x == 2));
    println!("Find 2 in arr2: {:?}", arr2.into_iter().find(|&&x| x == 2));

    let vec = vec![1, 4, 5, 7, 9, 23, 45, -5, 8, 10];
    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    println!(
        "index of first even number in {:?} is {:?}",
        vec,
        index_of_first_even_number.unwrap()
    );

    let index_of_first_negative_number = vec.iter().position(|x| x < &0);
    println!(
        "index of first negative number in {:?} is {:?}",
        vec,
        index_of_first_negative_number.unwrap()
    );
}

// impl trait
// Fn/FnMut/FnOnce
// move keyword must be used, bz any captures by reference would be dropped as soon as the function
// exited leaving invalid references in the closure
fn closure_as_output_param() {
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

// Closures can be used as arguments
// Functions also can be used as argumetns
// A Function that takes a closure as parameter
// then any FUNCTION that satisfies the trait bound of that clousure can be passed as a parameter

fn function_as_input_param() {
    // A funtion takes a  generic `F` argument bounded by `Fn`
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // define a function satisfying the `Fn` bound
    fn function() {
        println!("I am a function");
    }

    // define a closure satisfying the `Fn` bound
    let closure = || println!("I am a closure");

    call_me(closure);
    call_me(function);
}

fn closure_type_anonymity() {
    // When a closure is defined, the compiler implicitly creates a new anonymous structure to
    // store the captured variables inside. meamwhile implementing the functionality via one of the
    // traits: Fn, FnMut,FnOnce for this unkown type. This type is assigned to the varible which is
    // stored until calling

    // `F` must implement `Fn` trait for a closure which takes nothing and return nothing --
    // exactly what is required for `print`
    fn apply<F>(f: F)
    where
        // F: FnOnce(),
        F: Fn(),
    {
        f();
    }

    let x = 7;
    // Capture x into an anonymous type and implement `Fn` for it. Store it in `print`
    let print = || println!("{}", x);

    apply(print);
}
// should be annonated using one of a few traits
// DECREASING restriction:
// Fn: (&T)
// FnMut: (&mut T)
// FnOnce: (T)
// the compiler will capture variables in the LEAST RESTRICTIVE mannar possible
fn closure_as_input_param() {
    //  <F> denotes that F is a `Generic type parameter`
    // A function takes a closure as an argument and calls it
    fn apply<F>(f: F)
    where
        // The closure takes nothing and return nothing
        F: FnOnce(),
    {
        f();
    }

    // A function takes a closure and return  i32
    fn apply_to_3<F>(f: F) -> i32
    where
        // The closure takes i32 and return i32
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;

    let greeting = "hello";
    // A non-copy type
    // creates owned data from borrowed one
    let mut farwell = "goodbye".to_owned();

    let diary = || {
        // `greeting` is by reference: requires `Fn`
        println!("I said {}", greeting);

        // Mutation forces `farewell` to be captured by mutable reference: requires `FnMut`
        farwell.push_str("!!!");
        println!("Then I screamed {}", farwell);
        println!("Now I can sleep. zzzz");

        // Mannully calling drop forces `farewell` to be captured by value: requires `FnOnce`
        mem::drop(farwell);
    };

    // Call the function which applies the closure
    // Fn, FnMut, FnOnce all work
    apply(diary);

    // `double` satisfies `apply_to_3` trait bound(  Fn  trait  with refer )
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

// moving(move) or borrowing
// by reference: &T, ref T
// by mutable reference: &mut T , ref mut T
// by value: T
fn closure_capturing_test() {
    use std::mem;
    let color = String::from("green");

    // color borrow, `println!` only requires arguments by immutable reference so it doesnot impose
    // anything more restrictive
    let print = || println!("color: {}", color);

    // call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, bz the closure only holds an immutable reference to
    // `color`
    let _reborrow = &color;
    print();

    let _color_move = color;
    // print();  `color` moves out and can not use again

    let mut cnt = 0;
    // A closure to increase `cnt` could take either `&mut cnt` or `cnt` but the previous is less
    // restrictive so it takes that and borrows `cnt`
    // inc requires `mut` bz a `&mut` is stored inside
    let mut inc = || {
        cnt += 1;
        println!("current cnt is {}", cnt);
    };

    // Call the closure using a mutable borrow
    inc();

    // The closure still mutably borrows `cnt` bz it is called later. An attempt to reborrow will
    // lead to an error
    // let _reborrow = &cnt;
    inc();

    let _cnt_reborrow = &mut cnt;

    // cannot borrow the same mut cnt more than once at a time
    // inc();

    // A non-copy type
    let movable = Box::new(3);

    // A copy type would copy into the closure leaving the original untouched.
    // A non-copy type must move and so `movable` immediately moves into the closure
    // `mem::drop` free the heap
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

    consume(); // move movable
               // consume();

    let haystack = vec![1, 2, 3];
    println!("there is {} elements in vec", haystack.len());

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("there is {} elements in vec", haystack.len()); // closure takes ownership of
    // captured variable , in this case `haystack`
    // Uncomment the above line will result in compile-time error bz borrow checker doesn't allow
    // re-using variable after it has been moved

    // Remove `move` form closure will cause closure to borrow haystack immutably, hence is still
    // available
    let _haystack = vec![1, 2, 3];
    let _contains = |needle| _haystack.contains(needle);
    println!("there is {} elements in origin vec", _haystack.len());

    println!("{}", _contains(&1));
    println!("{}", _contains(&5));
    println!("there is {} elements in processed vec", _haystack.len());
}

// closures look like LAMBDA expression
// using `||` instead of () around input variable
// optional body delimination ({}) for a single expression(mandatory otherwise)
// the ability to capture the outer enviroment variables
fn closures_test() {
    fn inc(i: i32) -> i32 {
        i + 1
    }

    let closures_annotated = |i: i32| -> i32 { i + 1 };
    let closures_inferred = |i: i32| i + 1;

    let i = 1;
    println!("inc: {}", inc(i));
    println!("closures_annotated: {}", closures_annotated(i));
    println!("closures_inferred: {}", closures_inferred(i));

    // A closure taking no arguments which returns an `i32`A, the return type is inferred
    let one = || 1;

    println!("closure returning one: {}", one());
}

fn methods_test() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // This is a static method
        // Static method don't need to be called by an instance
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            // Point { x: x, y: y }
            Point { x, y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // This is an instance method
        // Need to be called by an instance
        // `&self` is sugar for `self: &Self` Where `Self` is the type of the caller object.
        // In this case, `Self` = `Rectangle`
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // This method requires the caller object to be mutable
        // `&mut self` desugars to `self: &mut Self`
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p1.y += y;

            self.p2.x += x;
            self.p2.y += y;
        }
    }

    let rect = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance method are called using the dot operator, the first argumetn `&self` is implicitly
    // passed, rect.permeter() == Rectangle::perimeter(&rect)
    println!("rect perimeter: {}", rect.perimeter());
    println!("rect area: {}", rect.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rect is immutable, but this method requires a mutable object
    // rect.translate(1.0, 1.0);

    square.translate(1.0, 1.0);

    // Pair owns resources: tow heap allocated integers
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // This method consume the resources of the caller object
        fn destroy(self) {
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);
            // first and second go out of scope and get freed
        }
    }

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // pair.destroy(); // Previous destory call consumed par object, pair value used after being moved
}

fn base_fn_test() {
    fn is_divisable(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }

        lhs % rhs == 0
    }

    // Functions that don't return a value actually return the unit type `()`
    fn fizzbuzz(n: u32) -> () {
        if is_divisable(n, 15) {
            println!("{} is divisiable by 15", n);
        } else if is_divisable(n, 5) {
            println!("{} is divisiable by 5", n);
        } else if is_divisable(n, 3) {
            println!("{} is divisiable by 3", n);
        } else {
            println!("{}", n);
        }
    }

    // When a function returns `()` the return type can be omitted from the signature
    fn iter_fizzbuzz(n: u32) {
        for n in 1..n + 1 {
            fizzbuzz(n);
        }
    }

    iter_fizzbuzz(100);
}
