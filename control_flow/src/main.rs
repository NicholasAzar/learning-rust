fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("number is divisible by 3.");
    } else {
        println!("number not divisible by 4 or 3");
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number: {}", number);
}
