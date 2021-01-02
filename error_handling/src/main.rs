use std::error;
use std::error::Error as _;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    iterate_over_results();
    wrapping_custom_errors();
    boxing_errors();
    define_an_error_type();
    multiple_error_types();
    misc_eh();
    map_and_then();
    map_eh();
    unpack_options_with_qz_mark();
    option_and_unwrap();
    result_eh();
    eh_panic();
}

/*
 *
 * fn main() -> Result<(), ParseIntError> {
 *     let number_str = "d";
 *     let number = match number_str.parse::<i32>() {
 *         Ok(number) => number,
 *         Err(e) => return Err(e),
 *     };
 *     println!("{}", number);
 *     Ok(())
 * }
 *
 */
// `panic` is useful for tests and dealing with unrecoverable errors
// `Option` type is for when a value is optional or when the lack of a value is not an error
// `unwrap` if fine for prototyping and cases where it's absolutely certain that thereis guranteed
// to be a value
// `expect` is more useful since it lets you specify an error message

// if `x` is an `Option`, then evaluating `x?` will return the underlying value if `x`  is `Some`,
// Otherwise it will terminate whatever function is being executed and return `None`
// `?`chains to make code much more readable
//

// A way to write simple code while preserving the original errors is to `Box` them
// The drawback is that the underlying error type is only known at runtime and not statically
// determined

fn iterate_over_results() {
    // `iter::map` operation might fail:
    let strings = vec!["kongfu", "90", "10"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Result is {:?}", numbers);

    // Ignore the failed items with filter_map()
    let s2 = vec!["kongfu", "90", "10"];
    let n2: Vec<_> = s2
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Result is {:?}", n2);

    // `Result` implements `FromIter` so that a vector of results(Vec<Result<T,E>>) can be turned
    // into a result with a vector(Result<Vec<T>, E>)
    // Once `Result:Err` is found, the iteration will terminate
    let s3 = vec!["kongfu", "90", "10"];
    let n3: Result<Vec<_>, _> = s3.into_iter().map(|s| s.parse::<i32>()).collect();

    println!("Result is {:?}", n3);

    // Collect all valid values and failures with `partition()`
    let s4 = vec!["kongfu", "90", "10"];
    let (n4, e): (Vec<_>, Vec<_>) = s4
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok); // n4: partition returns true; e: partition returns false

    let numbers: Vec<_> = n4.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = e.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

fn wrapping_custom_errors() {
    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // defer to the parse error implementation for their error.
        // Supplying extra info requires adding more data to the type
        Parse(ParseIntError),
    }

    type Result<T> = std::result::Result<T, DoubleError>;

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),

                // The wrapped error contans additional information and  is available via the
                // source() method
                DoubleError::Parse(..) => {
                    write!(f, "the provided string could not be parsed as int")
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,

                //The cause is the underlying implementation error type. Is implicitly cast to the
                //trait object `&error::Error`. This works bz the underlying type already
                //implements the `Error` trait
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    // Implement the conversion from `ParseIntErr` to `DoubleError`
    //
    // This will be automatically called by `?` if a
    // `ParseIntError` needs to be converted into a `DoubleError`
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;

        // Here we implicitly use the `ParseIntError` implementation of `From`(which we defined
        // above) in order to create a `DoubleError`
        let parsed = first.parse::<i32>()?;

        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("Caused by: {}", source);
                }
            }
        }
    }

    let numbers = vec!["42", "93", "19"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["rust", "RBE", "12"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

fn boxing_errors() {
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "[Boxing Error]: invalid first item for doubling in empty vec"
            )
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            // Converts to Box
            .ok_or_else(|| EmptyVec.into())
            .and_then(|s| {
                s.parse::<i32>()
                    // Converts to Box
                    .map_err(|e| e.into())
                    .map(|i| 2 * i)
            })
    }

    fn double_first_with_z_marks(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(EmptyVec)?;

        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("boxing info: The first doubled is {}", n),
            Err(e) => println!("boxing error: Error: {}", e),
        }
    }

    let numbers = vec!["42", "92", "19"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["rust", "rbe", "19"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));

    let numbers2 = vec!["42", "92", "19"];
    let empty2: Vec<&str> = vec![];
    let strings2 = vec!["rust", "rbe", "19"];

    print(double_first_with_z_marks(numbers2));
    print(double_first_with_z_marks(empty2));
    print(double_first_with_z_marks(strings2));
}

