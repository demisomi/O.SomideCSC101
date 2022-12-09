// A program reads users input and performs the corresponding calculations.

use std::io;

fn calculator(user_selection: i32) {
    let mut user_input2 = String::new();
    let mut user_input3 = String::new();
    let mut user_input4 = String::new();

    
    if user_selection == 1 {
        println!("Enter the value height of the trapezium");
        io::stdin().read_line(&mut user_input2).expect("Not a valid string");
        let height:f64 = user_input2.trim().parse().expect("Not a valid number");

        println!("Enter the value for base 1 of the trapezium");
        io::stdin().read_line(&mut user_input3).expect("Not a valid string");
        let base_1:f64 = user_input3.trim().parse().expect("Not a valid number");

        println!("Enter the value for base 2 of the trapezium");
        io::stdin().read_line(&mut user_input4).expect("Not a valid string");
        let base_2:f64 = user_input4.trim().parse().expect("Not a valid number");

        let area:f64 = height/2.0 * (base_1 + base_2);
        println!("The area of the trapezium is {}", area);
    }

    else if user_selection == 2 {
        println!("Enter the value for diagonal 1 of the Rhombus");
        io::stdin().read_line(&mut user_input2).expect("Not a valid string");
        let diagonal_1:f64 = user_input2.trim().parse().expect("Not a valid number");

        println!("Enter the value for diagonal 2 of the Rhombus");
        io::stdin().read_line(&mut user_input3).expect("Not a valid string");
        let diagonal_2:f64 = user_input3.trim().parse().expect("Not a valid number");

        let area:f64 = 1.0/2.0 * diagonal_1 * diagonal_2;
        println!("The area of the Rhombus is {}", area);
    }

    else if user_selection == 3 {
        println!("Enter the value for the base of the parallelogram");
        io::stdin().read_line(&mut user_input2).expect("Not a valid string");
        let base:f64 = user_input2.trim().parse().expect("Not a valid number");

        println!("Enter the value for the altitude of the parallelogram");
        io::stdin().read_line(&mut user_input3).expect("Not a valid string");
        let altitude:f64 = user_input3.trim().parse().expect("Not a valid number");

        let area:f64 = base * altitude;
        println!("The area of the parallelogram is {}", area);
    }
    else if user_selection == 4 {
        println!("Enter the value for the length of the cube");
        io::stdin().read_line(&mut user_input2).expect("Not a valid string");
        let length:f64 = user_input2.trim().parse().expect("Not a valid number");
        
        let area:f64 = 6.0 * length * length;
        println!("The area of the cube is {}", area);
    }
    else if user_selection == 5 {
        println!("Enter the value for the radius of the cylinder");
        io::stdin().read_line(&mut user_input2).expect("Not a valid string");
        let radius:f64 = user_input2.trim().parse().expect("Not a valid number");

        println!("Enter the value for the height of the cylinder");
        io::stdin().read_line(&mut user_input3).expect("Not a valid string");
        let height:f64 = user_input3.trim().parse().expect("Not a valid number");

        let volume:f64 = 3.14 * radius * radius * height ;
        println!("The volume of the cylinder is {}", volume);
    }
}

fn main() {
    loop{
        println!("                               ");
        println!("This calculator helps you calculate the following");
        println!("                               ");
        println!("1. Area of a Trapezium");
        println!("2. Area of the rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("                               ");
        println!("To select what you want to calculate use \"1\" for area of a Trapezium");
        println!("To select what you want to calculate use \"2\" for area of the Rhombus");
        println!("To select what you want to calculate use \"3\" for area of Parallelogram");
        println!("To select what you want to calculate use \"4\" for area of Cube ");
        println!("To select what you want to calculate use \"5\" for volume of Cylinder");
    
        let mut user_input1 = String::new();
        io::stdin().read_line(&mut user_input1).expect("Not a valid string");
        let users_choice:i32= user_input1.trim().parse().expect("Not a valid number");
        if (users_choice != 1) && (users_choice != 2) && (users_choice != 3) && (users_choice != 4) && (users_choice != 5) {
            println!("Unrecognized input please input the right number");
            continue;
        }
        calculator(users_choice);
        break; 
    }    
}
