#[derive(Debug)]
pub struct Student {
    // Since our struct fields aren't public, they can only be
    // accessed through our methods
    name: String,
    age: u8,
    grade: f64,
}

impl Student {
    pub fn new(name: String, age: u8, grade: f64) -> Student {
        Student {
            name,
            age,
            grade,
        }
    }
}

#[derive(Debug)]
pub struct Classroom {
    student1: Student,
    student2: Student,
    student3: Student,
}

impl Classroom {
    pub fn new(s1: Student, s2: Student, s3: Student) -> Classroom {
        Classroom {
            student1: s1,
            student2: s2,
            student3: s3,
        }
    }

    pub fn display_students(&self) {
        println!("{self:#?}");
    }

    // Returns true if the string matches any of the students in the classroom
    //
    // NOTE: probably not the most elegant way to go about it, also needs to consider
    // handling letter capitalization and etc.
    pub fn find_student(&self, student: String) -> bool {
        if student == self.student1.name || student == self.student2.name ||
        student == self.student3.name {
            true
        } else {
            false
        }
    }

    pub fn calculate_avg_grade(&self) -> f64 {
        (self.student1.grade + self.student2.grade + self.student3.grade) / 3.0
    }
}
