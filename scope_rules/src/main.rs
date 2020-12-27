// Scopes include ownership, borrowing, and lifetime
fn main() {
    raii();
    owership_and_moves();
    mutability();
    borrowing();
    aliasing();
    ref_pattern();
    lifetime();
}

fn lifetime() {
    let i = 3; // Lifetime for `i` starts
    {
        let borrow1 = &i; //`borrow1` lifetime starts
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends

    {
        let borrow2 = &i; // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    } // `borrow2` ends

    /*
     * EXPLICIT ANNOTATION:
     */

    // foo<'a> , foo<'a,'b>, &'a T

    // takes two referenes to `i32` which have different lifetimes `a` and `b`
    // These two  lifetimes must both be at least as long as the function `print_refs`
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {}, y is {}", x, y);
    }

    // A function which takes no arguments, but has a lifetime parameter `'a`
    fn failed_borrow<'a>() {
        let _x = 12;

        // ERROR: `_x` does not live long enough
        // let y: &'a i32 = &_x;
        // Attempting to use the lifetime `'a` as an explicit type  annotation inside the function
        // will fail bz the lifetime of `&_x` if shorter than that of `y`(function's lifetime)
        // A short lifetime cannot be coerced into a longer one
    }

    // Borrows('&') of both variables are passed into the function
    let (four, nine) = (4, 9);

    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must be longer than that of `print_refs`
    print_refs(&four, &nine);

    failed_borrow();
    // `failed_borrow`contains no references to force `'a` to  be longer than the lifetime of the
    // function, but `'a` is longer
    // Bz the lifetime is never constrained, it defaults to `'static`

    /*
     * FUNCTION:
    /*! Any reference MUST have an annotated lifetime
     */ Any reference being returned MUST have the same lifetime as an input or be STATIC
     */

    // One input reference with lifetime `a` which must live at least at long as the function
    fn print_one<'a>(x: &'a i32) {
        println!("print_one: x is {}", x);
    }

    // Mutable reference are possible with lifetime as well
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
        println!("add_one: x is {}", x);
    }

    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("print_multi: x is {}, y is {}", x, y);
    }

    fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
        x
    }
    /*
     *
     *     fn invalid_out<'a>() -> &'a String {
     *         &String::from("hello")
     *     }
     *     // 'a must live longer than the function
     *     // Here, `&String::from("hello")` would create a `String`, followed by a reference. Then the
     *     // data is dropped upon exiting the scope, leaving a reference to invalid data to be returned
     */

    let x = 7;
    let y = 8;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    /*
     * METHODS:
     */
    struct Owner(i32);

    impl Owner {
        // Annotate lifetime as in a standalone function
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'a>(&'a self) {
            println!("print: {}", self.0);
        }
    }

    let mut owner = Owner(19);
    owner.add_one();
    owner.print();

    /*
     * Structs:
     */

    // The reference to `i32` must outlive `Borrowed`
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // both reference here MUST outlive this structure
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 10;
    let y = 20;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };

    let refer = Either::Ref(&x);
    let num = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", refer);
    println!("y is NOT borrowed in {:?}", num);

    /*
     * Traits:
     */

    #[derive(Debug)]
    struct TraitsBorrowed<'a> {
        x: &'a i32,
    }

    // `impl` KEYWORD need explicitly be annotated
    impl<'a> Default for TraitsBorrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    let b: TraitsBorrowed = Default::default();
    println!("b is {:?}", b);

    /*
     * Bounds:
    /*! T:'a: All reference in T must outlive  lifetime 'a
     */  T: Trait + 'a: Type A must implement trait Trait and all references in T must outlive 'a
     */

    use std::fmt::Debug;

    // `Ref` contains a reference to `T` that has an unknown lifetime `'a` `T` is bounded such
    // that any Reference in `T` must outlive `a`
    // The lifetime of `Ref` may not exceed `a`

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("print: t is {:?}", t);
    }

    // A reference to `T` is taken where `T` implements `Debug` and all Reference in `T` outlive `a`
    // `a` must outlive the function
    fn print_ref2<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("print ref2: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);

    print_ref2(&x); // i32 implement Debug trait by default
    print_ref2(&ref_x);

    print(x);
    print(ref_x);

    /*
     * Coercion:
     * A longer lifetime can be coerced into a shorter one so that is works inside a scope
     * */

    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    // <'a: 'b> means lifetime of a is at least as long as b
    // Here, we take in an `&'a i32` and return `&'b i32` as a result of coercion
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    let first = 2; // longer lifetime
    {
        let second = 4; // shorter lifetime
        println!(
            "The product of {} *{} is {}",
            first,
            second,
            multiply(&first, &second)
        );

        println!("{} is the first", choose_first(&first, &second));
    }

    /*
     * Static:
     */

    // Reference lifetime
    // READ-ONLY memory of the binary:
    // Make a constant with the `static` declaration
    // Make a `string` literal which has type: `&'static str`

    static NUM: i32 = 10;

    // static lifetime is coerced to that of the input argument
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        let static_string = "I am in read-only memory";
        // When `static_tring` goes out of scope, the reference can no longer be used, but the data
        // remains in the binary
    }

    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("Num : { } stays accessible!", NUM);

    // Trait Bound
    // Type does not contain any non-static references
    // The receiver can hold on to the type for as long as they want and it will neer become
    // invalid until they drop it
    //
    // Any owned data always pass a 'static lifetime bound, but a reference to that owned data
    // generally does not
    //

    fn print_it(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }

    fn use_it() {
        // i is owned and contains no reference, thus it's 'static.
        let i = 5;
        print_it(i);
        // borrowed value does not live long enough
        // print_it(&i);
    }

    /*
     * Elision:
     */

    fn elided_input(x: &i32) {
        println!("elided_input: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("annotated_input: {}", x);
    }

    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    let x = 6;
    elided_input(&x);
    annotated_input(&x);

    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));
} // Lifetime ends

