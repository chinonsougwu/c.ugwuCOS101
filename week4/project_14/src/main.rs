
use std::io;
fn main()
{
 
println!("Welcome to The speed Calculator");
println!();
    let mut distance = String::new();
    let mut time = String::new();

    println!("What is the distance the car travelled: ");
    io::stdin().read_line(&mut distance).expect("not a valid string");
    let a:f32 =distance.trim().parse().expect("Not a valid number");

    println!("How many hours did it take: ");
    io::stdin().read_line(&mut time).expect("not a valid string");
    let b:f32 =time.trim().parse().expect("Not a valid number");

    let s:f32 =a/b;
    println!("The speed is: {}",s);
    






}
