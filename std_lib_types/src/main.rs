fn main() {
    // panic_macro();
    results();
    options();
    strings();
    vectors();
    box_stack_and_heap();
    hash_map();
    hash_set();
    rc();
    arc();
}

// `Arc`: Atomic reference Counting
//  shares ownership between threads
//  when the last reference pointer to a value is out of scope, the variable is dropped
fn arc() {
    use std::sync::Arc;
    use std::thread;

    // This variable declaration is where it's value is specified
    let apple = Arc::new("the same apple");
    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference in the memory
        // heap
        let apple = Arc::clone(&apple);
        // OR apple = apple.clone();
        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated in the Arc
            // variable pointer's location
            println!("{:?}", apple);
        });
    }
}

// `Rc`: reference Counting : keeps track of the number of the references which means the number of
// owners of the value wrapped inside an `Rc`
// `Rc` increases by 1 when cloned and decreases by 1 when dropped out of the scope
// `Rc` and the value are all dropped when reference count bz ZERO
// `Rc` NEVER performs a deep copy, Cloning just creates another pointer to the wrapped value and
// increase the count
fn rc() {
    use std::rc::Rc;
    let rc_examples = "Rc examples".to_string();
    {
        println!("-- rc_a is created --");

        // rc_examples is moved into rc_a
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Ref Count of rc_a is {}", Rc::strong_count(&rc_a));

        {
            println!("-- rc_a is cloned to rc_b --");
            let rc_b = Rc::clone(&rc_a);
            println!("Ref Count of rc_b is {}", Rc::strong_count(&rc_b));
            println!("Ref Count of rc_a is {}", Rc::strong_count(&rc_a));

            // Two `Rc`'s equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            println!(
                "Len of the value inside rc_a is {}, len is {}",
                rc_a,
                rc_a.len()
            );

            println!(
                "Len of the value inside rc_b is {}, len is {}",
                rc_b,
                rc_b.len()
            );

            println!("-- rc_b is dropped out of scope --");
        }

        println!("Ref Count of rc_a is {}", Rc::strong_count(&rc_a));
        println!("-- rc_a is dropped out of scope --");
    }

    // rc_exmaples already moved into rc_a
    // when rc_a is dropped, rc_examples is dropped together
    // println!("rc_examples: {}", rc_examples);
}

// HashSet<T> is just a wrapper around HashMap<T,()>
// No duplicate elements(BTreeSet: ordered set)
// 4 Primary operations:
// 1. union: elements in both sets
// 2. difference: elements in first set but not the second set
// 3. intersection: elements ONLY in both sets
// 4. symmetric_difference: elements in one set or the other but not both
//
fn hash_set() {
    use std::collections::HashSet;

    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    println!("insert result: {}", a.insert(5));
    println!("contains result: {}", a.contains(&5));

    println!("insert result: {}", a.insert(5));

    //If a collection's element type implements `Debug`
    //then the collection implements `Debug`
    //print in the format[ elem1, elem2, ..]
    println!("A:{:?}", a);
    println!("B:{:?}", b);

    println!("Union: {:?}", a.union(&b));
    println!("Difference: {:?}", a.difference(&b));
    println!("Intersection: {:?}", a.intersection(&b));
    println!("Symmetirc Difference: {:?}", a.symmetric_difference(&b));
}

// HashMap keys can be booleans, integers,strings or any other type that implements the `Eq` and
// `Hash` traits
// HashMap can start a certain capacity using `HashMap::with_capacity(uint) ` or
// `HashMap::new()`(recommened)

// Any type that implements `Eq` and `Hash` traits can be KEY in `HashMap`
// bool; int;uint(variations thereof);String;&str(get())
// f32,f64 DONOT implement hash for PRESION ERRORS
// collections implement `Eq` and  `Hash` if element type implements `Eq` and `Hash`, For example:
// Vec<T> implements `Hash` if T implements `Hash`
// custom type implement `Eq` and `Hash` via:
// [derive(PartialEq, Eq,Hash)]

fn hash_map() {
    use std::collections::HashMap;

    fn call(num: &str) -> &str {
        match num {
            "798-1364" => "We'are sorry, the call cannot be completed as dialed. Please hang upp and try again",
            "645-7689" => "Hello, this is Mr .Awesome's Pizza, My name is Fred. What can I get for you today",
            _ => "Hi . Who is this again?"
        }
    }

    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "546-7689");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&num) => println!("Calling Daniel: {}", num),
        _ => println!("Don't have Daniel's number"),
    }

    // if exist return `None` otherwise the `Some(new_value)` return
    // no impact for exit already
    contacts.insert("Daniel", "163-1234");
    contacts.insert("Frank", "163-1234");

    match contacts.get(&"Daniel") {
        Some(&num) => println!("Calling Daniel: {}", num),
        _ => println!("Don't have Daniel's number"),
    }

    match contacts.get(&"Ashley") {
        Some(&num) => println!("Calling Ashley: {}", num),
        _ => println!("Don't have Ashley's number"),
    }

    contacts.remove(&"Ashley");

    for (contact, &number) in contacts.iter() {
        println!("Calling {} : {}", contact, number);
    }

    #[derive(PartialEq, Eq, Hash)]
    struct Account<'a> {
        username: &'a str,
        password: &'a str,
    }

    struct AccountInfo<'a> {
        name: &'a str,
        email: &'a str,
    }

    type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

    fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
        println!("Username: {}", username);
        println!("Password: {}", password);
        println!("Attempting logon...");

        let logon = Account { username, password };

        match accounts.get(&logon) {
            Some(account_info) => {
                println!("Successful logon!");
                println!("Name: {}", account_info.name);
                println!("Email: {}", account_info.email);
            }
            _ => println!("Login Failed"),
        }
    }

    let mut accounts: Accounts = HashMap::new();
    let account = Account {
        username: "frank",
        password: "rust2021",
    };

    let account_info = AccountInfo {
        name: "tly",
        email: "dreamflytly@163.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "frank", "rust2021");
    try_logon(&accounts, "frank", "rust2020");
}

