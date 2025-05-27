use std::collections::HashMap;
mod handlers;
mod models;

use handlers::{check_balance, create_account, exit, login_account};
use models::user::User;
use rand::Rng;

#[derive(Debug, Clone)]
#[allow(unused)]
struct Account {
    id: u32,
    balance: i32,
    account_number: i32,
    holder: User,
}

impl User {
    fn new(name: String, username: String, users: &mut HashMap<String, User>) -> User {
        let mut rng = rand::rng();
        let account_number: u32 = rng.random_range(10_000_000..=99_999_999);

        let constructed_username = username.replace(" ", "_").to_lowercase();

        if users.contains_key(&constructed_username) {
            println!("Username already exists");
            users.get(&constructed_username).unwrap().clone()
        } else {
            User {
                name,
                account_number,
                username: constructed_username,
            }
        }
    }

    // fn get_user(&self) -> &Self {
    //     return &self;
    // }
}

#[allow(unused)]
fn main() {
    let mut users: HashMap<String, User> = HashMap::new();

    println!("------------Hello, Welcome to ABC Bank------------");
    loop {
        option_input(&mut users);
    }
}

pub fn get_input(s: &mut String) {
    use std::io::{Write, stdin, stdout};

    let _ = stdout().flush();
    stdin()
        .read_line(s)
        .expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
}

fn option_input(users: &mut HashMap<String, User>) {
    println!("\n\nPlease select your option:");

    let options = [
        "Create a new account",
        "Login to an existing account",
        "Check balance",
        "Exit",
    ];

    println!("--------------------------------");
    for (index, option) in options.iter().enumerate() {
        println!("{}. {}", index + 1, option);
    }
    println!("--------------------------------");

    println!("\nEnter your option:");
    let mut option = String::new();
    get_input(&mut option);

    match option.trim() {
        "1" => create_account(users),
        "2" => login_account(users),
        "3" => check_balance(users),
        "4" => exit(),
        _ => println!("Invalid option"),
    }
}
