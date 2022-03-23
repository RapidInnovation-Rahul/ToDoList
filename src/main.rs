use std::io;
use std::collections::HashMap;
struct User { // user type
    pass: String , // password
    list: Vec<String> , // to_do list
}
fn main() {
    let mut userlist = HashMap::new(); // actual data_base for now
    let rahul = User{
        pass : String::from("rahul1999"),
        list : vec![String::from("task1"), String::from("Task2"), String::from("Task3")],
    };
    let ankit = User{
        pass: String::from("ankit1998"),
        list : vec![String::from("Task4"), String::from("Task5"), String::from("Task6")],
    };


    userlist.insert(String::from("rahul"), rahul); // adding details to data_base
    userlist.insert(String::from("ankit"), ankit);

    println!("Enter Your User_Name: "); // taking username input
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read your UserName!!");
    
    // making user_name.to_string()
    user_name = user_name.trim().to_string();
    let user = userlist.get(&user_name);

    // searching for the user_data
    match user{
        Some(user) =>{ 
            // if user could be found
            println!("Enter your password: ");
            let mut p = String::new(); // the entered password.
            io::stdin().read_line(&mut p).expect("Failed to read your password!!");

            p = p.trim().to_string();

            // if username and password is correct.
            if &p == &user.pass.to_string(){ 
                println!("welcome {}",user_name);
                println!("your Tasks for today: {:?}",user.list);
            }

            // if password is wrong
            else { 
                println!("The password you entered is Wrong!");
            }
        }
        None=>{
            println!("User does not exist!!");
            // println!("Do you want to sign in?");
        }
    }
}
