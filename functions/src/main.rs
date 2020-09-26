fn main() {
    println!("Hello, world!");
    another_function(5, 7);
    let ret = plus_one(12);
    println!("{}", ret);
}

fn another_function(x: i32, y: i32) {
    println!("Another function, where the value of x is {}!", x);
    println!("Almost forgot....the value of y is {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
