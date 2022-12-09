use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let stu_name = vec!["oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_num = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering","Mechanical Engineering"];
    let level = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("Student Management System.txt").expect("file creation failed");
    file.write_all("    PAU SMIS   \n".as_bytes()).expect("write failed");

    
    for i in 0..stu_name.len() {
        let mut student_file = OpenOptions::new().append(true).open("Student Management System.txt").expect("cannot open file");
        let stu_data = format!("\n{}  {}  {}  {}", stu_name[i], matric_num[i], department[i], level[i]);
        student_file.write_all(stu_data.as_bytes()).expect("write failed");
    }

    println!("Data has been written");
}
