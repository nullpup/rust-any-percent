fn main() {
    println!("Hello, world!");
    let mut sum = 0;
    for i in 1..69 {
        sum += i;
    }
    println!("{sum}");

    for i in (1..69).rev() {
        sum -= i;
    }
    println!("{sum}");
}
