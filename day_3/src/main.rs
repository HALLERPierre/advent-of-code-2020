fn main() {
    let map: Vec<Vec<char>> = include_str!("../input")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    println!(
        "{}",
        (0..map.len() - 1)
            .map(|i| ((i + 1) * 3, i + 1))
            .filter(|(x, y)| map[*y][*x % map[0].len()] == '#')
            .count()
    );

    println!(
        "{}",
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|(right, down)| (0..)
                .map(|i| ((i + 1) * right, (i + 1) * down))
                .take_while(|(_, y)| y < &map.len())
                .filter(|(x, y)| map[*y][*x % map[*y].len()] == '#')
                .count())
            .product::<usize>()
    );
}
