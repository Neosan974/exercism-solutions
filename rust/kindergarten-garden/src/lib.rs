fn get_student_index(student: &str) -> usize {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    students.iter().position(|&s| s == student).unwrap()
}

fn get_plant_letters_of_student_by_index(rows: Vec<&str>, student_index: usize) -> Vec<&str> {
    let mut res: Vec<&str> = vec![];
    let index = student_index * 2;
    let first_row: Vec<&str> = rows[0].split("").filter(|&e| e != "").into_iter().collect();
    res.push(first_row[index]);
    res.push(first_row[index + 1]);
    let second_row: Vec<&str> = rows[1].split("").filter(|&e| e != "").into_iter().collect();
    res.push(second_row[index]);
    res.push(second_row[index + 1]);
    res
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = get_student_index(student);
    let rows: Vec<&str> = diagram.split("\n").into_iter().collect();
    let plant_letters_of_student = get_plant_letters_of_student_by_index(rows, student_index);

    plant_letters_of_student
        .iter()
        .map(|&code| match code {
            "G" => "grass",
            "C" => "clover",
            "R" => "radishes",
            "V" => "violets",
            _ => ".",
        })
        .collect()
}
