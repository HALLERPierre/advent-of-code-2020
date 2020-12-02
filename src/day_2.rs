mod read_input;

fn main() {
    let mut i = 0;
    let mut mins: Vec<i32> = Vec::new();
    let mut maxs: Vec<i32> = Vec::new();
    let mut chs: Vec<char> = Vec::new();
    let mut passwords: Vec<String> = Vec::new();
    if let Ok(lines) = read_input::read_lines("./inputs/input_2") {
        for line in lines {
            if let Ok(data) = line {
                let v: Vec<&str> = data.split(' ').collect();

                for (index, part) in v.iter().enumerate() {
                    if index == 0 {
                        let minmax: Vec<&str> = part.split('-').collect();
                        mins.push(minmax[0].parse::<i32>().unwrap());
                        maxs.push(minmax[1].parse::<i32>().unwrap());
                    }
                    if index == 1 {
                        let character = part.chars().nth(0).unwrap();
                        chs.push(character)
                    }
                    if index == 2 {
                        passwords.push(part.to_string())
                    }
                    i = i + 1;
                }
            }
        }
    }
    let mut n_passwords_step1 = 0;
    for (index, password) in passwords.iter().enumerate() {
        let min = mins[index];
        let max = maxs[index];
        let target_char = chs[index];
        let mut count_char = 0;
        for c in password.chars() {
            if c == target_char {
                count_char = count_char + 1;
            }
        }
        if count_char >= min && count_char <= max {
            n_passwords_step1 = n_passwords_step1 + 1;
        }
    }
    println!("Part One: {}", n_passwords_step1);

    let mut n_passwords_step2 = 0;
    for (index, password) in passwords.iter().enumerate() {
        let pos1 = mins[index] - 1;
        let pos2 = maxs[index] - 1;
        let target_char = chs[index];
        let password_chars = password.chars().collect::<Vec<char>>();

        let is_pos1_correct = password_chars[pos1 as usize] == target_char;
        let is_pos2_correct = password_chars[pos2 as usize] == target_char;

        if (is_pos1_correct || is_pos2_correct) && !(is_pos1_correct && is_pos2_correct) {
            n_passwords_step2 = n_passwords_step2 + 1;
        }

    }

    println!("Part Two: {}", n_passwords_step2);
}
