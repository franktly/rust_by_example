use std::env;

fn increase(num: i32) {
    println!("{}", num + 1);
}

fn decrease(num: i32) {
    println!("{}", num - 1);
}

fn help() {
    println!(
        "Usage:
    match_args <string>
    Check whether given string is the answer.
    match_args {{increase|decrease}} <integer>
    Increase or Decrease given integer by one.
    "
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("My name is 'match_args'. Try passing some arguments");
        }
        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => println!("This is not the answer."),
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];

            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument is not an integer");
                    help();
                    return;
                }
            };

            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid commmand");
                    help();
                }
            }
        }
        _ => {
            help();
        }
    }
}
