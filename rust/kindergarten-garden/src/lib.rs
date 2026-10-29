fn get_student_index(student: &str) -> usize {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    students.element_offset(&student).unwrap()
}

fn get_plant_letters_of_student_by_index(rows: Vec<String>, student_index: usize) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let index = (student_index * 2);
    let first_row = rows[0].chars();
    let a = first_row[0];
    res.push();
    res.push(rows[0][index + 1]);
    res.push(rows[1][index]);
    res.push(rows[1][index + 1]);
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = get_student_index(student);
    let rows = diagram.split("\n");

    todo!("based on the {diagram}, determine the plants the {student} is responsible for");
}
