fn main() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    let number = 3;
    
    if number != 0 {
        println!("number was not zero");
    }
    
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("the number is {}", number);
    
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("the result is {}", result);
    
    for number in (1..11).rev() {
        println!("{}", number);
    }
    println!("LIFT OFF!!");
    
    let list = ["Adam", "Bob", "Carla", "Daisy"];
    
    for name in list.iter() {
        println!("{}", name);
    }
}

