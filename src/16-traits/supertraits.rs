// Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait.
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}


struct CompSciStudentType {
  name: String,
  university: String,
  fav_language: &'static str,
  git_username: &'static str
}

impl CompSciStudent for CompSciStudentType {
  fn git_username(&self) -> String {
    self.git_username.to_string()
  }
}

impl Programmer for CompSciStudentType {
  fn fav_language(&self) -> String {
    self.fav_language.to_string()
  }
}

impl Student for CompSciStudentType {
  fn university(&self) -> String {
    self.university.clone()
  }
}

impl Person for CompSciStudentType {
  fn name(&self) -> String {
    self.name.clone()
  }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
  let student = CompSciStudentType {
    name: "Christophe".to_owned(),
    university: "LR University".to_string(),
    fav_language: "Rust",
    git_username: "Chris"
  };

  println!("{}", comp_sci_student_greeting(&student));
}
