
use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let base:f32 = input1.trim().parse().expect("not a valid string");

    println!("Enter the Height: ");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let height:f32 = input2.trim().parse().expect("not a valid string");

    if base > 0.0{
        let area:f32 = (base * height)/ 2.0;
        println!("The area of the triangle is: {} for SURE!",area);
    }












}
