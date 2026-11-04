mod classroom;

fn main() {
    // Our function takes ownership of each field
    // so if they were variables, we could not use them anymore
    let s1 = classroom::Student::new("Andrew".to_string(), 16, 4.9);
    let s2 = classroom::Student::new("Matthew".to_string(), 13, 3.4);
    let s3 = classroom::Student::new("Steve".to_string(), 14, 2.5);

    // Accessing the data manually is an error, because the field isn't public
    // println!("{}", s1.name);

    // Our function takes ownership of the students, so we can't use them anymore
    let class = classroom::Classroom::new(s1, s2, s3);
    println!("{}", class.calculate_avg_grade());
}
