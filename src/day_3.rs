mod read_input;

fn main() {
    let mut map : Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_input::read_lines("./inputs/input_3") {
        for line in lines {
            if let Ok(map_line) = line {
                map.push(map_line.chars().collect::<Vec<char>>());
            }
        }
    }
    let r_1_d_1 = get_number_of_trees_encountered((*map).to_vec(), 1, 1);
    let r_3_d_1 = get_number_of_trees_encountered((*map).to_vec(), 1, 3);
    let r_5_d_1 = get_number_of_trees_encountered((*map).to_vec(), 1, 5);
    let r_7_d_1 = get_number_of_trees_encountered((*map).to_vec(), 1, 7);
    let r_1_d_2 = get_number_of_trees_encountered((*map).to_vec(), 2, 1);
    let part_2_result = r_1_d_1 as i128 * r_3_d_1 as i128 * r_5_d_1  as i128 * r_7_d_1  as i128 * r_1_d_2 as i128;
    println!("Part 1: {}", r_3_d_1);
    println!("Part 2: {}", part_2_result);
}

fn get_number_of_trees_encountered(map: Vec<Vec<char>>, step_x: usize, step_y: usize) -> i32 {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    while x < map.len() {
        let line : &Vec<char> = &map[x];
        if line[y % line.len()] == '#' {
            trees = trees + 1;
        }
        y = y + step_y;
        x = x + step_x;

    }
    return trees;
}
