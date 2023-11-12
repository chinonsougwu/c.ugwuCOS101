use std::io;

fn main()
{
 
let p:i32= 3200;
let f:i32 = 3000;
let a:i32= 2500;
let e:i32 = 2000;
let w:i32 = 2500;
let mut input1 = String::new();

println!("Hello Welcome to Happy Resturant!");
println!("The following are on the Menu!");
println!();
println!("Press (P): For, poundo yam/Edinkaiko soup = N3200");
println!("Press (F): For, fried rice and Chicken = N3000");
println!("press (A): For, Amala and Ewedu soup = N2500");
println!("press (E): For, Eba and egusi soup = N2000");
println!("press (W): For, White rice and stew = N2500");
println!();
println!("So how much do you have");

io::stdin().read_line(&mut input1).expect("not a string");
let choice:i32 = input1.trim().parse().expect("input not a string");
println!("Also if your purchase exceeds N10,000 you will get a discount");
println!();

if choice == 2000
{
    println!("you can only afford 'eba and Egusi' ");
}
else if choice >2000 && choice <=2500 {
    println!("You can only afford 'eba and Egusi' and 'Amala and Ewedu soup','white rice and stew'");
}
else if choice >2000 && choice <= 3499{
    println!("You can only afford 'eba and Egusi','Amala and Ewedu soup','white rice and stew' and 'fried rice and chicken' ");
}
else if choice >3500 && choice <10000{
    println!("you can buy anyone but no discounts");
}
else {
    let m:i32 = (choice*5)/100;
    println!("your discount is {}  '5% applied'",m);
}














}
