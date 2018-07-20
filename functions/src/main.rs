fn main() {
    println!("Hello, world!");
    let x = 6;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    another_function(x, 5);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("X after plus_one: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}