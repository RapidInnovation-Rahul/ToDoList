use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
use std::io;

pub struct User{
    pub pass: String,
    pub list: Vec<String>,
}

pub fn addTask(list: &mut Vec<String>){
    println!("Enter task name you want to add: ");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read your task!!!");

    list.push(task);
    println!("Your task added successfully");
    display(list);
}

pub fn delTask(list: &mut Vec<String>){
    println!("pls enter the task index you want to delete: ");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read index");
    let index = index.trim().parse().unwrap();
    list.remove(index);
    display(list);
}
pub fn empList(list: &mut Vec<String>){ 
    let ele = String::from("None");
    list.retain(|x| *x!= ele);
    display(list);
}
pub fn display(TDList: &mut Vec<String>){
    println!("your to do list is: {:?}",TDList);
    println!("To Modify your To_DO_List select a number from the given list");
    println!("1. Add Task \n2. Delete Task\n 3. Empty List\n4. Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read your input!!");
    let i = input.trim().parse().unwrap();
    match i{
        1 => addTask(TDList),
        2 => delTask(TDList),
        3 => empList(TDList),
        _ => exit(),
    }
}
pub fn exit(){return}