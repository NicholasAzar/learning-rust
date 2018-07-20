fn main() {

    // S is a mutable string
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);


    // Ways variables and data interact: MOVE
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);


    let s1 = String::from("hello");
    let s2 = s1.clone();
    // let s2 = s1; // Would cause an error


    println!("s1: {}, s2: {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s); // s moves into the function
    // println!("s: {}", s); // Would cause an error since s is no longer valid.

    let x = 5;
    makes_copy(5); // since i32 is Copy, we still have access to it below.
    println!("x: {}", x);

    let got_it = gives_ownership();
    println!("got: {}", got_it);

    let got_it = takes_and_gives_back(got_it);
    println!("mine: {}", got_it);

    let s3 = String::from("hello");
    let (s3, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s3, len);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String { // moves the value it returns to the caller.
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    let result = String::from("other");
    a_string
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
