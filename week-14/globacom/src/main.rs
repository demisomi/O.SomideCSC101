use::std::io::Read;
use::std::io;

fn main() {
    println!("If you are an administrator please type 1");
    println!("If you are a project manager please type 2 ");
    println!("If you are an employee please type 3");
    println!("If you are a customer please type 4");
    println!("If you are a vendor please type 5");

    

    let mut user_input1 = String::new();
    io::stdin().read_line(& mut user_input1).expect("Not a valid String");
    let user_response:i32 = user_input1.trim().parse().expect("Not a valid Number");

    if user_response == 1 {
        let mut file = std::fs::File::open("globacom.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if user_response == 2 {
        let mut file = std::fs::File::open("project_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if user_response == 3{
        let mut file = std::fs::File::open("staff_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents)
    }
    else if user_response == 4 {
        let mut file = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents)
    }
    else {
        let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents)
    }

}
