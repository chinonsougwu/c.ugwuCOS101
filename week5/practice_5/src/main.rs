
use std::io;

fn main()
{
    let mut input = String::new();

    println!("\nEnter your height (in centimeters):");
    io::stdin().read_line(&mut input).expect("not a valid string");
    let height:f32 = input.trim().parse().expect("not a valid number");

    if height >= 150.0 && height <=170.0{
        println!("You are an average height person");
    }
    else if height >= 170.0 && <=190.0
    {
        println!(" You are quite tall");
    }
    else if height < 150.0 && >100.0{
        println!("You are dwarf");
    }
    else{
        println!("You are either abmormally tall or short");
    }
}
