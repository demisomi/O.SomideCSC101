use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Nigerian Brewries.txt").expect("create failed");
    file.write_all("Lager Stout Non-Alcoholic\n".as_bytes()).expect("write failed");
    file.write_all("33 Export Legend Maltina\n".as_bytes()).expect("write failed");
    file.write_all("Desperados Turbo King Amstel Malta\n".as_bytes()).expect("write failed");
    file.write_all("Goldberg Williams Malta Gold\n".as_bytes()).expect("write failed");
    file.write_all("Gulder            Fayrouz\n".as_bytes()).expect("write failed");
    file.write_all("Heineken                 \n".as_bytes()).expect("write failed");
    file.write_all("star                      \n".as_bytes()).expect("write failed");


    println!("\nData written to file. ");

}
