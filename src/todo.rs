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
    println!("your new to do list is: ");
    // display(TDList: &Vec<String>)
}

pub fn delTask(list: &mut Vec<String>){
    println!("pls enter the task index you want to delete: ");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read index");
    let index = index.trim().parse().unwrap();
    list.remove(index);
    println!("your new to do list is: ");
}
pub fn empList(list: &mut Vec<String>){ 
    let ele = String::from("None");
    list.retain(|x| *x!= ele);
    println!("Your updated todo list is: {:?}", list);
}
pub fn exit(){return}