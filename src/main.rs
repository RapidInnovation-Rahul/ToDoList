use std::io;
use std::fs::File;
use serde_json::Result;
use std::collections::HashMap;
use std::io::BufReader;

mod todo;
use crate::todo::User;
use crate::todo::{addTask, delTask, empList, display, exit};

fn read_dataset(path: String) -> Result<HashMap<String, User>>{
    let file = File::open(path).expect("Failed to load file");
    let reader = BufReader::new(file);
    let dataset: HashMap<String, User> = serde_json::from_reader(reader)?;

    Ok(dataset);
}

fn save_dataset(path: String, dataset: &HashMap<String, User>){
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, &dataset).expect("Failed to save data!");
}



#[allow(unused_mut)]
fn main() {

    let mut userlist = read_dataset(String::from("dataset.json")).unwrap();
    // let mut userlist = HashMap::new(); // actual data_base for now
    // let mut rahul = User{
    //     pass : String::from("rahul1999"),
    //     list : vec![String::from("task1"), String::from("Task2"), String::from("Task3")],
    // };
    // let mut ankit = User{
    //     pass: String::from("ankit1998"),
    //     list : vec![String::from("Task4"), String::from("Task5"), String::from("Task6")],
    // };
    // userlist.insert(String::from("rahul"), rahul); // adding details to data_base
    // userlist.insert(String::from("ankit"), ankit);

    println!("Enter Your User_Name: "); // taking username input
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read your UserName!!");
    
    // making user_name.to_string()
    let user_name = user_name.trim().to_string();
    let userName = &mut userlist.get_mut(&user_name);

    // searching for the user_data
    match userName{
        Some(user) =>{ 
            // if user could be found
            println!("Enter your password: ");
            let mut p = String::new(); // the entered password.
            io::stdin().read_line(&mut p).expect("Failed to read your password!!");

            p = p.trim().to_string();

            // if username and password is correct.
            let saved_pass = &user.pass.to_string();
            if &p == saved_pass{ 
                println!("welcome {}",user_name);
                display(&mut user.list);
                
            }

            // if password is wrong
            else { 
                while &p !=  saved_pass{ // incorrect pin can be entered only 3 times.
                    println!("The password you entered is Wrong!");
                    println!("Try again! : ");
                    let mut pass = String::new();
                    io::stdin().read_line(&mut pass).expect("Failed to read password!!");
                    pass = pass.trim().to_string();

                    // if password got correct in the loop
                    if &pass == saved_pass{
                        display(&mut user.list);
                        break;
                    };
                }
            }
        }
        None=>{ // if the user does not exist..
            println!("User does not exist!!");
            println!("Do you want to sign in with this UserName? press: Y");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).expect("Failed to read your ans!!!");
            ans = ans.trim().to_string();

            if ans == "y" || ans == "Y"{ // if user want to sign up..
                let new_user = &user_name;
                signUp(&mut userlist, new_user.to_string());
            }
            else{
                exit();
            }
        }
    }
}


#[allow(unused_mut)]
fn signUp(data: &mut HashMap<String,User>,userName: String){
    println!("Enter a password for your profile: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read your input!!");

    let mut user_data = User{
        pass : password,
        list : vec![],
    };
    data.insert(userName, user_data);
    println!("New User added successfuly");
}

