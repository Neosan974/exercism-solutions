pub fn recite(start_bottles: u32, mut take_down: u32) -> String {
    let numbers_as_strings = [
        "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    let titlecase_numbers_as_strings = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    let mut result = String::from("");
    let mut index = start_bottles;

    while take_down != 0 {
        let head_number = titlecase_numbers_as_strings[index as usize];
        let head_s = if index >= 2 { "s" } else { "" };
        result.push_str(
            format!("{head_number} green bottle{head_s} hanging on the wall,\n")
                .repeat(2)
                .as_str(),
        );

        let final_number = numbers_as_strings[index as usize - 1];
        let final_s = if index > 2 || index == 1 { "s" } else { "" };
        result.push_str(format!(
            "And if one green bottle should accidentally fall,\nThere'll be {final_number} green bottle{final_s} hanging on the wall."
        ).as_str());

        take_down -= 1;
        index -= 1;

        if take_down > 0 {
            result.push_str("\n\n");
        }
    }

    result
}
