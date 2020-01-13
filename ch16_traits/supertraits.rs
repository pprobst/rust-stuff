/*
 * Rust doesn't have "inheritance", but you can define a trait as being a superset of 
 * another trait. For example:
 */

trait Person {
    fn name(&self) -> String;
}

// Student is a supertrait of Person.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_lang(&self) -> String;
}

// CompSciStudent is a supertrait of both Programmer and Student.
// Implementing CompSciStudent requires you to impl both Subtraits.
trait CompSciStudent: Programmer + Student {
    fn git_user(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_user()
    )
}

fn main() {}
