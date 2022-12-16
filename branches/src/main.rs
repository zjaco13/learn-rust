fn main() {
    let number = 3;
    if number < 5 {
        println!("true condition");
    } else if number % 3 == 0 {
        println!("Fizz condition");
    } else {
        println!("nothing true");
    }
    

    let condition = true;

    let number2 = if condition { 6 } else { 5 };

    println!("number2: {number2}");

    //let number3 = if condition {5} else {"six"}; -> incompatible types
}
