use std::io;

struct Person{
    name : String,
    age : u8,
    height : f32
}

fn main() {
    println!("***** BASIC STRUCTS AND STUFF BY AYUSH *****");
    let mut choice : String;
    let mut p : Person = Person { name: String::from("John Doe"), age: 31, height: 187.5 };
    loop{
        println!("0 = Exit");
        println!("1 = Add a person");
        println!("2 = View the struct ");
        choice = String::new();
        io::stdin().read_line(&mut choice).expect("Unable to read choice input");
        let choice : u8 = choice.trim().parse().unwrap();
        if choice == 0{
            println!("Thankyou~");
            break;
        }
        else if choice == 1{
            println!("Enter the name of the person : - ");
            let mut name : String = String::new();
            io::stdin().read_line(&mut name).expect("Unable to name input");
            name = name.trim().to_string();
            println!("Enter the age of the person : - ");
            let mut age : String = String::new();
            io::stdin().read_line(&mut age).expect("Unable to name input");
            age = age.trim().to_string();
            let age : u8 = age.parse().unwrap();
            println!("Enter the height of the person : - ");
            let mut height : String = String::new();
            io::stdin().read_line(&mut height).expect("Unable to name input");
            height = height.trim().to_string();
            let height : f32 = height.parse().unwrap();
            p = Person{
                name : name,
                age: age,
                height : height,
            };
            println!("Person added succesfully");
        }
        else if choice == 2{
            println!("Name = {}",p.name);
            println!("Age = {}",p.age);
            println!("Height = {}",p.height);
        }
        else {
            println!("Enter a valid option");
        }
    }
}
