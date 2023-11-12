use std::io;

fn main() {

let mut input1 = String::new();
let mut input2 = String::new();

println!("Enter your lower bound");
io::stdin().read_line(&mut input1).expect("not a valid string");
let lower_bound:i32 = input1.trim().parse().expect("not a valid string");

println!("Enter upper bound");
io::stdin().read_line(&mut input2).expect("not a valid string");
let upper_bound:i32 = input2.trim().parse().expect("not a valid string");

for x in lower_bound..upper_bound{
    println!("count level is {}",x);
}




}
