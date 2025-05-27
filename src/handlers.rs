use crate::get_input;
use crate::models::account::Account;
use crate::models::user::User;
use std::collections::HashMap;

pub fn create_account(users: &mut HashMap<String, User>) {
    println!("\nPlease enter your name:");
    let mut name = String::new();
    get_input(&mut name);

    println!("\nPlease enter your username:");
    let mut username = String::new();
    get_input(&mut username);

    let new_user = User::new(name, username, users);
    users.insert(new_user.username.clone(), new_user.clone());

    println!("\nAccount created successfully!");
    println!("Username: {}", new_user.username);
    println!("Account Number: {}", new_user.account_number);
}

pub fn login_account(users: &mut HashMap<String, User>) {
    println!("\nPlease enter your username:");
    let mut username = String::new();
    get_input(&mut username);

    if let Some(user) = users.get(&username) {
        println!("\nWelcome back, {}!", user.name);
        println!("Account Number: {}", user.account_number);
    } else {
        println!("\nUser not found!");
    }
}

pub fn check_balance(users: &mut HashMap<String, User>) {
    println!("\nPlease enter your username:");
    let mut username = String::new();
    get_input(&mut username);

    if let Some(user) = users.get(&username) {
        let account = Account::new(user.clone());
        println!("\nAccount Details:");
        println!("Name: {}", user.name);
        println!("Account Number: {}", user.account_number);
        println!("Current Balance: ${}", account.get_balance());
    } else {
        println!("\nUser not found!");
    }
}

pub fn exit() {
    println!("\nThank you for using ABC Bank. Goodbye!");
    std::process::exit(0);
}
