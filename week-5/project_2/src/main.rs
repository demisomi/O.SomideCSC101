// program to determine annual incentive of employees.

use std::io;

fn main() {

    loop {
        let mut input_age = String::new();
        let mut input_experience = String::new();

        println!("Do you have experience type either \"yes\" or \"no\" to answer");
        io::stdin().read_line(&mut input_experience).expect("Not a vaild string");
        let experience = input_experience.trim().to_lowercase();

        if (experience != "yes") && (experience != "no") {
            println!("Please answer with either \"yes\" or \"no\"");
            continue;
        }
        else if experience == "no" {
            println!("Your annual incentive is N100,000");
            break;   
        }

        println!("Enter your age");
        io::stdin().read_line(&mut input_age).expect("Not a valid string");
        let age:i32 = input_age.trim().parse().expect("Not a valid number");

        if (experience == "yes") && (age >= 40) {
            println!("Your annual incentive is N1,560,000");
            break;
        }

        else if (experience == "yes") && (age >= 30) && (age < 40) {
            println!("Your annual incentive is N1,480,000");
            break;
        }

        else if (experience == "yes") && (age < 28) {
            println!("Your annual incentive is N1,300,000");
            break;
        }
    }       
}
