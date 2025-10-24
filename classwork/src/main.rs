use std::process::Command;
use std::io::{self, Write};
use std::fs::File;

fn reading_from_console(message:&String ) -> String {
    let mut buffer = String::new();

    print!("");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();
    return name;
}


fn read_file_linux(filename : String) {
    let output = Command::new("cat")
        .arg(&filename)
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn create_and_write_to_file(filename: &String) {
    let mut file = File::create(&filename).unwrap();
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
}


fn main(){
    while true == true{
        let message = "What do you want to do today? Type 1 to create file, type 2 to read file, type 3 if you want to finish the program:";
        println!("{}", message);
        let choice = reading_from_console(&"Type the numer".to_string()).parse().unwrap();

        match choice{
            1 => {
                let message = "What file to create?".to_string();
                let filename = reading_from_console(&message);
                create_and_write_to_file(&filename);
            },

            2 => {
                let message = "What file to open?".to_string();
                let filename = reading_from_console(&message); // accept from the user
                read_file_linux(filename);

            }
            3 => break,
            _ => {},
        }
        // ask the user if he wants to create the file
        // and write inside or he wants to read from existent file

        // create a file or write inside of it
        // accept everything from the console
        // filename he want to create and actual content
        // utilize linux touch and echo commands


        //or he wants to read from existent file

        // accept from the user file he wants to open
    }
}