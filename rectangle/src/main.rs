#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("area: {}", area(width1, height1));

    let rect1 = (30, 50);

    println!("area: {}", area_tup(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area: {}", area_struct(&rect2));

    println!("rect: {:?}", rect2);

    println!("area: {}", rect2.area());

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}