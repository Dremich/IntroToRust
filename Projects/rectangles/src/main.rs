struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 500,
        height: 300,
    };
    let rect2 = Rectangle::square(400);

    println!("The area of rectangle 1 is {} square pixels.", rect1.area());
    println!("The area of rectangle 2 is {} square pixels.", rect2.area());
}

