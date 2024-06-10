use std::io::{self};

fn main() {
    println!("Hello, world!");
    let int32: i32 = 32;
    let int64: i64 = 64;
    let float_num: f64 = 0.1 + 0.2;
    print!("{} {} {}", float_num, int32, int64);

    let first_string: &str = "Hello ";
    let second_string: String = String::from("World");
    let third_string: String = first_string.to_string() + &second_string;
    println!("{}", third_string);

    let mut boolean = false;
    if boolean {
        println!("true");
        boolean = false;
    } else {
        println!("false");
    }
    print!("boolean is \t {}", boolean);

    match third_string.as_str() {
        "HelloWorld" => println!("HelloWorld"),
        _ => println!("Not HelloWorld"),
    }

    for i in 0..10 {
        print!("{} ", i);
    }

    println!("Hello, world!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("You entered: {}", input.trim());
    let first_name = first_name(input);
    print!("your first name is {}", first_name);
}
fn first_name(str: String) -> String {
    let mut result = String::new();
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        result.push_str(str.as_str())
    }
    return result;
}
