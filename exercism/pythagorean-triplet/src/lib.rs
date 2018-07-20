
fn main() {
    find();
}

// cargo test -- --nocapture
pub fn find() -> Option<u32> {
    const RESULT:i32 = 1000;
    for a in 1..RESULT {
        for b in 1..RESULT {
            let c = RESULT - a - b;
            println!("a: {}, b: {}, c: {}", a, b, c);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                println!("SOLVED: a: {}, b: {}, c: {}", a, b, c);
                return Some(a * b * c);
            }
        }
    }
    None
}
