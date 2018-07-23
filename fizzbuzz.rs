use std::io;
use std::io::Write;

fn main() {
    println!("This program will print the fizzbuzz sequence within a range that you specify.");

    print!("Enter first number: ");
    io::stdout().flush().unwrap();

    let mut firstnum = String::new();

    io::stdin()
        .read_line(&mut firstnum)
        .expect("Failed to read line");

    let firstnum = firstnum.trim().parse::<u32>().expect("nan");

    print!("Enter last number: ");
    io::stdout().flush().unwrap();

    let mut secondnum = String::new();

    io::stdin().read_line(&mut secondnum)
        .expect("Failed to read line");

    let secondnum = secondnum.trim().parse::<u32>().expect("nan");

    run_loop(firstnum, secondnum);
}


fn run_loop(a: u32, b: u32) {
    for x in a..(b+1) {
      if x % 3 == 0 && x % 5 == 0 {
        println!("Fizzbuzz");
      } else if x % 3 == 0 {
        println!("Fizz");
      } else if x % 5 == 0 {
        println!("Buzz");
      } else {
        println!("{} ", x);
      }
    }
}
