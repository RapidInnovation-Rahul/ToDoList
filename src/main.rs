use std::io;
// use std::fs;
use std::vec;
// use serde_json::{Result, Value};
// use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use serde::__private::de::Content;

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


fn addUser(data: &mut HashMap<String, User>, username: String, password: String) {
    let user_data = User {
        password: password,
        todo: vec![],
    };
    data.insert(username, user_data);
}

fn addContent(data: &mut HashMap<String, User>, username: String, content: String){
    let user = data.get(&username).unwrap();
    let mut todo = &mut user.todo.clone();
    todo.push(content);
    let user_data = User {
        password: user.password.clone(),
        todo: todo.to_vec(),
    };
    data.insert(username, user_data);
    println!("Your new todo list: {:?}", &todo);
}

fn deleteContent(data: &mut HashMap<String, User>, username: String, idx: usize){
    let user = data.get(&username).unwrap();
    let mut todo = &mut user.todo.clone();
    todo.remove(idx);
    let user_data = User {
        password: user.password.clone(),
        todo: todo.to_vec(),
    };
    data.insert(username, user_data);
    println!("Your new todo list: {:?}", &todo);
}

fn editContent(data: &mut HashMap<String, User>, username: String, idx: usize, content: String){
    let user = data.get(&username).unwrap();
    let mut todo = &mut user.todo.clone();
    std::mem::replace(&mut todo[idx], content);
    let user_data = User {
        password: user.password.clone(),
        todo: todo.to_vec(),
    };
    data.insert(username, user_data);
    println!("Your new todo list: {:?}", &todo);
}


fn emptyTodo(data: &mut HashMap<String, User>, username: String){
    let user_data = User {
        password: username.clone(),
        todo: vec![],
    };
    data.insert(username, user_data);
    println!("Your todo list is empty!");
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
    io::stdin().read_line(&mut username).expect("failed to read the data");
    println!("your username: {}", &username);

    username = username.trim_end().to_string();
    let user = data.get(&username);
    // println!("{:?}", &user);

    // Pattern match to retrieve the value
    match user {
        Some(x) => {
            println!("Please enter your password");
            let mut password = String::new();
            io::stdin().read_line(&mut password).expect("failed to read the data");
            let saved_password = &x.password;
            
            if &password.trim_end().to_string() == saved_password {
                println!("Here is your todo list: {:?}", &x.todo);
                println!("What do you want to with list now? add?, delete?, edit?, empty?");
                let mut action = String::new();
                io::stdin().read_line(&mut action).expect("failed to read the data");
                
                if action.trim_end().to_lowercase() == "add" {
                    println!("Please add the content!");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("failed to read the data");
                    content = content.trim_end().to_string();
                    addContent(&mut data, username, content);
                }
                else if action.trim_end().to_lowercase() == "delete"{
                    println!("Here is your current todo list: {:?}", &x.todo);
                    println!("Please enter the number which you want to delete!");
                    let mut idx = String::new();
                    io::stdin().read_line(&mut idx).expect("failed to read the data");
                    idx = idx.trim_end().to_string();
                    let idx = idx.parse::<i32>().unwrap() - 1;
                    let idx = idx.try_into().unwrap();
                    deleteContent(&mut data, username, idx);
                }
                else if action.trim_end().to_lowercase() == "edit"{
                    println!("Here is your current todo list: {:?}", &x.todo);
                    println!("Please enter the number which you want to edit!");
                    let mut idx = String::new();
                    io::stdin().read_line(&mut idx).expect("failed to read the data");
                    idx = idx.trim_end().to_string();
                    let idx = idx.parse::<i32>().unwrap() - 1;
                    let idx = idx.try_into().unwrap();
                    println!("Please enter the new todo!");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("failed to read the data");
                    content = content.trim_end().to_string();
                    editContent(&mut data, username, idx, content);
                }
                else if action.trim_end().to_lowercase() == "empty"{
                    println!("Here is your current todo list: {:?}", &x.todo);
                    emptyTodo(&mut data, username);
                }
                else{
                    println!("Please select the right opration!");
                }
            }

            else {
                println!("Password is wrong");
            }
        },
        None => {
            println!("Wrong username or user doesn't exist");
            println!("Do you want to create a new user? press Y or N");
            let mut new_user = String::new();
            io::stdin().read_line(&mut new_user).expect("failed to read the data");

            if new_user.trim_end().to_lowercase() == "y" {
                println!("Please enter username:- ");
                let mut username = String::new();
                io::stdin().read_line(&mut username).expect("failed to read the data");
                println!("Please enter the password");
                let mut password = String::new();
                io::stdin().read_line(&mut password).expect("failed to read the data");
                addUser(&mut data, username.trim_end().to_string(), password.trim_end().to_string());
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