fn panic_macro() {
    fn division(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            panic!("division by zero");
        } else {
            dividend / divisor
        }
    }

    let _x = Box::new(0i32);

    //  The panic! can be used to generate a panic and start unwinding its stack. While unwinding,
    //  the runtime will take care of freeing all the resoures owned by the thread by calling the
    //  destructor of all its objects
    division(3, 0);

    println!("This point won't be reached!");
}

fn results() {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op(x: f64, y: f64) -> f64 {
        match div(x, y) {
            Err(why) => panic!("Div Faild for {:?}", why),
            Ok(ratio) => match ln(ratio) {
                Err(why) => panic!("Ln Faild for {:?}", why),
                Ok(ln) => match sqrt(ln) {
                    Err(why) => panic!("Sqrt Faild for {:?}", why),
                    Ok(sqrt) => sqrt,
                },
            },
        }
    }

    fn op_qz_mark_impl(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    fn op_qz_mark(x: f64, y: f64) {
        match op_qz_mark_impl(x, y) {
            Err(why) => panic!(match why {
                MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                MathError::DivisionByZero => "division by zero",
                MathError::NegativeSquareRoot => "square root of negative number",
            }),
            Ok(value) => println!("op qz mark is {}", value),
        }
    }

    println!("op 1 {}", op(100.0, 10.0));
    op_qz_mark(100.0, 11.0);

    // panic
    // println!("op 2 {}", op(1.0, 0.0));
    // panic
    // println!("op 3 {}", op(-2.0, 0.0));
}

fn options() {
    fn check_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    fn try_division(dividend: i32, divisor: i32) {
        match check_division(dividend, divisor) {
            None => println!("{} / {} failed", dividend, divisor),
            Some(quotient) => {
                println!("{} / {} = {}", dividend, divisor, quotient)
            }
        }
    }

    try_division(12, 4);
    try_division(12, 0);

    let none: Option<i32> = None;
    let _equi_none = None::<i32>;
    let opt_float = Some(0f32);

    println!("{:?} unwraps to {:?}", opt_float, opt_float.unwrap());

    // unwrapping None variant will panic
    // println!("{:?} unwraps to {:?}", none, none.unwrap());
}

// `String` Vec<u8> , valid UTF-8 sequence, heap allocated, growable and not null terminated
// `&str` is a slice(&[u8]) that always points to a valid UTF-8 sequence, can be used to view into
// a `String` , just like:  &[T] --> Vec<T>
fn strings() {
    use std::str;

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector and sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    let trimmed_str: &str = string.trim_matches(|x| x == ' ' || x == ',');

    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    // Literals and escapes

    // \ -- escape
    let byte_escape = "I'm writting \x52\x75\x73\x74";
    println!("What are you doing\x3F(\\x3F means ?) {}", byte_escape);

    // ... or Unicode code points \u --unicode
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // \-- linebreak
    let long_string = "String literals
                       can span multiple lines.
                       The linebreak and indentation here ->\
                       <- can be scaped too!";

    println!("{}", long_string);

    //raw string r -- raw bytes
    let raw_str = r"Escape don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // quotes in a raw string , add a pair of #s r#-- quotes
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // '# in string need more than one # to pre and suffix as a pair r####.. --quotes contains #
    let longer_delimiter = r###"A String with "# in it. and even "##!"###;
    println!("{}", longer_delimiter);

    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    // bytes literals cannot hold unicode
    // let escaped_unicode = b"\u{211D}";
    println!("Some escaped bytes: {:?}", escaped);

    // raw byte strings work just like raw strings br-- byte raw
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // raw byte can convert to raw string
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("Add the same as text: {}", my_str);
    }

    // br# --byte raw quotes
    let _quotes = br#"You can also used "" formatting, \
    like with noraml raw strings"#;

    // Byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb";

    // But then they can't always be converted to `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: {}", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}

// pointer to the data, length, capacity
fn vectors() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected(0..10) into : {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    xs.push(4);
    println!("Vector: {:?}", xs);

    // Immutable vectors can't grow
    // collected_iterator.push(0);

    println!("Vector len: {}", xs.len());
    println!("Vector Second Element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop());

    println!("Vector Fourth elements: {:?}", xs[2]);
    println!("Contents of xs: ");

    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("xs[{}] = {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 2;
    }
    println!("Double Updated vector: {:?}", xs);
}

fn box_stack_and_heap() {
    use std::mem;
    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Point> {
        Box::new(Point { x: 0.0, y: 0.0 })
    }

    let pt: Point = origin();
    let rect: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    let boxed_rect: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_pt: Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "Point occupied {} bytes on the stack",
        mem::size_of_val(&pt)
    );

    println!(
        "Rectangle occupied {} bytes on the stack",
        mem::size_of_val(&rect)
    );

    println!(
        "Boxed Point occupied {} bytes on the stack",
        mem::size_of_val(&boxed_pt)
    );

    println!(
        "Boxed Rectangle occupied {} bytes on the stack",
        mem::size_of_val(&boxed_rect)
    );

    println!(
        "Boxed box occupied {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // copy the data contained in `boxed_point`
    // into `unboxed_point`
    let unboxed_point: Point = *boxed_pt;
    println!(
        "Unboxed point occupied {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}
