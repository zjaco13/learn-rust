fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {result}");

    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count: {count}");


    let mut number = 3;

    while number != 0 {
        println!("number: {number}");
        number -=1;
    }


    let a = [1,2,4,6,2];
    for element in a {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
