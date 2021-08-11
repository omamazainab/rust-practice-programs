use std::io;

fn main(){
// The distance between two cities (in km.) is input through the
// keyboard. Write a program to convert and print this distance in
// meters, feet, inches and centimeters.

    let mut distance = String::new();

    println!("Enter distance in kilometers ");

    io::stdin()
        .read_line(&mut distance)
        .expect("Failed to get the input");

    let distance : f32 = distance.trim().parse().expect("Please input a number");

    let meters = distance * 1000.0;
    let feet = distance * 3280.84;
    let inches = distance * 39370.1;
    let centimeters = meters * 100.0;
    
    println!("{} meters",meters);
    println!("{} feet",feet);
    println!("{} inches",inches);
    println!("{} centimeters", centimeters);

}