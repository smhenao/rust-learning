use std::io::{self, Write};

mod classroom;

fn create_class() -> classroom::Classroom {
    // Our function takes ownership of each field
    // so if they were variables, we could not use them anymore
    let s1 = classroom::Student::new("Andrew".to_string(), 16, 4.9);
    let s2 = classroom::Student::new("Matthew".to_string(), 13, 3.4);
    let s3 = classroom::Student::new("Steve".to_string(), 14, 5.0);

    // Accessing this data manually is an error, because the field isn't public
    // println!("{}", s1.grade);

    // Our function takes ownership of the students, so we can't use them anymore
    classroom::Classroom::new(s1, s2, s3)
}

fn main() {
    let class = create_class();

    println!("Welcome to the student management system");

    loop {
        println!("
==== Student Management ====

1. Display students
2. Find student
3. Calculate average grade
4. Find best student
5. Exit
");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        let option: u32 = option.trim().parse().expect("Please input a valid number");

        match option {
            1 => class.display_students(),

            2 => {
                print!("Input the student name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                // trim to get rid of the newline
                if class.find_student(name.trim().to_string()) == true {
                    println!("Student found!");
                } else {
                    println!("Student not found");
                }
            },

            3 => println!("The class average is: {}", class.calculate_avg_grade()),

            4 => println!("The student with the best grade is: {}",
                class.find_best_student().name),

            5 => break,

            _ => println!("Please pick a valid choice"),
        }
    }
}
