use std::io;

fn main() {
    // let mut
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant value is: {}",THREE_HOURS_IN_SECONDS);
    //shadowed
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    //parse string to number
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Parsed value:{}",guess);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    if index > a.len() -1 {
        println!("Out of index :{}",index);
        return;
    }
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}