use std::io;

fn main(){

// Employee basic salary is input through the keyboard. His dearness
// allowance is 40% of basic salary, and house rent allowance is 20% of
// basic salary. Write a program to calculate his gross salary.

    let mut salary = String::new();

    println!("Input salary");

    io::stdin()
        .read_line(&mut salary)
        .expect("Failed to get input");
    
    let salary : f32 = salary.trim().parse().expect("Please type a number!");

    let dearness_allowence = salary * (0.4);
    let rent_allowence = salary * (0.2);

    println!("Gross salary is: {}",salary+dearness_allowence+rent_allowence);

}