fn ref_pattern() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let c = "Q";

    // the following lines are the same
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid  when destructuring a struct
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mut_point = point;
    {
        // `ref` can be paired with `mut` to make mutable reference
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mut_point;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable point is ({}, {})", mut_point.x, mut_point.y);

    // A mutable tuple that includes a pointer
    let mut mut_tuple = (Box::new(4u32), 3u32);
    {
        let (_, ref mut last) = mut_tuple;
        *last = 10u32;
    }

    println!("tuple is {:?}", mut_tuple);
}

// Data can be immutably borrowed any number of times, but while immutably borrowed, the origin
// data can't be mutably borrowed(One mutable borrow is allowed at a time, The origin data can be
// borrowed again ONLY AFTER the mutable reference has been used for the last time)

fn aliasing() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;
    println!(
        "Point has coordinate: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    //  Can't borrow `point` as mutable bz it's currently borrowed as immutable
    /*
     *
     *     let mut_borrow = &mut point;
     *     println!(
     *         "Point has coordinate: ({}, {}, {})",
     *         borrowed_point.x, another_borrow.y, point.z
     *     );
     */

    let mut_borrow_point = &mut point;

    mut_borrow_point.x = 5;
    mut_borrow_point.y = 1;
    mut_borrow_point.z = 5;

    //Can't borrow `point` as mutable bz it's currently borrowed as immutable
    // let y = &point.y;
    // Can't print bz `println!` takes an immutable reference
    // println!("Point z coordinate is {}", point.z);

    // OK! Mutable references can be passed as immutable to `println!`
    println!(
        "Point has coordinate ({}, {}, {})",
        mut_borrow_point.x, mut_borrow_point.y, mut_borrow_point.z
    );

    // The mutable reference  is NO LONGER USED for the REST of the code so it is possible to
    // reborrow
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinate ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

// Without taking ownership over it using `borrowing` mechanism(&T)
// borrow checker(Reference always point to valid object)

fn borrowing() {
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // Can't destroy `boxed_i32` while the inner value is borrowed later
        // eat_box_i32(boxed_i32);

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed if eat_box_i32 invoke is
        // uncommented
        borrow_i32(_ref_to_i32);

        // `_ref_to_i32` goes oout of scope and no longer borrowed
    }

    // `boxed_i32` can now give up owership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}

fn mutability() {
    let immut_box = Box::new(5u32);
    println!("immut_box contains: {}", immut_box);

    // Mutability Error
    // *immut_box = 4;

    // Move the box and change the owership(and mutability)
    let mut mut_box = immut_box;
    *mut_box = 4;
    println!("mut_box contains: {}", mut_box);

    // &mut T: mutable reference
    // &T: immutable reference

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        // `&'static str` is a reference to a string allocated in READ ONLY MEMORY
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!(
            "immutable borrowed {} - {} edition written by {}",
            book.title, book.year, book.author
        );
    }

    fn new_edition(book: &mut Book) {
        book.year = 2021;
        println!(
            "mutable borrowed {} - {} NEW edition written by {}",
            book.title, book.year, book.author
        );
    }

    let immut_book = Book {
        author: "frank",
        title: "Rust Program",
        year: 2020,
    };

    // create a mutable COPY of `immut_book` (implement Clone and Copy Trait)
    let mut mut_book = immut_book;

    // Immutably borrow an immutable object
    borrow_book(&immut_book);

    // Immutably borrow a mutable object
    borrow_book(&mut_book);

    // Borrow a mutable  object as mutable
    new_edition(&mut mut_book);

    // Cann't borrow a immutable object as mutable
    // new_edition(&immut_book);
}

// Owership of the resources is transferred(MOVE):
// Assigments(let x= y)
// Passing function argumetns by value(foo(x))
//
fn owership_and_moves() {
    // This function takes owership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);
        // `c` is destroyed and the memory is freed
    }

    // stack allocated integer
    let x = 5u32;
    // Copy x into y -- no resources are moved
    let y = x;
    println!("x is {}, y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // move `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`
    // Both are now pointers to the same heap allocated data, but `b` now owns it

    // `a` can no longer access the data, bz it no longer owns the heap memory
    // println!("a contains: {}", a);
    println!("b contains: {}", b);

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this  point, this action would result in deferencing
    // freed memory
    // println!("b contains: {}", b);
}

fn raii() {
    fn create_box() {
        // Allocate an integer on the heap
        let _b1 = Box::new(3i32);
        // `_b1` is destoryed here and memory gets freed
    }

    let _b2 = Box::new(4i32);
    {
        let _b3 = Box::new(5i32);
        // `_b3` is destoryed here and memory gets freed
    }

    // There's no need to manually free memory
    for _ in 0u32..1_000 {
        create_box();
    }
    // `_b2` is destoryed here and memory gets freed

    struct ToDrop;
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");
}
