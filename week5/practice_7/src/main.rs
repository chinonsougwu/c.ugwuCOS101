use std::io;

fn main() {
 
   let mut input1 = String::new();

   println!("Enter a number");
   io::stdin().read_line(&mut input1).expect("not a string");
   let mut num:i32 = input1.trim().parse().expect("not a string");

    while num < 10 {

    println!("Inside loop number value is {}",num);
    num+=1;
    }

    println!("outside loop number value is {}",num);


}
