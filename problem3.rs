use std::io;

fn main(){
//If the marks obtained by a student in five different subjects are
// input through the keyboard, write a program to find out the
// aggregate marks and percentage marks obtained by the student.
// Assume that the maximum marks that can be obtained by a student
// in each subject is 100.

    let mut subject1 = String::new();
    let mut subject2 = String::new();
    let mut subject3 = String::new();
    let mut subject4 = String::new();
    let mut subject5 = String::new();
    
    println!("enter subject1 marks: ");
    io::stdin()
        .read_line(&mut subject1)
        .expect("Failed to read input");

    println!("enter subject2 marks: ");
    io::stdin()
        .read_line(&mut subject2)
        .expect("Failed to read input");
    
    println!("enter subject3 marks: ");
    io::stdin()
        .read_line(&mut subject3)
        .expect("Failed to read input");
        
    println!("enter subject4 marks: ");
    io::stdin()
        .read_line(&mut subject4)
        .expect("Failed to read input");
    
    println!("enter subject5 marks: ");
    io::stdin()
        .read_line(&mut subject5)
        .expect("Failed to read input"); 
        
    let subject1 : f32 = subject1.trim().parse().expect("please enter a number");
    let subject2 : f32 = subject2.trim().parse().expect("please enter a number");
    let subject3 : f32 = subject3.trim().parse().expect("please enter a number");
    let subject4 : f32 = subject4.trim().parse().expect("please enter a number");
    let subject5 : f32 = subject5.trim().parse().expect("please enter a number");

    let agregate = subject1+subject2+subject3+subject4+subject5;
    println!("agregate is : {}",agregate);

    println!("percentage is : {}",agregate/5.0)

}