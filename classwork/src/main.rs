fn main() {
    benefits_of_logical_entity();
}

fn benefits_of_logical_entity() {
    pub trait ShowInfo {
        fn show_info(&self) -> String;
    }

    pub struct UndergradStudent {
        pub name: String,
        pub major: String,
        pub gpa: f32,
    }

    pub struct GradStudent {
        pub name: String,
        pub major: String,
        pub gpa: f32,
        pub thesis: String,
    }

    impl ShowInfo for UndergradStudent {
        fn show_info(&self) -> String {
            format!(
                "Undergraduate: {}, Major: {}, GPA: {:.2}",
                self.name, self.major, self.gpa
            )
        }
    }

    impl ShowInfo for GradStudent {
        fn show_info(&self) -> String {
            format!(
                "Graduate: {}, Major: {}, GPA: {:.2}, Thesis: {}",
                self.name, self.major, self.gpa, self.thesis
            )
        }
    }

    pub struct Enrollment<T: ShowInfo> {
        pub students: Vec<T>,
    }

    impl<T: ShowInfo> Enrollment<T> {
        pub fn new() -> Self {
            Enrollment { students: Vec::new() }
        }

        pub fn add(&mut self, student: T) {
            self.students.push(student);
        }
    }

    impl<T: ShowInfo> ShowInfo for Enrollment<T> {
        fn show_info(&self) -> String {
            self.students
                .iter()
                .map(|s| s.show_info())
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    let u1 = UndergradStudent {
        name: "Alice".into(),
        major: "Computer Science".into(),
        gpa: 3.8,
    };

    let g1 = GradStudent {
        name: "Bob".into(),
        major: "Data Science".into(),
        gpa: 3.9,
        thesis: "Neural Networks".into(),
    };

    pub fn show_student_info(item: impl ShowInfo) {
        println!("{}", item.show_info());
    }

    show_student_info(u1);
    show_student_info(g1);

    let mut undergrad_enrollment = Enrollment::new();
    undergrad_enrollment.add(UndergradStudent {
        name: "Carol".into(),
        major: "Math".into(),
        gpa: 3.6,
    });

    let mut grad_enrollment = Enrollment::new();
    grad_enrollment.add(GradStudent {
        name: "Dave".into(),
        major: "Computer Engineering".into(),
        gpa: 4.0,
        thesis: "Machine Learning".into(),
    });

    println!("\n--- Enrollment Info ---");
    println!("{}", undergrad_enrollment.show_info());
    println!("{}", grad_enrollment.show_info());
}