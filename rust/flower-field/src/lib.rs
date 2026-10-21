const FLOWER: u8 = "*".as_bytes()[0];

fn get_char_at(garden: &[&str], x: usize, y: usize) -> u8 {
    garden[y].as_bytes()[x]
}

fn get_nb_flowers_around(garden: &[&str], x: usize, y: usize) -> u8 {
    let width = garden[y].len();
    let height = garden.len();
    let min_y = if y == 0 { 0 } else { y - 1 };
    let max_y = (if y == height { height - 1 } else { y + 1 }).min(height - 1);
    let min_x = if x == 0 { 0 } else { x - 1 };
    let max_x = (if x == width { width - 1 } else { x + 1 }).min(width - 1);
    let mut count = 0;
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            if i == y && j == x {
                continue;
            }
            let char = get_char_at(garden, j, i);
            count += if char == FLOWER { 1 } else { 0 };
        }
    }
    count
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut annotated: Vec<String> = vec![];
    if garden.len() == 0 {
        return annotated;
    }
    for y in 0..garden.len() {
        annotated.push("".to_string());
        let row = garden[y].as_bytes();
        for x in 0..row.len() {
            let col = row[x];
            if col == FLOWER {
                annotated[y] += "*";
                continue;
            }
            let nb_flowers_around = get_nb_flowers_around(garden, x, y);
            annotated[y] += format!(
                "{}",
                if nb_flowers_around == 0 {
                    ' '.to_string()
                } else {
                    nb_flowers_around.to_string()
                }
            )
            .as_str();
        }
    }
    annotated
}
