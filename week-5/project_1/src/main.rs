// Rust program to roots of a quadratic equation

use std::io;

fn main() {
    let mut a:f32 = 0.0;
    let mut b:f32 = 0.0;
    let mut c:f32 = 0.0;
    
    let mut root_a:f32 = 0.0;
    let mut root_b:f32 = 0.0;

    let mut real_part:f32 = 0.0;
    let mut imag_part:f32 = 0.0;
    let mut disc:f32  = 0.0;
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    a = input1.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    b = input2.trim().parse().expect("Not a valid number");
    
    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    c = input3.trim().parse().expect("Not a valid number");

    if a == 0.0 || b == 0.0 || c == 0.0 
    {
        println!("Unable to determine roots");
        return;
    }
    else 
    {
        disc = b * b - 4.0 * a * c;
        if disc < 0.0 
        {
            println!("Not a real Root");
            real_part = -b / (2.0 * a);
            disc = disc.abs();
            imag_part = disc.sqrt() / (2.0 * a);
            println!("root1 = {}  +i {}", real_part, imag_part);
            println!("root2 = {}  -i {}", real_part, imag_part);
        }
        else if disc == 0.0
        {
            println!("The roots are real and equal");
            root_a = -b / (2.0 * a);
            root_b = root_a;
            println!("root1 = {}", root_a);
            println!("root2 = {}", root_b);
        }
        else if disc > 0.0 
        {
            println!("The roots are real and distinct");
            root_a = (-b + disc.sqrt()) / (2.0 * a);
            root_b = (-b - disc.sqrt()) / (2.0 * a);
            println!("root1 = {}  ", root_a);
            println!("root2 = {}  ", root_b);
        }
    }
}

