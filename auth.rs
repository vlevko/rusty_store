//! This is a simplest possible authentication module.
//! The password is stored directly in the program as a
//! plain text constant.

/// Stores the password for working with the program.
const SECRET: &str = "password";

/// Prompts the user for a password and returns `true` if
/// it matches the stored SECRET value.
/// To exit the function, enter `x`, and it will return
/// `false`.
pub fn authorize() -> bool {
    let mut password = String::new();
    loop {
        println!("Enter password, or x to escape:");
        password.clear();
        std::io::stdin().read_line(&mut password).unwrap();
        password = password.trim().to_string();
        if password == "x" {
            return false;
        } else if password == SECRET {
            break;
        }
    }
    true
}
