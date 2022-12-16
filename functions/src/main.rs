fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 1;
        x + 1
    };

    println!("the value of y is {y}");

    let x = plus_1(3);
    println!("the value of x is {x}")
}

fn another_function(x: i32) {
    println!("value of x is {x}.");
}

fn plus_1(x: i32) -> i32 {
    x + 1
}
