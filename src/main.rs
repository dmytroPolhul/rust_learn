use std::io;

fn main() {
    println!("Please input sequence numer:");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("It is not a number");

    let mut result:i32;
    let mut a = 1;
    let mut b = 1;

    for num in (3..number).rev() {
       result = a+b;
        a=b;
        b=result;

        if num == 3 {
            println!("Number {number} of fibonacci sequence is {result}")
        }
    }

}
