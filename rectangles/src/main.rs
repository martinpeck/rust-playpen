fn main() {
    let width = 30;
    let height = 50;
    
    println!(
        "The area of the rectangle is {}",
        area1(width, height)
    );
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {}",
        area2(&rect1)
    );

}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
