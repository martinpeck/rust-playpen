fn main() {
    let _v: Vec<i32> = Vec::new();
    
    let mut v = vec![1,2,3];
    
    v.push(4);
    v.push(5);
    v.push(6);
    
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("The third element is {}", third),
    }
    
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
