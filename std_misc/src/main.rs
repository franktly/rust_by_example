fn main() {
    threads();
    map_reduce();
    channels();
    path();
    file_io();
    child_process();
    filesys_op();
    program_args();
    ffi();
}

// FFI: Foreign Function Interface
// extern block annotation with a #[link] attribute
fn ffi() {
    use std::fmt;

    // extern block links to the libm library
    #[link(name = "m")]
    extern "C" {
        fn csqrtf(z: Complex) -> Complex;
        fn ccosf(z: Complex) -> Complex;
    }

    fn cos(z: Complex) -> Complex {
        unsafe { ccosf(z) }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }

    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // use 0. instead of 0 like 0.0f
            if self.im < 0. {
                write!(f, "{} - {}i", self.re, -self.im)
            } else {
                write!(f, "{} + {}i", self.re, self.im)
            }
        }
    }

    let z = Complex { re: -1., im: 0. };
    let z_sqrt = unsafe { csqrtf(z) };

    println!("The square root of {:?} is {:?}", z, z_sqrt);
    println!("cos({:?}) = {:?}", z, cos(z));
}

fn program_args() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);
    println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}

fn filesys_op() {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::prelude::*;
    use std::os::unix;
    use std::path::Path;

    // open and read
    fn cat(path: &Path) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        /*
         *
         *         match f.read_to_string(&mut s) {
         *             Ok(_) => Ok(s),
         *             Err(e) => Err(e),
         *         }
         *
         */
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // create and write
    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;
        f.write_all(s.as_bytes())
    }

    // create and write
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // create_dir
    println!("`mkdir a`");
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    // create_dir_all
    println!("`mkdir -p a/c/d`");
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("", "").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    // read_dir
    println!("`ls a`");
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    // remove_files
    println!("`rm a/c/e.txt`");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    // remove_dir
    println!("`rmdir a/c/d`");
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

// The `std::Child` struct rep a running child process and exposes the `std::in` , `std::out`,
// `std::err` handles for interaction with the underlying process via pipes
fn child_process() {
    use std::io::prelude::*;
    use std::process::{Command, Stdio};

    // returns process::Output struct
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc successed and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }

    // returns std::Child struct
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    static STR: &'static str = "the quick brown fox jumped over the lazy dog\n";

    match process.stdin.unwrap().write_all(STR.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent string to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => println!("wc responsed with:\n {}", s),
    }

    // child wait
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
}

// returns io::Result<T> OR Result<T,io::Error>
fn file_io() {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io;
    use std::io::prelude::*;
    use std::io::BufRead;
    use std::path::Path;

    let path = Path::new("hello.txt");
    let display = path.display();

    // open in read-only mode and return `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    // returns `io::Result<usize>`
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contians: \n{}", display, s),
    }

    //`file` goes out  of scope, and the `hello.txt` file gets closed

    let path = Path::new("hi.txt");
    let display = path.display();
    /*
     *
     *     // open a file in write-only mode
     *     let mut f2 = match File::create(&path) {
     *         Err(why) => panic!("couldn't create {}: {}", display, why),
     *         Ok(flie) => file,
     *     };
     *
     *     // write str to file
     */
    static HI: &str =
        "hello i love programming rust language, it builds for effcient , fast , safe, powerful software software";

    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
    {
        Ok(ref mut file) => {
            writeln!(file, "Hi rust !!!").unwrap();
            match file.write_all(HI.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
            println!("successfully wrote to {}", display);
        }
        Err(why) => panic!("couldn't write to {}: {}", display, why),
    }

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
            }
        }
    }

    // The output is wrapped in a result to allow matching on errors
    // returns  an iterator to the reader of the lines of the file
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

fn path() {
    use std::path::Path;
    let path = Path::new(".");

    // returns  a `Show`able structure
    let display = path.display();
    println!("path is {}", display);

    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

fn channels() {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;

    static NTHREADS: i32 = 5;

    // Channels have two endpoints: Sender<T> and Receiver<T>:T  is the type of message to be
    // transferred
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // threads handle collection
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation,
            // the thread will continue immmediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Messages are all received here
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // block the current thread if no message is available
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread paniced");
    }

    // Show the order in which the messages were sent
    // println!("{:?}", ids);
    // let res: Vec<_> = ids.iter().filter_map(Result::Ok).collect(); WRONG
    let res: Vec<_> = ids
        .iter()
        // .into_iter()  move ownership
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .collect();
    println!("{:?}", res);
    // println!("{:?}", ids); ERROR if using into_iter()
}

fn map_reduce() {
    use std::thread;
    // WhiteSpace seprated chunk will be handled in a different trhead
    let data = "124 5 4 354325 3587 49 357 77 68 93275  9315 84572 93457 943257 943 2852 947 6827 657 9657638 78";

    //Make a vector to hold the child-threads
    let mut children = vec![];

    let chunked_data = data.split_whitespace();

    for (i, data_seg) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_seg);
        // spawn() returns a handle to the new thread
        // which we MUST keep to access the returned value

        children.push(thread::spawn(move || -> u32 {
            let result = data_seg
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result = {}", i, result);
            result
        }));
    }

    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    let final_result = intermediate_sums.iter().sum::<u32>();
    println!("Final sum result: {}", final_result);
}

// `spawn` function with the argument of a moving closure
fn threads() {
    use std::thread;
    const NTHREADS: u32 = 10;

    // Make a vector to hold the children which are spawaned
    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }))
    }

    for child in children {
        // Wait for the thread to finish. Returns a result
        let _ = child.join();
    }
}
