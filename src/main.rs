use mongo::{create_student, Student};

fn main() {
    let student = Student{
        name: String::from("Kacper"),
        class: String::from("IT"),
        credit: 0,
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(create_student(student)).expect("Error has occured while creating a student.");
}