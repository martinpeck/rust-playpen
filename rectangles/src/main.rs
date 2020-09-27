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
    
    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );
    
    println!("rect1 is {:#?}", rect1);
    
    let rect_a = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect_b = Rectangle {
        width: 10, 
        height: 40,
    };
    
    let rect_c = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("Can rect_a hold rect_b? {}", rect_a.can_hold(&rect_b));
    println!("Can rect_a hold rect_c? {}", rect_a.can_hold(&rect_c));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
