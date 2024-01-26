use std::io;
use std::io::Read;
use std::fs::File;


fn main() {
    println!("Welcome to GLOBACOM");
    println!("Dear User, WHERE DO YOU OPERATE \nAdministrator?\nProject manager?\nemployee?\nCustomers?\nVendor? ");
    println!("Please select the first letter of your answer")
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Not a valid string");
    let answer = answer.trim();

    match answer {
        "a" => print_file_content("globacon_db.sql"),
        "p" => print_file_content("project_tb.sql"),
        "e" => print_file_content("staff_tb.sql"),
        "c" => print_file_content("customertable_tb.sql"),
        "v" => print_file_content("dataplantable_tb.sql"),
        _ => println!("No access to info"),
    }

}
fn print_file_content(file_name: &str) {1``
    if let Ok(mut file) = File::open(file_name) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        print!("{}", contents);
    } else {
        println!("Error opening file");
    }
}















}
