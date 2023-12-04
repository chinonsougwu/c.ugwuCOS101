use std::io;
let department = computer_science;

fn main() {
   
    println!("\n Student Information Manangement System!");
    
    // input name
    println!("\nPlease enter your name:");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("failed to read input");

    println!("Your name is: {} right?",name);

    //input age
    println!("\nEnter your age");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("failed input");
    let age:i32 = age.trim().parse().expect("input not an integer");
    println!("Your age is {}",age);   
if name == department && age>19
{
    println!(" you are gayyy");
}
}
