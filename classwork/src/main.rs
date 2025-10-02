//Student

// Struct responsible for Data
struct Student {
    major: String, 
    name: String, 

}

impl Student{
    fn new(name: String, major: String) -> Student {
        Student { name, major }
    }

    fn get_major(&self) -> &String {
        &self.major
    }

    fn set_major(&mut self, new_major: String) {
        self.major = new_major;
    }
}

fn main() {
   let mut my_student = Student::new(
        "Andres Sierra Cantu".to_string(),
        "Computer Engineering".to_string(),
    );

    // Print initial major using getter
    println!("Student: {}", my_student.name);
    println!("Initial Major: {}", my_student.get_major());

    // Change the major using setter
    my_student.set_major("Electrical Engineering".to_string());

    // Print updated major
    println!("Updated Major: {}", my_student.get_major());
}
