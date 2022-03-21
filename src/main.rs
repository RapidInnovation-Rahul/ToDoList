use std::io;
// use std::fs;
use std::vec;
// use serde_json::{Result, Value};
// use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// #[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    password: String,
    todo: Vec<String>,
}
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Address {
//     street: String,
//     city: String,
//     country: String,
// }


// impl User {
//     fn is_exist(&self, username: String) -> bool{
//         let user = self.get(username);
//     }
// }

// fn is_exist(data: HashMap, username: String) -> bool{
//     // let team_name = String::from("user3");
//     let user = data.get(&username);
//     match user {
//         Some(x) => println!("Please enter your password"),
//         None => println!("Wrong username or user doesn't exist"),
//     }
// }

fn addUser(data: &mut HashMap<String, User>, username: String, password: String) {
    let user_data = User {
        password: password,
        todo: vec![],
    };
    data.insert(username, user_data);
}
fn main() {
    let mut data = HashMap::new();

    let user1 = User {
        password: String::from("password1"),
        todo: vec![String::from("todo1"), String::from("todo2")],
    };

    let user2 = User {
        password: String::from("password1"),
        todo: vec![String::from("todo1"), String::from("todo2")],
    };
    let user3 = User {
        password: String::from("password1"),
        todo: vec![String::from("todo1"), String::from("todo2")],
    };
    data.insert(String::from("user1"), user1);
    data.insert(String::from("user2"), user2);
    data.insert(String::from("user3"), user3);

    for (key, value) in &data {
        println!("{}: {:?}", key, value);
    }


    // let username = String::from("user4");
    // println!("your username: {}", &username);
    
    println!("Please enter username:- ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("failed to read the username");
    println!("your username: {}", &username);

    let user = data.get(&username.trim_right().to_string());
    // println!("{:?}", &user);

    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            println!("Please enter your password");
            let mut password = String::new();
            io::stdin().read_line(&mut password).expect("failed to read the username");
            let saved_password = &x.password;
            if &password.trim_right().to_string() == saved_password {
                println!("{:?}", &x.todo);
            }
            else {
                println!("Password is wrong");
            }
        },
        None => {
            println!("Wrong username or user doesn't exist");
            println!("Do you want to create a new user? press Y or N");
            let mut new_user = String::new();
            io::stdin().read_line(&mut new_user).expect("failed to read the username");

            if new_user.trim_right().to_lowercase() == "y" {
                println!("Please enter username:- ");
                let mut username = String::new();
                io::stdin().read_line(&mut username).expect("failed to read the username");
                println!("Please enter the password");
                let mut password = String::new();
                io::stdin().read_line(&mut password).expect("failed to read the username");
                addUser(&mut data, username.trim_right().to_string(), password.trim_right().to_string());
            }

            // for (key, value) in &data {
            //     println!("{}: {:?}", key, value);
            // }
        },
    }

    // if let Some()
    // println!("{:?}", user)

    // for (key, value) in &data {
    //     println!("{}: {}", key, value.password);
    // }
    // let mut username = String::new();

    // println!("Enter username:-");
    // io::stdin().read_line(&mut username).expect("failed to read the username");
    // println!("username is {}", username);


    // let json: serde_json::Value =
    //     serde_json::from_str(the_file).expect("JSON was not well-formatted");

    // let the_file = fs::read_to_string("data.json")
    //     .expect("Unable to read file");

    // let v: User = serde_json::from_str(&the_file)
    //     .expect("JSON does not have correct format.");
    
    // println!("data: {}", v.username)
}
