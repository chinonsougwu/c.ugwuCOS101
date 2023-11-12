use std::io;

fn main() 
{
 
println!("Hello,");
println!();
println!("Welcome to the Quadratic Calculator");
println!();

let mut root1 = String::new();
let mut root2 = String::new();
let mut root3 = String::new();

println!("Enter the first number: ");
io::stdin().read_line(&mut root1).expect("this is not a string");
let a:i32 =root1.trim().parse().expect("this is not a string");

println!("Enter the Second number: ");
io::stdin().read_line(&mut root2).expect("this is not a string");
let b:i32 =root2.trim().parse().expect("This is not a string");

println!("Enter the third number: ");
io::stdin().read_line(&mut root3).expect("this is not a string");
let c:i32 =root3.trim().parse().expect("this is not a string");

let discriminant = (b+b) - (4*a*c);

if discriminant > 0{
	println!("The equation has exactly 2 real distinct roots");
}
else if discriminant == 0{
	println!("The equation has exactly 1 real root");
}
else if discriminant < 0{
	println!("The equation has no real roots");
}
















}
