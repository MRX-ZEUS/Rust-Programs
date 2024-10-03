#![allow(unused)]
use std::io::{self, Read, Write};
use std::fs::{File,OpenOptions};
fn main() {
    println!("***** FILE IO OPERATION *****");
    let mut choice : String = String::new();
    println!("1 = Write to a file");
    println!("2 = Read from a file");
    println!("3 = Create a file ");
    println!("4 = Append to a file ");
    io::stdin().read_line(&mut choice).expect("Unable to read choice");
    let choice : u8 = choice.trim().parse().unwrap();
    if choice == 1{
        println!("Enter the name of the file you want to write :- ");
        let mut filename :String = String::new();
        io::stdin().read_line(&mut filename).expect("Unable to read file name input");
        println!("Enter the content to write to file (\"Warning : This will erase existing data\")");
        let mut fc : String = String::new();
        io::stdin().read_line(&mut fc).expect("Unable to read file input!");
        let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(false)
        .open(filename){
            Ok(file) => file,
            Err(e) =>{
                eprintln!("Error in opening writable file = {}",e);
                return;
            }
        };
        match file.write_all(fc.as_bytes()){
            Ok(_) => println!("File writing completed "),
            Err(e) => {
                eprintln!("Error in writing file = {}",e);
                return;
            }
        };
    }
    else if choice == 2{
        println!("Enter the name of the file you want to read from :- ");
        let mut filename :String = String::new();
        io::stdin().read_line(&mut filename).expect("Unable to read file name input");
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e) =>{
                eprintln!("Error = {}",e);
                return;
            }
        };
        let mut fc : String = String::new();
        match file.read_to_string(&mut fc){
            Ok(_) => println!("File Content : {}",fc),
            Err(e) => {
                eprintln!("Error in reading file = {}",e);
                return;
            }
        };
    }
    else if choice == 3{
        println!("Enter the name of the file you want to create along with extension :- ");
        let mut filename :String = String::new();
        io::stdin().read_line(&mut filename).expect("Unable to read file name input");
        let file = match File::create(filename) {
            Ok(_) => println!("File created successfully"),
            Err(e) => {
                eprintln!("Error in creating file = {} ",e);
                return;
            }
        };
    }
    else if choice == 4{
        println!("Enter the name of the file you want to append to :- ");
        let mut filename :String = String::new();
        io::stdin().read_line(&mut filename).expect("Unable to read file name input");
        println!("Enter the content to write to file (\"Warning : This not will erase existing data\")");
        let mut fc : String = String::new();
        io::stdin().read_line(&mut fc).expect("Unable to read file input!");
        let mut file = match OpenOptions::new().write(true).append(true).create(false).open(filename){
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error = {}",e);
                return;
            }
        };
        match file.write_all(fc.as_bytes()){
            Ok(_) => println!("Content appended to file succesfully"),
            Err(e) => {
                eprintln!("Error in appending to file = {}",e);
                return;
            }
        }
    }
    else{
        println!("Enter a valid option");
    }
}