fn define_an_error_type() {
    #[derive(Debug, Clone)]
    struct DoubleError;

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[define error type]: invalid first item to double")
        }
    }

    type Result<T> = std::result::Result<T, DoubleError>;

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            // Change the error to our new type
            .ok_or(DoubleError)
            .and_then(|s| {
                s.parse::<i32>()
                    // Update to the new error type here alos
                    .map_err(|_| DoubleError)
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("Define Error Type Info: The first doubled is {}", n),
            Err(e) => println!("Define Error Type Error: {}", e),
        }
    }

    let numbers = vec!["42", "92", "19"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["rust", "rbe", "19"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

fn multiple_error_types() {
    fn double_first(vec: Vec<&str>) -> i32 {
        let first = vec.first().unwrap(); // Gen err 1
        2 * first.parse::<i32>().unwrap() // Gen err 2
    }

    fn double_first_map(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
    }

    let numbers = vec!["43", "3", "545"];
    let numbers_2 = vec!["43", "3", "545"];
    let empty: Vec<&str> = vec![];
    // let empty:  = vec![];
    let strings = vec!["kongfu", "3", "545"];

    println!(
        "Multiple Error Type: The first doubled is {}",
        double_first(numbers)
    );
    println!(
        "Multiple Error Type: The first doubled map is {:?}",
        double_first_map(numbers_2)
    );
    // Err 1
    // println!("The empty doubled is {}", double_first(empty));
    // Err 2
    // println!("The strings doubled is {}", double_first(strings));

    // stop processing on errors(like `?`) but keep going when the `Option` is `None`
    fn double_first_with_map_or(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

        opt.map_or(Ok(None), |r| r.map(Some))
    }

    let num = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let s2 = vec!["rust", "perfect", "12"];

    println!(
        "The first double_first_with_map_or
     is {:?}",
        double_first_with_map_or(num)
    );
    println!(
        "The first double_first_with_map_or
     is {:?}",
        double_first_with_map_or(empty)
    );
    println!(
        "The first double_first_with_map_or
     is {:?}",
        double_first_with_map_or(s2)
    );
}

// `?` is almost exactly equivalent to an `unwrap` which returns instead of panicking on Errs
// map For `Result`
fn misc_eh() {
    type AliasResult<T> = Result<T, ParseIntError>;

    // fn multiply(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
    fn multiply(f_n_str: &str, s_n_str: &str) -> AliasResult<i32> {
        match f_n_str.parse::<i32>() {
            Ok(first_number) => match s_n_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // fn multiply_map_and_then(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
    fn multiply_map_and_then(f_n_str: &str, s_n_str: &str) -> AliasResult<i32> {
        f_n_str.parse::<i32>().and_then(|first_number| {
            s_n_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn multiply_early_returns(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
        let f_n = match f_n_str.parse::<i32>() {
            Ok(first_number) => first_number,
            Err(e) => return Err(e),
        };

        let s_n = match s_n_str.parse::<i32>() {
            Ok(second_number) => second_number,
            Err(e) => return Err(e),
        };

        Ok(f_n * s_n)
    }

    // unwrap or return Err(err)
    fn multiply_with_qz_marks(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
        let f_n = f_n_str.parse::<i32>()?;
        let s_n = s_n_str.parse::<i32>()?;

        Ok(f_n * s_n)
    }
    /*
     *
     *     // older code with try macro[Older Code]
     *     fn multiply_with_try_macro(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
     *          let f_n = try!(f_n_str.parse::<i32>());
     *          let s_n = try!(s_n_str.parse::<i32>()); ///!try macro is deprecated in 2018Edition
     *
     *         Ok(f_n * s_n)
     *     }
     *
     */
    /*
     *  Error !!!
     *         f_n_str.parse::<i32>().map(|first_number| {
     *             {
     *                 s_n_str
     *                     .parse::<i32>()
     *                     .and_then(|second_number| first_number * second_number)
     *             })
     */

    // fn print(result: Result<i32, ParseIntError>) {
    fn print(result: AliasResult<i32>) {
        match result {
            Ok(n) => {
                println!("n is {}", n);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    let twenty = multiply("10", "3");
    print(twenty);

    let ttd = multiply("d", "3");
    print(ttd);

    let twenty = multiply_map_and_then("10", "3");
    print(twenty);

    let ttd = multiply_map_and_then("d", "3");
    print(ttd);

    let twenty = multiply_early_returns("10", "3");
    print(twenty);

    let ttd = multiply_early_returns("d", "3");
    print(ttd);

    let twenty = multiply_with_qz_marks("10", "3");
    print(twenty);

    let ttd = multiply_with_qz_marks("d", "3");
    print(ttd);
}

//
// `Result` is a richer version of `Option`
// Result<T,E>:
// Ok(T): An element T was found
// Err(E): An error was found with element E

fn result_eh() {
    fn multiply(f_n_str: &str, s_n_str: &str) -> i32 {
        let f_n = f_n_str.parse::<i32>().unwrap();
        let s_n = s_n_str.parse::<i32>().unwrap();

        f_n * s_n
    }

    let twenty = multiply("4", "10");
    println!("4 * 10 is {:?}", twenty);

    let tt = multiply("d", "6"); // panic for unwrap invoke
    println!("6 * d is {:?}", tt);
}

// `and_then()` calls its function input with the wrapped value and returns the result. If the
// Option is None, then it returns None instead

fn map_and_then() {
    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }

    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    fn have_ingredients(food: &Food) -> Option<&Food> {
        match food {
            Food::Sushi => None,
            _ => Some(food),
        }
    }

    fn have_recipe(food: &Food) -> Option<&Food> {
        match food {
            Food::CordonBleu => None,
            _ => Some(food),
        }
    }

    // chain of match
    fn cookable_v1(food: &Food) -> Option<&Food> {
        match have_recipe(food) {
            None => None,
            Some(food) => match have_ingredients(food) {
                None => None,
                Some(food) => Some(food),
            },
        }
    }

    // This can conveniently be rewritten move compactly with `and_then()`
    fn cookable_v2(food: &Food) -> Option<&Food> {
        have_recipe(food).and_then(have_ingredients)
    }

    fn eat(food: &Food, day: Day) {
        match cookable_v2(food) {
            Some(food) => println!("v2: Yay! On {:?} we get to eat {:?}", day, food),
            None => println!("v2: Oh on. We don't get to eat on {:?}", day),
        };
        match cookable_v1(food) {
            Some(food) => println!("v1: Yay! On {:?} we get to eat {:?}", day, food),
            None => println!("v1: Oh on. We don't get to eat on {:?}", day),
        }
    }

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(&cordon_bleu, Day::Monday);
    eat(&steak, Day::Tuesday);
    eat(&sushi, Day::Wednesday);
}
//
// `Option` built in method `map()`
fn map_eh() {
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    // Food -> Peeled
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // Peeled -> Chopped
    fn chop(peel: Option<Peeled>) -> Option<Chopped> {
        match peel {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    // Chopped -> Cooked
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // Food -> Cooked
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mm, I love {:?}", food),
            None => println!("Oh no! It wasn't edible"),
        }
    }

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;
    let p2 = Some(Food::Potato);

    let cooked_apple = cook(chop(peel(apple)));

    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);
    let cooked_p2 = process(p2);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
    eat(cooked_p2);
}

fn unpack_options_with_qz_mark() {
    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        let next_age: u8 = current_age?;
        Some(format!("Next year I wll be {}", next_age))
    }

    println!("Next birthday: {:?}", next_birthday(Some(10)));

    struct Person {
        job: Option<Job>,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_num: Option<PhoneNum>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNum {
        area_code: Option<u8>,
        number: u32,
    }

    impl Person {
        fn work_phone_area_code(&self) -> Option<u8> {
            self.job?.phone_num?.area_code
        }
    }

    // Id{ filed: Some(Id2{field2: Some(id3{...})})}
    let p = Person {
        job: Some(Job {
            phone_num: Some(PhoneNum {
                area_code: Some(222),
                number: 888888,
            }),
        }),
    };

    println!("work_phone_area_code is {:?}", p.work_phone_area_code());
}
fn option_and_unwrap() {
    fn give_commoner(gift: Option<&str>) {
        match gift {
            Some("snake") => println!("Yuck!  I'm putting this snake back in the forest"),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No gift? Oh well"),
        }
    }

    fn give_royal(gift: Option<&str>) {
        let inside = gift.unwrap();
        if inside == "snake" {
            panic!("Poinz Snake!!!!");
        }
        println!("I love {:?}s", inside);
    }

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_royal(bird);
    give_royal(nothing);
}

fn eh_panic() {
    fn drink(beverage: &str) {
        if beverage == "lemonde" {
            panic!("AAAaaa!!!");
        }
        println!("Some refreshing {} is all I need", beverage);
    }

    drink("water");
    drink("lemonde");
}
