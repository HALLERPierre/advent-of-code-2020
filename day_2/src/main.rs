fn main() {
    println!("{}",include_str!("../input")
        .lines()
        .map(|p| p.split(' ').collect::<Vec<&str>>())
        .filter(|p| {
            let char = p[1].chars().nth(0).unwrap();
            let count = p[2].matches(char).count();
            let bounds: Vec<usize> = p[0].split('-').map(|p| p.parse::<usize>().unwrap()).collect();
            count >= bounds[0] && count <= bounds[1]
        })
        .count()
    );

    println!("{}",include_str!("../input")
        .lines()
        .map(|p| p.split(' ').collect::<Vec<&str>>())
        .filter(|p| {
            let char = p[1].chars().nth(0).unwrap();
            let password = p[2].chars().collect::<Vec<char>>();
            let positions: Vec<usize> = p[0].split('-').map(|p| p.parse::<usize>().unwrap()).collect();
            let is_pos1_correct = password[positions[0] - 1] == char;
            let is_pos2_correct = password[positions[1] - 1] == char;
            (is_pos1_correct || is_pos2_correct) && !(is_pos1_correct && is_pos2_correct)
        })
        .count()
    );
}
