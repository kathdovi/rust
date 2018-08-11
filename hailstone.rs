use std::io;
use std::io::Write;

fn main() {
    
    print!("Enter number: ");
    io::stdout().flush().unwrap();

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num = num.trim().parse::<u32>().expect("NaN");

    hailstone(num);
}

fn hailstone(mut num: u32) {
    let mut num_steps = 0;
    
    print!("{} ", num);
    while num > 1 {
        if num%2 == 0 {
            let new_num = num/2;
            print!("{} ", new_num);
            num = new_num;
        } else if num%2 == 1 {
            let new_num = (num*3) + 1;
            print!("{} ", new_num);
            num = new_num;
        }
        num_steps+=1;
    }
    println!();
    println!("This process took {} steps to reach 1", num_steps);
}
