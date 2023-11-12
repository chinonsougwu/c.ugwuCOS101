use std::io;

fn main()
{

let mut name = String::new();
let mut a = String::new();
let mut b = String::new();

println!("Welcome to the ANNUAL INCENTIVE Calculator");
println!();
println!("Whats your name:");    
io::stdin().read_line(&mut name).expect("not a number");
println!("Hello {} nice to meet you!",name);

println!("How old are you {}",name);

io::stdin().read_line(&mut a).expect("this is not a string");
let age:f32 = a.trim().parse().expect("this is not a string");

println!("{} , How many years of experience do you have?",name);
io::stdin().read_line(&mut b).expect("this is not a string");
let experience:f32 = b.trim().parse().expect("this is not a string");

if age >= 40.0 && experience > 1.0 {
    println!("{} Your incentive is N1,560,000.0",name);
}
else if age >=30.0 && age < 39.0 && experience > 1.0{
    println!("{} your incentive is N1,480,000.0",name);
}
else if age <28.0 && experience >1.0 {
    println!("{} your incentive is N1,300,000.0",name);
}
else if experience <1.0{
    println!("{} your incentive is N100,000",name);
}



}
