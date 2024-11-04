//! This program is made for demonstration purposes.
//! It is a simple Inventory Management System.
mod auth;
mod inventory;

use inventory::Inventory;

/// Entry point to the program. Displays the main menu and
/// prompts the user for an option number to continue. To 
/// exit the program, enter `x`.
fn main() {
    if !auth::authorize() {
        return;
    }

    let mut inventory = Inventory::new();

    let mut feature = String::new();
    loop {
        println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
        println!("Enter feature number to go to, or x to escape:");
        println!("Inventory Management  1");
        println!("Sales Management      2");
        println!("Purchase Management   3");
        println!("Reporting             4");
        feature.clear();
        std::io::stdin().read_line(&mut feature).unwrap();
        feature = feature.trim().to_string();
        if feature == "x" {
            return;
        }
        match &feature[..] {
            "1" => inventory::inventory_handler(&mut inventory),
            "2" => inventory::sales_handler(&mut inventory),
            "3" => inventory::purchase_handler(&mut inventory),
            "4" => inventory::report_handler(&mut inventory),
            _ => (),
        }
    }
}
