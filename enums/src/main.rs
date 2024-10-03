use std::io;

enum Day{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let mut number : String = String::new();
    println!("***** BASIC ENUM AND STUFF BY AYUSH *****");
    println!("Choose a number between 1 and 7 \'inclusive\'");
    io::stdin().read_line(&mut number).expect("Unable to read number input");
    let day : Day;
    let number : u8 = number.trim().parse().expect("Unable to convert number to uint type");
    if (number < 1) || (number > 7){
        println!("You serious...");
        return;
    }
    else{
        match number{
            1 => {
                day = Day::Monday;
            },
            2 => {
                day = Day::Tuesday;
            },
            3 => {
                day = Day::Wednesday;
            },
            4 => {
                day = Day::Thursday;
            },
            5 => {
                day = Day::Friday;
            },
            6 => {
                day = Day::Saturday;
            },
            7 => {
                day = Day::Sunday;
            },
            _ => {
                println!("BYe Bye ");
                return;
            }
        }
    }
    match day{
        Day::Saturday => {
            println!("You choose day off . ENJOY~!!");
        },
        Day::Sunday => {
            println!("You choose day off . ENJOY~!!");
        },
        _ =>{
            println!("You have to work :)))");
        },
    }
}
