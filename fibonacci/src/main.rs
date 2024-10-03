use std::io;
fn main() {
    println!("Enter the limit of fibonacci series :");
    let mut range : String = String::new();
    io::stdin().read_line(&mut range).expect("Unable to read range input");
    let range : u8 = range.trim().parse().unwrap();
    let mut i : u8 = 0;
    let mut a : u32 = 0;
    let mut b : u32 = 1;
    let mut c : u32;
    println!("basic value = 0\nbasic value = 1");
    while i < range{
        c = a+b;
        a = b;
        b = c;
        i+=1;
        println!("current value = {}",c);
    }
}