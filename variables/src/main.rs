fn main() {
    let mut a = 5;
    println!("The value of a is {}", a);
    a = 6;
    println!("The value of a is {}", a);

    let b = 5;
    let b = b + 1;
    let b = b + 1;

    println!("The value of b is {}", b);

    let c = 2.0;
    let d: f32 = 3.1;

    println!("The value of c is {} and the value of d is {}", c, d);

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The valye of y is {}", y);
    println!("The value of the first element is {}", tup.0);

    let ar = [1, 2, 3, 4, 5];
    println!("The value of ar[0] is {}", ar[0])
}
