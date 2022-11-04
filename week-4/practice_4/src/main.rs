fn main() {
    let fullname = "Chidubem John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science". to_string();
    // push string 
    school.push_str(" and Technology");
    
    println!("My name is {}", fullname);
    // check length
    println!("My name is: {}", fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);


    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}",school);
}
